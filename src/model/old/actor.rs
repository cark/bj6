use super::actor_type::ActorTypes;

use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Actor {
    pub actor_type_id: ActorTypeId,
    pub looks_to: Direction,
    pub coord: IVec2,
    pub pushable: bool,
    pub dragable: bool,
    pub rotatable: bool,
}

impl Actor {
    // pub fn new(actor_types: &ActorTypes, type_name: &str, coord: IVec2) -> Self {
    //     let actor_type = actor_types.get(type_name).unwrap();
    //     Self {
    //         actor_type_id: type_name.to_string(),
    //         looks_to: actor_type.looks_to,
    //         // coord,
    //         pushable: actor_type.pushable,
    //         dragable: actor_type.dragable,
    //         rotatable: actor_type.rotatable,
    //     }
    // }

    pub fn rotate(&mut self) {
        self.looks_to = self.looks_to.rotate();
    }
}
