use bevy::prelude::*;

use crate::model::direction::RelDir;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type", content = "arg")]
pub enum Action {
    Forward,
    Push(RelDir),
    Turn(RelDir),
    Hit(Vec<IVec2>),
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub struct Program(pub Vec<Action>);

impl Program {
    pub fn iter(&self) -> impl Iterator<Item = &Action> {
        self.0.iter()
    }
}
