use bevy::prelude::*;

use crate::{
    data::game_config::GameConfig,
    model::{actor::Actor, actor_type::ActorTypes},
    screens::Screen,
};

use super::tile::tile_coord_to_world_coord;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_actor_spawned);
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
