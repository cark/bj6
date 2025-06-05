use std::f32::consts::PI;

use bevy::{prelude::*, sprite};

use crate::{
    data::game_config::GameConfig,
    demo::ui::actions::SetActiveActionEvent,
    model::{
        actor::{Actor, Direction},
        actor_type::ActorTypes,
        board::Board,
    },
    screens::Screen,
};

use super::{
    GameplayState,
    drag::{DragSource, StartDragEvent},
    tile::{HoveredActorEntity, HoveredTileCoord, tile_coord_to_world_coord},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_actor_spawned);
    app.add_observer(on_spawn_actor);
    app.add_systems(
        Update,
        (actor_click, update_actions, rotate).run_if(in_state(GameplayState::Placement)),
    );
    app.add_observer(on_actor_rotation_fixup);
}

pub fn on_actor_rotation_fixup(
    _: Trigger<ActorRotationFixupEvent>,
    mut q_actor: Query<(&Actor, &mut Transform, &mut Sprite)>,
) {
    for (actor, mut tr, mut sprite) in &mut q_actor {
        match actor.looks_to {
            Direction::Up => {
                tr.rotation = Quat::from_rotation_z(PI / 2.0);
                sprite.flip_x = false;
            }
            Direction::Down => {
                tr.rotation = Quat::from_rotation_z(-PI / 2.0);
                sprite.flip_x = false;
            }
            Direction::Left => {
                tr.rotation = Quat::from_rotation_z(0.);
                sprite.flip_x = true;
            }
            Direction::Right => {
                tr.rotation = Quat::from_rotation_z(0.);
                sprite.flip_x = false;
            }
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct ActorRotationFixupEvent;

fn update_actions(
    mut commands: Commands,
    q_actor: Query<&Actor>,
    hovered_actor_entity: Res<HoveredActorEntity>,
    // gameplay_state: Res<State<GameplayState>>,
) {
    let (_actor_hover, actor_rotatable) = if let Some(entity) = **hovered_actor_entity {
        if let Ok(actor) = q_actor.get(entity) {
            (true, actor.rotatable)
        } else {
            (false, false)
        }
    } else {
        (false, false)
    };
    commands.trigger(SetActiveActionEvent(
        "r_rotate".to_string(),
        actor_rotatable,
    ));
}

#[derive(Component, Debug, Clone, Deref)]
pub struct Coord(IVec2);

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self(ivec2(x, y))
    }
}

impl From<IVec2> for Coord {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpawnActorEvent {
    pub actor_type_id: String,
    pub coord: IVec2,
}

pub fn on_actor_spawned(
    trigger: Trigger<OnAdd, Actor>,
    mut commands: Commands,
    actor_types: Res<ActorTypes>,
    config: Res<GameConfig>,
    actors: Query<(&Actor, &Coord)>,
) {
    let entity = trigger.target();
    let (actor, coord) = actors.get(entity).unwrap();
    let actor_type = actor_types.get(&actor.actor_type).unwrap();
    let translation = tile_coord_to_world_coord(**coord, config.checker.tile_size);
    commands
        .entity(entity)
        .insert((
            StateScoped(Screen::Gameplay),
            Visibility::default(),
            Name::new(actor_type.name.clone()),
            Transform::from_translation(translation.extend(2.0)),
            Sprite {
                image: actor_type.sprite_handle.clone().unwrap(),
                custom_size: Some(Vec2::splat(config.checker.tile_size)),
                ..default()
            },
        ))
        .remove::<Coord>();
}

pub fn on_spawn_actor(
    trigger: Trigger<SpawnActorEvent>,
    mut commands: Commands,
    mut board: ResMut<Board>,
    actor_types: Res<ActorTypes>,
) {
    let ev = trigger.event();
    let actor = Actor::new(&actor_types, &ev.actor_type_id, ev.coord);
    let entity = commands.spawn((actor, Coord::from(ev.coord))).id();
    if let Err(err) = board.set(ev.coord, entity) {
        warn!("{err}");
        commands.entity(entity).despawn();
    }
}

pub fn actor_click(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    hovered_actor_entity: Res<HoveredActorEntity>,
    hovered_tile_coord: Res<HoveredTileCoord>,
    q_actor: Query<&Actor, With<Actor>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(entity) = **hovered_actor_entity {
            if let Some(coord) = **hovered_tile_coord {
                if let Ok(actor) = q_actor.get(entity) {
                    if actor.dragable {
                        commands.trigger(StartDragEvent {
                            source: DragSource::Board {
                                dragged_entity: entity,
                                start_coord: coord,
                            },
                            actor_type_id: q_actor.get(entity).unwrap().actor_type.clone(),
                        })
                    }
                }
            }
        }
    }
}

fn rotate(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    hovered_actor_entity: Res<HoveredActorEntity>,
    mut q_actor: Query<&mut Actor>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        if let Some(entity) = **hovered_actor_entity {
            if let Ok(mut actor) = q_actor.get_mut(entity) {
                if actor.rotatable {
                    // warn!("rotate");

                    actor.rotate();
                    commands.trigger(ActorRotationFixupEvent);

                    // match actor.looks_to {
                    //     Direction::Up => {
                    //         tr.rotation = Quat::from_rotation_z(PI / 2.0);
                    //         sprite.flip_x = false;
                    //     }
                    //     Direction::Down => {
                    //         tr.rotation = Quat::from_rotation_z(-PI / 2.0);
                    //         sprite.flip_x = false;
                    //     }
                    //     Direction::Left => {
                    //         tr.rotation = Quat::from_rotation_z(0.);
                    //         sprite.flip_x = true;
                    //     }
                    //     Direction::Right => {
                    //         tr.rotation = Quat::from_rotation_z(0.);
                    //         sprite.flip_x = false;
                    //     }
                    // }
                }
            }
        }
    }
}
