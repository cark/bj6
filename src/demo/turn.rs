use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, RepeatCount, RepeatStrategy, Tween, lens::TransformPositionLens};

use crate::{
    data::game_config::GameConfig,
    demo::{
        GameplayState,
        actor::{ACTOR_Z, ActorEntities},
        follow::{self, FollowerLens, Follows},
        level::{LevelAssets, ResetBoardEvent},
        puff::SpawnHitParticlesEvent,
        tile::tile_coord_to_world_coord,
    },
    model::{
        actor::ActorId,
        game::Game,
        runner::{Cmd, Dest, Runner},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<TurnState>();
    app.init_resource::<DelayedHitParticleSpawns>();

    app.add_systems(OnEnter(GameplayState::Turn), enter);
    app.add_systems(OnExit(GameplayState::Turn), exit);
    app.add_systems(OnEnter(TurnState::NeedCommand), need_command);
    app.add_systems(OnExit(TurnState::PlayCommand), exit_play_command);
    app.add_systems(OnEnter(TurnState::EndTurn), enter_end_turn);
    app.add_systems(
        Update,
        tick_play_command_timer.run_if(in_state(TurnState::PlayCommand)),
    );
    app.add_systems(Update, spawn_delayed_particles);

    app.add_observer(on_spawn_activation);
    app.add_observer(on_despawn_activation);
    app.add_observer(on_move_actor);
    app.add_observer(on_fail_move_actor);
    app.add_observer(on_hit);

    app.add_observer(on_start_timer);
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
    NeedCommand,
    PlayCommand,
    EndTurn,
}

#[derive(Event, Debug)]
struct SpawnActivation(ActorId);

#[derive(Event, Debug)]
struct DespawnActivation(ActorId);

#[derive(Resource, Clone)]
struct PlayCommandTimer(Timer);

#[derive(Event, Debug)]
struct StartTimerEvent(Duration);

#[derive(Event, Debug)]
struct MoveActorEvent(ActorId, IVec2);

#[derive(Event, Debug)]
struct FailMoveActorEvent(ActorId, IVec2);

#[derive(Event, Debug)]
struct HitEvent(ActorId, IVec2);

#[derive(Resource, Debug, Default)]
struct DelayedHitParticleSpawns(Vec<(Timer, IVec2)>);

fn enter(mut commands: Commands, game: Res<Game>, mut set_state: ResMut<NextState<TurnState>>) {
    warn!("start running!");
    let runner_game = game.clone();
    // warn!("board before running: {:#?}", runner_game.board());
    let mut runner = Runner::new(runner_game);
    let (new_game, mut result) = runner.run();
    // warn!("board after running: {:#?}", new_game.board());
    warn!("done running!\n{result:#?}");
    result.reverse();
    commands.insert_resource(Cmds(result, new_game));
    set_state.set(TurnState::NeedCommand);
}

fn exit(mut command: Commands, mut set_state: ResMut<NextState<TurnState>>) {
    command.remove_resource::<Cmds>();
    set_state.set(TurnState::WorkaroundBugs);
}

fn need_command(
    mut commands: Commands,
    mut cmds: ResMut<Cmds>,
    mut set_state: ResMut<NextState<TurnState>>,
    // mut game: ResMut<Game>,
    config: Res<GameConfig>,
) {
    if let Some(cmd) = cmds.pop() {
        set_state.set(TurnState::PlayCommand);
        match cmd {
            Cmd::Activate(actor_id) => {
                warn!("activation");
                commands.trigger(SpawnActivation(actor_id));
                commands.trigger(StartTimerEvent(Duration::from_secs_f32(
                    config.turn.activation_duration,
                )));
            }
            Cmd::Deactivate(actor_id) => {
                warn!("deactivation");
                commands.trigger(DespawnActivation(actor_id));
                commands.trigger(StartTimerEvent(Duration::from_secs_f32(
                    config.turn.deactivation_duration,
                )));
            }
            Cmd::MoveTo(Ok(dest)) => {
                warn!("move");
                commands.trigger(MoveActorEvent(dest.from_actor_id, dest.to_coord));
                commands.trigger(StartTimerEvent(Duration::from_secs_f32(
                    config.turn.move_duration,
                )));
            }
            Cmd::MoveTo(Err(dest)) => {
                warn!("fail move");
                // warn!("{dest:?}");
                commands.trigger(FailMoveActorEvent(dest.from_actor_id, dest.to_coord));
                commands.trigger(StartTimerEvent(Duration::from_secs_f32(
                    config.turn.move_duration,
                )));
            }
            Cmd::TryPush(dest) => todo!(),
            Cmd::Turn(actor_id, rel_dir) => todo!(),
            Cmd::CompletePush(dest) => todo!(),
            Cmd::CancelPush(dest) => todo!(),
            Cmd::Done => {
                warn!("done");
                set_state.set(TurnState::EndTurn)
            }
            Cmd::Hit(Dest {
                to_coord,
                from_actor_id,
            }) => {
                warn!("hit");
                commands.trigger(HitEvent(from_actor_id, to_coord));
                commands.trigger(StartTimerEvent(Duration::from_secs_f32(
                    config.turn.hit_duration,
                )));
            }
        }
    } else {
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
        let tween = Tween::new(
            EaseFunction::CircularOut,
            Duration::from_secs_f32(0.5),
            FollowerLens {
                start: vec3(0.0, config.ui.activation_icon_offset * 0.8, 1.),
                end: vec3(0.0, config.ui.activation_icon_offset * 1.2, 1.),
            },
        )
        .with_repeat_count(RepeatCount::Infinite)
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat);
        commands.spawn((
            ActivationSprite,
            Sprite {
                image: assets.activation.clone(),
                custom_size: Some(Vec2::splat(config.ui.activation_icon_size)),
                ..default()
            },
            Transform::from_translation((Vec3::Y * config.ui.activation_icon_offset).with_z(4.)),
            Animator::new(tween),
            Follows {
                target: actor_entity,
                offset: vec3(0.0, config.ui.activation_icon_offset, 1.0),
            },
        ));
        // commands.entity(actor_entity).with_child((
        //     ActivationSprite,
        //     Sprite {
        //         image: assets.activation.clone(),
        //         custom_size: Some(Vec2::splat(config.ui.activation_icon_size)),
        //         ..default()
        //     },
        //     Transform::from_translation((Vec3::Y * config.ui.activation_icon_offset).with_z(4.)),
        //     Animator::new(tween),
        // ));
    }
}

fn on_despawn_activation(
    trigger: Trigger<DespawnActivation>,
    mut commands: Commands,
    actor_entities: Res<ActorEntities>,
    q_activation_sprites: Query<(Entity, &Follows), With<ActivationSprite>>,
    // q_actor_children: Query<&Children, With<ActorId>>,
    mut game: ResMut<Game>,
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
}

fn on_start_timer(
    trigger: Trigger<StartTimerEvent>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut commands: Commands,
) {
    let duration = trigger.event().0;
    commands.insert_resource(PlayCommandTimer(Timer::new(duration, TimerMode::Once)));
    next_state.set(TurnState::PlayCommand);
}

fn tick_play_command_timer(
    mut play_command_timer: ResMut<PlayCommandTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    let timer = &mut play_command_timer.0;
    timer.tick(time.delta());
    if timer.just_finished() {
        next_state.set(TurnState::NeedCommand);
    }
}

fn exit_play_command(mut commands: Commands) {
    commands.remove_resource::<PlayCommandTimer>();
}

fn enter_end_turn(
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut next_gameplay_state: ResMut<NextState<GameplayState>>,
    cmds: Res<Cmds>,
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    next_turn_state.set(TurnState::WorkaroundBugs);
    next_gameplay_state.set(GameplayState::Placement);
    *game = cmds.1.clone();
    commands.trigger(ResetBoardEvent);
}

fn on_move_actor(
    trigger: Trigger<MoveActorEvent>,
    mut commands: Commands,
    actor_entities: Res<ActorEntities>,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
) {
    let ev = trigger.event();
    let actor_id = ev.0;
    if let Some(actor_view) = game.actor_view(&actor_id) {
        if let Some(actor_entity) = actor_entities.get(&actor_id) {
            let start = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size)
                .extend(ACTOR_Z);
            let end = tile_coord_to_world_coord(ev.1, config.checker.tile_size).extend(ACTOR_Z);
            game.update_actor(&ev.0, |actor| actor.coord = ev.1);

            warn!(config.turn.move_duration);
            let tween = Tween::new(
                EaseFunction::CircularInOut,
                Duration::from_secs_f32(config.turn.move_duration),
                TransformPositionLens { start, end },
            )
            .with_repeat_count(1);
            commands.entity(actor_entity).insert(Animator::new(tween));
        }
    }
}

fn on_fail_move_actor(
    trigger: Trigger<FailMoveActorEvent>,
    mut commands: Commands,
    actor_entities: Res<ActorEntities>,
    config: Res<GameConfig>,
    game: Res<Game>,
) {
    let ev = trigger.event();
    // warn!("on_fail_move_actor_event {ev:?}");
    let actor_id = ev.0;
    if let Some(actor_view) = game.actor_view(&actor_id) {
        if let Some(actor_entity) = actor_entities.get(&actor_id) {
            // warn!("{:?} -> {:?}", actor_view.actor.coord, ev.1);
            let start = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size)
                .extend(ACTOR_Z);
            let end = tile_coord_to_world_coord(ev.1, config.checker.tile_size).extend(ACTOR_Z);
            // warn!("{start:?} -> {end:?}");
            // warn!(config.turn.move_duration);
            let tween = Tween::new(
                EaseFunction::CircularIn,
                Duration::from_secs_f32(config.turn.move_duration / 2.0),
                TransformPositionLens {
                    start,
                    end: start.lerp(end, 0.2),
                },
            )
            .with_repeat_count(1)
            .then(Tween::new(
                EaseFunction::CircularOut,
                Duration::from_secs_f32(config.turn.move_duration / 2.0),
                TransformPositionLens {
                    start: start.lerp(end, 0.2),
                    end: start,
                },
            ));
            commands.entity(actor_entity).insert(Animator::new(tween));
        }
    }
}

fn on_hit(
    trigger: Trigger<HitEvent>,
    mut commands: Commands,
    actor_entities: Res<ActorEntities>,
    config: Res<GameConfig>,
    game: Res<Game>,
    mut delayed_hit_particle_spawns: ResMut<DelayedHitParticleSpawns>,
) {
    let ev = trigger.event();
    let actor_id = ev.0;
    let target_coord = ev.1;
    delayed_hit_particle_spawns.0.push((
        Timer::new(
            Duration::from_secs_f32(config.turn.hit_duration * 2.0 / 3.0),
            TimerMode::Once,
        ),
        target_coord,
    ));
    //commands.trigger(SpawnHitParticlesEvent(target_coord));
    if let Some(actor_view) = game.actor_view(&actor_id) {
        if let Some(actor_entity) = actor_entities.get(&actor_id) {
            let start = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size)
                .extend(ACTOR_Z);
            let end =
                tile_coord_to_world_coord(target_coord, config.checker.tile_size).extend(ACTOR_Z);

            let tween = Tween::new(
                EaseFunction::CircularOut,
                Duration::from_secs_f32(config.turn.hit_duration / 3.0),
                TransformPositionLens {
                    start,
                    end: start.lerp(end, -0.2),
                },
            )
            .with_repeat_count(1)
            .then(Tween::new(
                EaseFunction::ExponentialIn,
                Duration::from_secs_f32(config.turn.hit_duration / 3.0),
                TransformPositionLens {
                    start: start.lerp(end, -0.2),
                    end: start.lerp(end, 0.6),
                },
            ))
            .then(Tween::new(
                EaseFunction::QuarticOut,
                Duration::from_secs_f32(config.turn.hit_duration / 3.0),
                TransformPositionLens {
                    start: start.lerp(end, 0.6),
                    end: start,
                },
            ));
            commands.entity(actor_entity).insert(Animator::new(tween));
        }
    }
}

fn spawn_delayed_particles(
    mut commands: Commands,
    mut delayed_hit_particle_spawns: ResMut<DelayedHitParticleSpawns>,
    time: Res<Time>,
) {
    delayed_hit_particle_spawns.0.retain_mut(|(timer, coord)| {
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.trigger(SpawnHitParticlesEvent(*coord));
        }
        !timer.just_finished()
    });
}
