use bevy::{platform::collections::HashMap, prelude::*};

use super::{actor::Direction, program::Program};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ActorType {
    pub name: String,
    pub program: Program,
    #[serde(default = "default_as_true")]
    pub moveable: bool,
    #[serde(default = "default_as_true")]
    pub pickupable: bool,
    #[serde(default = "default_as_right")]
    pub looks_to: Direction,
    pub max_activations: u8,
    pub prize: u8,
    pub sprite_name: String,
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
    pub fn get(&self, name: &str) -> Option<&ActorType> {
        self.0.get(name)
    }
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct ActorTypesHandle(pub Handle<ActorTypes>);
