use bevy::prelude::*;

use crate::{
    data::game_config::GameConfig,
    model::{actor::Actor, actor_type::ActorTypes, board::Board},
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
        actor_click.run_if(in_state(GameplayState::Placement)),
    );
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
    actors: Query<&Actor>,
) {
    let entity = trigger.target();
    let actor = actors.get(entity).unwrap();
    let actor_type = actor_types.get(&actor.actor_type).unwrap();
    let translation = tile_coord_to_world_coord(actor.coord, config.checker.tile_size);
    commands.entity(entity).insert((
        StateScoped(Screen::Gameplay),
        Visibility::default(),
        Name::new(actor_type.name.clone()),
        Transform::from_translation(translation.extend(2.0)),
        Sprite {
            image: actor_type.sprite_handle.clone().unwrap(),
            custom_size: Some(Vec2::splat(config.checker.tile_size)),
            ..default()
        },
    ));
}

pub fn on_spawn_actor(
    trigger: Trigger<SpawnActorEvent>,
    mut commands: Commands,
    mut board: ResMut<Board>,
    actor_types: Res<ActorTypes>,
) {
    let ev = trigger.event();
    let actor = Actor::new(&actor_types, &ev.actor_type_id, ev.coord);
    let entity = commands.spawn(actor).id();
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
