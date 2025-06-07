use bevy::prelude::*;

use crate::model::{direction::Dir, program::Program};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, serde::Deserialize, Component)]
#[serde(transparent)]
pub struct ActorTypeId(String);

impl ActorTypeId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, serde::Deserialize, Component)]
pub struct ActorType {
    pub name: String,
    pub program: Program,
    #[serde(default = "default_as_true")]
    pub pushable: bool,
    #[serde(default = "default_as_true")]
    pub dragable: bool,
    #[serde(default = "default_as_true")]
    pub rotatable: bool,
    #[serde(default = "default_as_right")]
    pub looks_to: Dir,
    pub max_activations: u8,
    pub prize: u8,
    pub sprite_name: String,
    pub cost: u32,
    #[serde(skip_deserializing)]
    pub sprite_handle: Option<Handle<Image>>,
    pub description: String,
}

fn default_as_true() -> bool {
    true
}

fn default_as_right() -> Dir {
    Dir::Right
}
