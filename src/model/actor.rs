use crate::model::{
    actor_type::{ActorType, ActorTypeId},
    direction::Dir,
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Actor {
    pub actor_type_id: ActorTypeId,
    pub looks_to: Dir,
    pub activations_left: u8,
    pub coord: IVec2,
    pub activated: bool,
}

impl Actor {
    pub fn from_actor_type(
        actor_type_id: &ActorTypeId,
        actor_type: &ActorType,
        coord: IVec2,
    ) -> Self {
        Self {
            actor_type_id: actor_type_id.clone(),
            looks_to: actor_type.looks_to,
            activations_left: actor_type.max_activations,
            coord,
            activated: false,
        }
    }

    pub fn rotate(&mut self) {
        self.looks_to = self.looks_to.rotate();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, Component, Default)]
pub struct ActorId(usize);

impl ActorId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct ActorView {
    pub actor_id: ActorId,
    pub actor: Actor,
    pub actor_type: ActorType,
}
