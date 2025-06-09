use std::time::Duration;

use bevy::prelude::*;
use bevy_tween::{
    bevy_time_runner::{TimeRunner, TimeRunnerEnded},
    combinator::{event, forward, sequence, tween},
    interpolate::translation,
    prelude::{AnimationBuilderExt, EaseKind, Repeat, RepeatStyle, TweenEvent},
    tween::{AnimationTarget, TargetComponent},
    tween_event::TweenEventPlugin,
};

use crate::{
    audio::sound_effect,
    data::game_config::GameConfig,
    demo::{
        GameplayState, Paused,
        actor::{ACTOR_Z, ActorEntities, ActorRotationFixupEvent},
        camera::CameraToActorEvent,
        follow::{Follows, follow_offset},
        level::{LevelAssets, ResetBoardEvent},
        puff::SpawnHitParticlesEvent,
        tile::tile_coord_to_world_coord,
        ui::{game_over::GameOverEvent, top_bar::UpdateTopBarEvent},
    },
    model::{
        actor::ActorId,
        direction::RelDir,
        game::Game,
        runner::{Cmd, Dest, Runner},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<TurnState>();
    app.add_plugins(TweenEventPlugin::<AnimEvent>::default());

    app.add_systems(OnEnter(GameplayState::Turn), enter);
    app.add_systems(OnExit(GameplayState::Turn), exit);

    app.add_systems(OnEnter(Paused(true)), enter_pause);
    app.add_systems(OnExit(Paused(true)), exit_pause);

    app.add_systems(OnEnter(TurnState::EndTurn), enter_end_turn);
    app.add_systems(Update, on_time_runner_ended);

    app.add_observer(on_spawn_activation);
    app.add_observer(on_despawn_activation);
    app.add_observer(on_move_actor);
    app.add_observer(on_fail_move_actor);
    app.add_observer(on_hit);
    app.add_observer(on_need_command);
    app.add_observer(on_try_push);
    app.add_observer(on_complete_push);
    app.add_observer(on_cancel_push);
    app.add_observer(on_prize);
    app.add_observer(on_turn);

    app.add_observer(on_anim_event);
}

#[derive(Clone, Debug, Default)]
enum AnimEvent {
    Hit(IVec2),
    AnimDone,
    PlaySfx(Handle<AudioSource>),
    #[default]
    None,
}

#[derive(Resource, Debug, Default)]
struct Cmds(Vec<Cmd>, Game);

impl Cmds {
    fn pop(&mut self) -> Option<Cmd> {
        self.0.pop()
    }
}

#[derive(SubStates, Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[source(GameplayState = GameplayState::Turn)]
#[states(scoped_entities)]
pub enum TurnState {
    #[default]
    WorkaroundBugs,
    EndTurn,
}

#[derive(Event, Debug)]
struct SpawnActivation(ActorId);

#[derive(Event, Debug)]
struct DespawnActivation(ActorId);

#[derive(Event, Debug)]
struct MoveActorEvent(ActorId, IVec2);

#[derive(Event, Debug)]
struct FailMoveActorEvent(ActorId, IVec2);

#[derive(Event, Debug)]
struct HitEvent(ActorId, IVec2);

#[derive(Event, Debug)]
struct NeedCommandEvent;

#[derive(Event, Debug)]
struct TryPushEvent(ActorId, IVec2);

#[derive(Event, Debug)]
struct CancelPushEvent(ActorId, IVec2);

#[derive(Event, Debug)]
struct CompletePushEvent(ActorId, IVec2);

#[derive(Event, Debug)]
struct TurnEvent(ActorId, RelDir);

#[derive(Event, Debug)]
#[allow(dead_code)]
struct PrizeEvent(ActorId, u64);

fn enter(mut commands: Commands, game: Res<Game>) {
    warn!("start running!");
    let runner_game = game.clone();
    // warn!("board before running: {:#?}", runner_game.board());
    let mut runner = Runner::new(runner_game);
    let (new_game, mut result) = runner.run();
    // warn!("board after running: {:#?}", new_game.board());
    warn!("done running!\n{result:#?}");
    result.reverse();
    commands.insert_resource(Cmds(result, new_game));
    commands.trigger(NeedCommandEvent);
}

fn exit(mut command: Commands, mut set_state: ResMut<NextState<TurnState>>) {
    command.remove_resource::<Cmds>();
    set_state.set(TurnState::WorkaroundBugs);
}

fn enter_pause(mut q_time_runner: Query<&mut TimeRunner>) {
    for mut time_runner in &mut q_time_runner {
        time_runner.set_paused(true);
    }
}

fn exit_pause(mut q_time_runner: Query<&mut TimeRunner>) {
    for mut time_runner in &mut q_time_runner {
        time_runner.set_paused(false);
    }
}

fn enter_end_turn(
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut next_gameplay_state: ResMut<NextState<GameplayState>>,
    cmds: Res<Cmds>,
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    *game = cmds.1.clone();

    let game_over = game.is_round_end() && !game.can_go_next_round();
    if game_over {
        commands.trigger(GameOverEvent);
    } else {
        next_turn_state.set(TurnState::WorkaroundBugs);
        next_gameplay_state.set(GameplayState::Placement);
        commands.trigger(ResetBoardEvent);
        if game.is_round_end() {
            game.next_round();
        }
    }
}

fn on_need_command(
    _trigger: Trigger<NeedCommandEvent>,
    mut commands: Commands,
    mut cmds: ResMut<Cmds>,
    mut set_state: ResMut<NextState<TurnState>>,
) {
    if let Some(cmd) = cmds.pop() {
        match cmd {
            Cmd::Activate(actor_id) => {
                warn!("activation");
                commands.trigger(SpawnActivation(actor_id));
            }
            Cmd::Deactivate(actor_id) => {
                warn!("deactivation");
                commands.trigger(DespawnActivation(actor_id));
            }
            Cmd::MoveTo(Ok(dest)) => {
                warn!("move");
                commands.trigger(MoveActorEvent(dest.from_actor_id, dest.to_coord));
            }
            Cmd::MoveTo(Err(dest)) => {
                warn!("fail move");
                commands.trigger(FailMoveActorEvent(dest.from_actor_id, dest.to_coord));
            }
            Cmd::TryPush(dest) => {
                warn!("try push");
                commands.trigger(TryPushEvent(dest.from_actor_id, dest.to_coord));
            }
            Cmd::CompletePush(dest) => {
                warn!("complete push");
                commands.trigger(CompletePushEvent(dest.from_actor_id, dest.to_coord));
            }
            Cmd::Turn(actor_id, rel_dir) => {
                warn!("turn");
                commands.trigger(TurnEvent(actor_id, rel_dir));
            }
            Cmd::CancelPush(dest) => {
                warn!("cancel push");
                commands.trigger(CancelPushEvent(dest.from_actor_id, dest.to_coord));
            }
            Cmd::Done => {
                warn!("Cmd::Done");
                set_state.set(TurnState::EndTurn);
            }
            Cmd::Hit(Dest {
                to_coord,
                from_actor_id,
            }) => {
                warn!("Cmd::Hit");
                commands.trigger(HitEvent(from_actor_id, to_coord));
            }
            Cmd::Prize(actor_id, gold) => {
                warn!("Cmd::Prize {gold}");
                commands.trigger(PrizeEvent(actor_id, gold));
            }
        }
    } else {
        warn!("no more commands");
        set_state.set(TurnState::EndTurn);
    }
}

#[derive(Component, Debug, Clone, Copy)]
struct ActivationSprite;

fn on_spawn_activation(
    trigger: Trigger<SpawnActivation>,
    mut commands: Commands,
    actor_entities: Res<ActorEntities>,
    assets: Res<LevelAssets>,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
) {
    let ev = trigger.event();
    let actor_id = ev.0;
    game.update_actor(&actor_id, |actor| {
        actor.activated = true;
        actor.activations_left -= 1;
    });

    if let Some(actor_entity) = actor_entities.get(&actor_id) {
        let target = TargetComponent::marker();
        commands
            .spawn((
                ActivationSprite,
                Sprite {
                    image: assets.activation.clone(),
                    custom_size: Some(Vec2::splat(config.ui.activation_icon_size)),
                    ..default()
                },
                Transform::from_translation(
                    (Vec3::Y * config.ui.activation_icon_offset).with_z(4.),
                ),
                Follows {
                    target: actor_entity,
                    offset: vec3(0.0, config.ui.activation_icon_offset, 1.0),
                },
                AnimationTarget,
            ))
            .with_children(|cmd| {
                cmd.spawn(())
                    .animation()
                    .repeat(Repeat::Infinitely)
                    .repeat_style(RepeatStyle::PingPong)
                    .insert(tween(
                        Duration::from_secs_f32(0.25),
                        EaseKind::CircularOut,
                        target.with(follow_offset(
                            vec3(0.0, config.ui.activation_icon_offset * 0.8, 1.),
                            vec3(0.0, config.ui.activation_icon_offset * 1.2, 1.),
                        )),
                    ));
            });
    }
    done_in(commands.reborrow(), config.turn.activation_duration);
}

fn on_despawn_activation(
    trigger: Trigger<DespawnActivation>,
    mut commands: Commands,
    actor_entities: Res<ActorEntities>,
    q_activation_sprites: Query<(Entity, &Follows), With<ActivationSprite>>,
    mut game: ResMut<Game>,
    config: Res<GameConfig>,
) {
    let actor_id = trigger.event().0;
    game.update_actor(&actor_id, |actor| {
        actor.activated = false;
    });
    if let Some(actor_entity) = actor_entities.get(&actor_id) {
        for (sprite_entity, follows) in &q_activation_sprites {
            if follows.target == actor_entity {
                commands.entity(sprite_entity).despawn();
            }
        }
    }
    done_in(commands.reborrow(), config.turn.deactivation_duration);
}

fn on_turn(
    trigger: Trigger<TurnEvent>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    config: Res<GameConfig>,
) {
    let actor_id = trigger.event().0;
    let rel_dir = trigger.event().1;
    game.update_actor(&actor_id, |actor| {
        actor.looks_to = actor.looks_to.apply_relative(rel_dir)
    });
    commands.trigger(ActorRotationFixupEvent);
    done_in(commands.reborrow(), config.turn.turn_duration);
}

fn on_move_actor(
    trigger: Trigger<MoveActorEvent>,
    mut commands: Commands,
    actor_entities: Res<ActorEntities>,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    assets: Res<LevelAssets>,
) {
    let ev = trigger.event();
    let actor_id = ev.0;
    if let Some(actor_view) = game.actor_view(&actor_id) {
        if let Some(actor_entity) = actor_entities.get(&actor_id) {
            let start = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size)
                .extend(ACTOR_Z);
            let end = tile_coord_to_world_coord(ev.1, config.checker.tile_size).extend(ACTOR_Z);
            game.update_actor(&ev.0, |actor| actor.coord = ev.1);

            commands.trigger(CameraToActorEvent(actor_id));

            let target = TargetComponent::marker();
            commands
                .entity(actor_entity)
                .insert(AnimationTarget)
                .with_children(|cmd| {
                    cmd.spawn(()).animation().insert(sequence((
                        tween(
                            Duration::from_secs_f32(config.turn.move_duration),
                            EaseKind::CircularInOut,
                            target.with(translation(start, end)),
                        ),
                        event(AnimEvent::AnimDone),
                    )));
                });
            anim_event_in(
                commands.reborrow(),
                config.turn.move_duration / 2.,
                AnimEvent::PlaySfx(assets.move_sfx.clone()),
            );
        }
    }
}

fn on_fail_move_actor(
    trigger: Trigger<FailMoveActorEvent>,
    mut commands: Commands,
    actor_entities: Res<ActorEntities>,
    config: Res<GameConfig>,
    game: Res<Game>,
    assets: Res<LevelAssets>,
) {
    let ev = trigger.event();
    let actor_id = ev.0;
    if let Some(actor_view) = game.actor_view(&actor_id) {
        if let Some(actor_entity) = actor_entities.get(&actor_id) {
            let start = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size)
                .extend(ACTOR_Z);
            let end = tile_coord_to_world_coord(ev.1, config.checker.tile_size).extend(ACTOR_Z);
            let target = TargetComponent::marker();
            commands
                .entity(actor_entity)
                .insert(AnimationTarget)
                .with_children(|cmd| {
                    cmd.spawn(()).animation().insert(sequence((
                        tween(
                            Duration::from_secs_f32(config.turn.move_duration / 2.),
                            EaseKind::CircularIn,
                            target.with(translation(start, start.lerp(end, 0.2))),
                        ),
                        tween(
                            Duration::from_secs_f32(config.turn.move_duration / 2.),
                            EaseKind::CircularOut,
                            target.with(translation(start.lerp(end, 0.2), start)),
                        ),
                        event(AnimEvent::AnimDone),
                    )));
                });
            anim_event_in(
                commands.reborrow(),
                config.turn.move_duration / 2.,
                AnimEvent::PlaySfx(assets.move_fail_sfx.clone()),
            );
        }
    }
}

fn on_hit(
    trigger: Trigger<HitEvent>,
    mut commands: Commands,
    actor_entities: Res<ActorEntities>,
    config: Res<GameConfig>,
    game: Res<Game>,
    assets: Res<LevelAssets>,
) {
    let ev = trigger.event();
    let actor_id = ev.0;
    let target_coord = ev.1;
    if let Some(actor_view) = game.actor_view(&actor_id) {
        if let Some(actor_entity) = actor_entities.get(&actor_id) {
            commands.trigger(CameraToActorEvent(actor_id));
            let start = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size)
                .extend(ACTOR_Z);
            let end =
                tile_coord_to_world_coord(target_coord, config.checker.tile_size).extend(ACTOR_Z);
            let target = TargetComponent::marker();
            commands
                .entity(actor_entity)
                .insert(AnimationTarget)
                .with_children(|cmd| {
                    cmd.spawn(()).animation().insert(sequence((
                        tween(
                            Duration::from_secs_f32(config.turn.hit_duration / 3.0),
                            EaseKind::CircularOut,
                            target.with(translation(start, start.lerp(end, -0.2))),
                        ),
                        tween(
                            Duration::from_secs_f32(config.turn.hit_duration / 3.0),
                            EaseKind::ExponentialIn,
                            target.with(translation(start.lerp(end, -0.2), start.lerp(end, 0.6))),
                        ),
                        event(AnimEvent::Hit(target_coord)),
                        tween(
                            Duration::from_secs_f32(config.turn.hit_duration / 3.0),
                            EaseKind::QuarticOut,
                            target.with(translation(start.lerp(end, 0.6), start)),
                        ),
                        event(AnimEvent::AnimDone),
                    )));
                });
            anim_event_in(
                commands.reborrow(),
                config.turn.hit_duration / 3.,
                AnimEvent::PlaySfx(assets.hit_sfx.clone()),
            );
        }
    }
}

fn on_try_push(
    trigger: Trigger<TryPushEvent>,
    mut commands: Commands,
    game: Res<Game>,
    config: Res<GameConfig>,
    actor_entities: Res<ActorEntities>,
    assets: Res<LevelAssets>,
) {
    let ev = trigger.event();
    let actor_id = ev.0;
    let target_coord = ev.1;
    if let Some(actor_view) = game.actor_view(&actor_id) {
        if let Some(actor_entity) = actor_entities.get(&actor_id) {
            commands.trigger(CameraToActorEvent(actor_id));
            let start = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size)
                .extend(ACTOR_Z);
            let end =
                tile_coord_to_world_coord(target_coord, config.checker.tile_size).extend(ACTOR_Z);
            let target = TargetComponent::marker();
            commands
                .entity(actor_entity)
                .insert(AnimationTarget)
                .with_children(|cmd| {
                    cmd.spawn(()).animation().insert(sequence((
                        tween(
                            Duration::from_secs_f32(config.turn.try_push_duration),
                            EaseKind::QuadraticOut,
                            target.with(translation(start, start.lerp(end, 0.5))),
                        ),
                        event(AnimEvent::AnimDone),
                    )));
                });
            anim_event_in(
                commands.reborrow(),
                0.0,
                AnimEvent::PlaySfx(assets.try_push_sfx.clone()),
            );
        }
    }
}

fn on_complete_push(
    trigger: Trigger<CompletePushEvent>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    config: Res<GameConfig>,
    actor_entities: Res<ActorEntities>,
    assets: Res<LevelAssets>,
) {
    // todo!();
    let ev = trigger.event();
    let actor_id = ev.0;
    let target_coord = ev.1;
    if let Some(actor_view) = game.actor_view(&actor_id) {
        if let Some(actor_entity) = actor_entities.get(&actor_id) {
            commands.trigger(CameraToActorEvent(actor_id));
            let start = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size)
                .extend(ACTOR_Z);
            let end =
                tile_coord_to_world_coord(target_coord, config.checker.tile_size).extend(ACTOR_Z);
            game.update_actor(&actor_id, |actor| actor.coord = target_coord);
            let target = TargetComponent::marker();
            commands
                .entity(actor_entity)
                .insert(AnimationTarget)
                .with_children(|cmd| {
                    cmd.spawn(()).animation().insert(sequence((
                        tween(
                            Duration::from_secs_f32(config.turn.complete_push_duration),
                            EaseKind::QuadraticIn,
                            target.with(translation(start.lerp(end, 0.5), end)),
                        ),
                        event(AnimEvent::AnimDone),
                    )));
                });
            anim_event_in(
                commands.reborrow(),
                config.turn.complete_push_duration * 0.25,
                AnimEvent::PlaySfx(assets.move_sfx.clone()),
            );
        }
    }
}

fn on_cancel_push(
    trigger: Trigger<CancelPushEvent>,
    mut commands: Commands,
    game: Res<Game>,
    config: Res<GameConfig>,
    actor_entities: Res<ActorEntities>,
    assets: Res<LevelAssets>,
) {
    // todo!();
    let ev = trigger.event();
    let actor_id = ev.0;
    let target_coord = ev.1;
    if let Some(actor_view) = game.actor_view(&actor_id) {
        if let Some(actor_entity) = actor_entities.get(&actor_id) {
            commands.trigger(CameraToActorEvent(actor_id));
            let start = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size)
                .extend(ACTOR_Z);
            let end =
                tile_coord_to_world_coord(target_coord, config.checker.tile_size).extend(ACTOR_Z);
            // game.update_actor(&actor_id, |actor| actor.coord = target_coord);
            let target = TargetComponent::marker();
            commands
                .entity(actor_entity)
                .insert(AnimationTarget)
                .with_children(|cmd| {
                    cmd.spawn(()).animation().insert(sequence((
                        tween(
                            Duration::from_secs_f32(config.turn.cancel_push_duration),
                            EaseKind::QuadraticIn,
                            target.with(translation(start.lerp(end, 0.5), start)),
                        ),
                        event(AnimEvent::AnimDone),
                    )));
                });
            anim_event_in(
                commands.reborrow(),
                0.0,
                AnimEvent::PlaySfx(assets.cancel_push_sfx.clone()),
            );
        }
    }
}

fn on_anim_event(
    trigger: Trigger<TweenEvent<AnimEvent>>,
    mut commands: Commands,
    // mut next_state: ResMut<NextState<TurnState>>,
) {
    match trigger.event().data {
        AnimEvent::Hit(coord) => {
            commands.trigger(SpawnHitParticlesEvent(coord));
        }
        AnimEvent::AnimDone => commands.trigger(NeedCommandEvent),
        AnimEvent::PlaySfx(ref handle) => {
            commands.spawn(sound_effect(handle.clone()));
        }
        AnimEvent::None => unreachable!(),
    }
}

fn on_time_runner_ended(mut reader: EventReader<TimeRunnerEnded>, mut commands: Commands) {
    for ev in reader.read() {
        let ended = ev.time_runner;
        if ev.is_completed() {
            commands.entity(ended).despawn();
        }
    }
}

fn on_prize(
    trigger: Trigger<PrizeEvent>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    assets: Res<LevelAssets>,
) {
    let ev = trigger.event();
    game.earn_prize_gold(ev.1);
    commands.trigger(UpdateTopBarEvent);
    anim_event_in(
        commands.reborrow(),
        0.0,
        AnimEvent::PlaySfx(assets.coin_sfx.clone()),
    );
    done_in(commands.reborrow(), 0.01);
}

fn done_in(mut commands: Commands, secs: f32) {
    commands.animation().insert(sequence((
        forward(Duration::from_secs_f32(secs)),
        event(AnimEvent::AnimDone),
    )));
}

fn anim_event_in(mut commands: Commands, secs: f32, anim_event: AnimEvent) {
    commands.animation().insert(sequence((
        forward(Duration::from_secs_f32(secs)),
        event(anim_event),
    )));
}
