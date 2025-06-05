use bevy::{platform::collections::HashMap, prelude::*};

use super::{actor::Direction, program::Program};

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
    pub looks_to: Direction,
    pub max_activations: u8,
    pub prize: u8,
    pub sprite_name: String,
    pub cost: u32,
    #[serde(skip_deserializing)]
    pub sprite_handle: Option<Handle<Image>>,
}

fn default_as_true() -> bool {
    true
}

fn default_as_right() -> Direction {
    Direction::Right
}

// impl Default for ActorType {
//     fn default() -> Self {
//         Self {
//             program: Program::default(),
//             moveable: true,à
//             max_activations: 1,
//             activation_prize: 1,
//             sprite_name: "".to_string(),
//         }
//     }
// }

#[derive(Debug, Clone, serde::Deserialize, Resource, Asset, TypePath)]
pub struct ActorTypes(pub HashMap<String, ActorType>);

impl ActorTypes {
    pub fn get(&self, actor_type_id: &str) -> Option<&ActorType> {
        self.0.get(actor_type_id)
    }
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct ActorTypesHandle(pub Handle<ActorTypes>);

#[derive(Component, Debug, Clone)]
pub struct ActorTypeId(pub String);
