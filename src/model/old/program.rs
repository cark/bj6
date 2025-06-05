use bevy::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type", content = "arg")]
pub enum Action {
    Forward,
    Push(Push),
    TurnLeft,
    TurnRight,
    TurnBack,
    Hit(Vec<IVec2>),
}

#[derive(Debug, Clone, serde::Deserialize)]
// #[serde(tag = "push")]
pub enum Push {
    None,
    Front,
    Left,
    Right,
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub struct Program(pub Vec<Action>);
