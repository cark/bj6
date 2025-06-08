use bevy::prelude::*;

use crate::{
    AppSystems,
    data::game_config::GameConfig,
    demo::{
        Paused, actor::ActorRotationFixupEvent, puff::SpawDropParticlesEvent,
        ui::actions::SetActiveActionEvent,
    },
    model::{actor::ActorId, actor_type::ActorTypeId, game::Game},
};

use super::{
    GameplayState,
    actor::SpawnActorEvent,
    mouse::MouseWorldCoords,
    tile::{HoveredActor, HoveredTileCoord, tile_coord_to_world_coord},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_start_drag);
    app.add_observer(on_cancel_drag);
    app.add_observer(on_drop);

    app.add_systems(
        Update,
        (move_drag_image, check_clicks)
            .in_set(AppSystems::Update)
            .run_if(in_state(GameplayState::Drag).and(in_state(Paused(false)))),
    );
    app.add_systems(OnExit(GameplayState::Drag), exit);
    app.add_systems(
        Update,
        update_actions.run_if(in_state(GameplayState::Drag).or(in_state(GameplayState::Placement))),
    );
}

fn update_actions(
    mut commands: Commands,
    hovered_actor: Res<HoveredActor>,
    gameplay_state: Res<State<GameplayState>>,
    drag: Option<Res<Drag>>,
) {
    let dragging = gameplay_state.get() == &GameplayState::Drag;
    let (hover_actor, can_drag) = if let Some((_, actor_view)) = &**hovered_actor {
        (true, actor_view.actor_type.dragable)
    } else {
        (false, false)
    };
    let from_shop = if let Some(drag) = drag {
        drag.source.is_from_shop()
    } else {
        false
    };
    let (drag, drop, cancel) = match (dragging, hover_actor, can_drag, from_shop) {
        (false, true, true, _) => (true, false, false),
        (false, _, _, _) => (false, false, false),
        (true, true, true, false) => (false, true, true),
        (true, true, true, true) => (false, false, true),
        (true, true, false, _) => (false, false, true),
        (true, _, _, _) => (false, true, true),
    };
    commands.trigger(SetActiveActionEvent("lmb_drag".to_string(), drag));
    commands.trigger(SetActiveActionEvent("lmb_drop".to_string(), drop));
    commands.trigger(SetActiveActionEvent("rmb_cancel_drag".to_string(), cancel));
}

#[derive(Resource, Debug, Clone)]
struct Drag {
    actor_type_id: ActorTypeId,
    source: DragSource,
    can_drop: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum DragSource {
    Shop {
        shop_index: usize,
    },
    Board {
        dragged_entity: Entity,
        start_coord: IVec2,
    },
}

#[allow(dead_code)]
impl DragSource {
    pub fn is_from_board(&self) -> bool {
        matches!(self, DragSource::Board { .. })
    }

    pub fn is_from_shop(&self) -> bool {
        matches!(self, DragSource::Shop { .. })
    }
}

#[derive(Event, Debug, Clone)]
pub struct StartDragEvent {
    pub source: DragSource,
    pub actor_type_id: ActorTypeId,
}

#[derive(Event, Debug, Clone)]
pub struct CancelDragEvent;

#[derive(Event, Debug, Clone)]
pub struct ApplyDragEvent;

fn exit(mut commands: Commands) {
    commands.remove_resource::<Drag>();
}

#[derive(Component, Debug, Clone)]
struct DragImage;

fn on_start_drag(
    trigger: Trigger<StartDragEvent>,
    mut next_state: ResMut<NextState<GameplayState>>,
    game: Res<Game>,
    mut commands: Commands,
    config: Res<GameConfig>,
    hovered_tile_coord: Res<HoveredTileCoord>,
    mut q_sprite: Query<(&mut Sprite, &ActorId)>,
) {
    // warn!("on_start_drag");
    let ev = trigger.event();
    if let Some(actor_type) = game.actor_types().get(&ev.actor_type_id) {
        commands.insert_resource(Drag {
            actor_type_id: ev.actor_type_id.clone(),
            source: ev.source,
            can_drop: false,
        });
        next_state.set(GameplayState::Drag);

        let sprite = Sprite {
            image: actor_type.sprite_handle.clone().unwrap(),
            color: Color::linear_rgba(1.0, 1.0, 1.0, 0.5),
            custom_size: Some(Vec2::splat(config.checker.tile_size * config.drag.scale)),
            ..default()
        };

        let entity = commands
            .spawn((StateScoped(GameplayState::Drag), DragImage, sprite))
            .id();

        if let Some(coord) = **hovered_tile_coord {
            let tile_size = config.checker.tile_size;
            commands.entity(entity).insert(Transform::from_translation(
                tile_coord_to_world_coord(coord, tile_size).extend(3.0),
            ));
        }
        if let DragSource::Board { dragged_entity, .. } = ev.source {
            if let Ok((mut sprite, _actor_id)) = q_sprite.get_mut(dragged_entity) {
                sprite.color = Color::linear_rgba(1.0, 1.0, 1.0, config.drag.alpha);
            }
        }
    }
}

fn move_drag_image(
    mouse_world_coords: Res<MouseWorldCoords>,
    image_tr: Single<(&mut Transform, &mut Sprite), With<DragImage>>,
    hovered_actor: Res<HoveredActor>,
    hovered_tile_coord: Res<HoveredTileCoord>,
    config: Res<GameConfig>,
    mut drag: ResMut<Drag>,
    game: Res<Game>,
) {
    drag.can_drop = false;
    let tile_size = config.checker.tile_size;
    if let Some(mouse_world_coords) = mouse_world_coords.0 {
        let (mut tr, mut sprite) = image_tr.into_inner();
        if hovered_actor.as_ref().is_some() {
            let (_hovered_entity, actor_view) = hovered_actor.as_ref().as_ref().unwrap();
            let can_drop = match drag.source {
                DragSource::Board { .. } => actor_view.actor_id != game.board().start_actor_id(),
                DragSource::Shop { .. } => false,
            };
            if can_drop {
                tr.translation =
                    tile_coord_to_world_coord(actor_view.actor.coord, tile_size).extend(3.0);
                sprite.color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
                drag.can_drop = true;
            } else {
                tr.translation = mouse_world_coords.extend(3.0);
                sprite.color = Color::linear_rgba(1.0, 1.0, 1.0, config.drag.alpha);
            }
        } else if let Some(coord) = &**hovered_tile_coord {
            tr.translation = tile_coord_to_world_coord(*coord, tile_size).extend(3.0);
            sprite.color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
            drag.can_drop = true;
        }
    }
}

fn check_clicks(buttons: Res<ButtonInput<MouseButton>>, mut commands: Commands) {
    if buttons.just_pressed(MouseButton::Right) {
        commands.trigger(CancelDragEvent);
    }
    if buttons.just_pressed(MouseButton::Left) {
        commands.trigger(ApplyDragEvent);
    }
}

fn on_cancel_drag(
    _trigger: Trigger<CancelDragEvent>,
    mut next_state: ResMut<NextState<GameplayState>>,
    mut game: ResMut<Game>,
    drag: Res<Drag>,
    mut q_actor_sprite: Query<&mut Sprite, With<ActorId>>,
) {
    // warn!("on_cancel_drag");
    match drag.source {
        DragSource::Shop { shop_index } => {
            game.return_item(&drag.actor_type_id, shop_index);
            next_state.set(GameplayState::Placement);
        }
        DragSource::Board { .. } => {
            for mut sprite in q_actor_sprite.iter_mut() {
                sprite.color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
            }
            next_state.set(GameplayState::Placement);
        }
    }
}

fn on_drop(
    _trigger: Trigger<ApplyDragEvent>,
    mut next_state: ResMut<NextState<GameplayState>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    drag: Res<Drag>,
    hovered_actor: Res<HoveredActor>,
    hovered_tile_coord: Res<HoveredTileCoord>,
    mut q_tr: Query<&mut Transform, With<ActorId>>,
    mut q_actor_sprite: Query<&mut Sprite, With<ActorId>>,
    config: Res<GameConfig>,
) {
    // warn!("on_drop");
    match drag.source {
        DragSource::Shop { .. } => {
            if let Some(coord) = &**hovered_tile_coord {
                if drag.can_drop {
                    commands.trigger(SpawnActorEvent {
                        actor_type_id: drag.actor_type_id.clone(),
                        coord: *coord,
                    });
                    commands.trigger(SpawDropParticlesEvent(*coord));
                    next_state.set(GameplayState::Placement);
                }
            }
        }
        DragSource::Board {
            dragged_entity,
            start_coord,
        } => {
            if let Some(target_coord) = **hovered_tile_coord {
                if drag.can_drop {
                    commands.trigger(SpawDropParticlesEvent(target_coord));
                    game.swap_coords(start_coord, target_coord);
                    if let Some((target_entity, _actor_view)) = &**hovered_actor {
                        *q_tr.get_mut(*target_entity).unwrap() = Transform::from_translation(
                            tile_coord_to_world_coord(start_coord, config.checker.tile_size)
                                .extend(2.0),
                        );
                        commands.trigger(SpawDropParticlesEvent(start_coord));
                    }
                    *q_tr.get_mut(dragged_entity).unwrap() = Transform::from_translation(
                        tile_coord_to_world_coord(target_coord, config.checker.tile_size)
                            .extend(2.0),
                    );
                    q_actor_sprite.get_mut(dragged_entity).unwrap().color =
                        Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
                    commands.trigger(ActorRotationFixupEvent);
                    next_state.set(GameplayState::Placement);
                }
            }
        }
    }
}
