use super::actor_type::ActorTypes;

use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Actor {
    pub actor_type: String,
    pub looks_to: Direction,
    pub coord: IVec2,
    pub moveable: bool,
    pub pickupable: bool,
}

impl Actor {
    pub fn new(actor_types: &ActorTypes, type_name: &str, coord: IVec2) -> Self {
        let actor_type = actor_types.get(type_name).unwrap();
        Self {
            actor_type: type_name.to_string(),
            looks_to: actor_type.looks_to,
            coord,
            moveable: actor_type.moveable,
            pickupable: actor_type.pickupable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
