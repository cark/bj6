use bevy::prelude::*;

use crate::{
    AppSystems,
    data::game_config::GameConfig,
    demo::ui::actions::SetActiveActionEvent,
    model::{actor::Actor, actor_type::ActorTypes, board::Board, game::Game, shop::Shop},
};

use super::{
    GameplayState,
    actor::SpawnActorEvent,
    mouse::MouseWorldCoords,
    tile::{HoveredActorEntity, HoveredTileCoord, tile_coord_to_world_coord},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_start_drag);
    app.add_observer(on_cancel_drag);
    app.add_observer(on_drop);

    app.add_systems(
        Update,
        (move_drag_image, check_clicks)
            .in_set(AppSystems::Update)
            .run_if(in_state(GameplayState::Drag)),
    );
    app.add_systems(OnExit(GameplayState::Drag), exit);
    app.add_systems(
        Update,
        update_actions.run_if(in_state(GameplayState::Drag).or(in_state(GameplayState::Placement))),
    );
}

fn update_actions(
    mut commands: Commands,
    hovered_actor_entity: Res<HoveredActorEntity>,
    gameplay_state: Res<State<GameplayState>>,
    drag: Option<Res<Drag>>,
    q_actor: Query<&Actor>,
) {
    let dragging = gameplay_state.get() == &GameplayState::Drag;
    let (hover_actor, can_drag) = if let Some(entity) = **hovered_actor_entity {
        if let Ok(actor) = q_actor.get(entity) {
            (true, actor.dragable)
        } else {
            (false, false)
        }
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
    actor_type_id: String,
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
    pub actor_type_id: String,
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
    actor_types: Res<ActorTypes>,
    mut commands: Commands,
    config: Res<GameConfig>,
    hovered_tile_coord: Res<HoveredTileCoord>,
    mut q_sprite: Query<&mut Sprite, With<Actor>>,
) {
    let ev = trigger.event();
    if let Some(actor_type) = actor_types.get(&ev.actor_type_id) {
        commands.insert_resource(Drag {
            actor_type_id: ev.actor_type_id.clone(),
            source: ev.source,
            can_drop: false,
        });
        next_state.set(GameplayState::Drag);

        let entity = commands
            .spawn((
                StateScoped(GameplayState::Drag),
                DragImage,
                Sprite {
                    image: actor_type.sprite_handle.clone().unwrap(),
                    color: Color::linear_rgba(1.0, 1.0, 1.0, 0.5),
                    custom_size: Some(Vec2::splat(config.checker.tile_size * config.drag.scale)),
                    ..default()
                },
            ))
            .id();
        if let Some(coord) = **hovered_tile_coord {
            let tile_size = config.checker.tile_size;
            commands.entity(entity).insert(Transform::from_translation(
                tile_coord_to_world_coord(coord, tile_size).extend(3.0),
            ));
        }
        if let DragSource::Board { dragged_entity, .. } = ev.source {
            q_sprite.get_mut(dragged_entity).unwrap().color =
                Color::linear_rgba(1.0, 1.0, 1.0, config.drag.alpha);
        }
    }
}

fn move_drag_image(
    mouse_world_coords: Res<MouseWorldCoords>,
    image_tr: Single<(&mut Transform, &mut Sprite), With<DragImage>>,
    hovered_actor_entity: Res<HoveredActorEntity>,
    hovered_tile_coord: Res<HoveredTileCoord>,
    config: Res<GameConfig>,
    mut drag: ResMut<Drag>,
    board: Res<Board>,
) {
    drag.can_drop = false;
    let tile_size = config.checker.tile_size;
    if let Some(mouse_coords) = mouse_world_coords.0 {
        let (mut tr, mut sprite) = image_tr.into_inner();
        if hovered_actor_entity.as_ref().is_some() {
            let can = match drag.source {
                DragSource::Board { .. } => {
                    let hovered_entity = hovered_actor_entity.as_ref().unwrap();
                    hovered_entity != board.start_actor()
                }
                DragSource::Shop { .. } => false,
            };
            if can {
                if let Some(coord) = &**hovered_tile_coord {
                    tr.translation = tile_coord_to_world_coord(*coord, tile_size).extend(3.0);
                    sprite.color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
                }
                drag.can_drop = true;
            } else {
                tr.translation = mouse_coords.extend(3.0);
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

    actor_types: Res<ActorTypes>,
    mut shop: ResMut<Shop>,
    mut game: ResMut<Game>,
    drag: Res<Drag>,
    mut q_actor_sprite: Query<&mut Sprite, With<Actor>>,
) {
    match drag.source {
        DragSource::Shop { shop_index } => {
            if shop.return_item(&drag.actor_type_id, shop_index, &mut game, &actor_types) {
                next_state.set(GameplayState::Placement);
            }
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
    mut board: ResMut<Board>,
    drag: Res<Drag>,
    hovered_actor_entity: Res<HoveredActorEntity>,
    hovered_tile_coord: Res<HoveredTileCoord>,
    mut q_tr: Query<&mut Transform, With<Actor>>,
    mut q_actor_sprite: Query<&mut Sprite, With<Actor>>,
    config: Res<GameConfig>,
) {
    match drag.source {
        DragSource::Shop { .. } => {
            if let Some(coord) = &**hovered_tile_coord {
                if drag.can_drop {
                    commands.trigger(SpawnActorEvent {
                        actor_type_id: drag.actor_type_id.clone(),
                        coord: *coord,
                    });
                    next_state.set(GameplayState::Placement);
                }
            }
        }
        DragSource::Board {
            dragged_entity,
            start_coord,
        } => {
            if let Some(target_coord) = **hovered_tile_coord {
                if drag.can_drop && board.swap(start_coord, target_coord).is_ok() {
                    if let Some(target_entity) = &**hovered_actor_entity {
                        *q_tr.get_mut(*target_entity).unwrap() = Transform::from_translation(
                            tile_coord_to_world_coord(start_coord, config.checker.tile_size)
                                .extend(2.0),
                        );
                    }
                    *q_tr.get_mut(dragged_entity).unwrap() = Transform::from_translation(
                        tile_coord_to_world_coord(target_coord, config.checker.tile_size)
                            .extend(2.0),
                    );
                    q_actor_sprite.get_mut(dragged_entity).unwrap().color =
                        Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
                    next_state.set(GameplayState::Placement);
                }
            }
        }
    }
}
