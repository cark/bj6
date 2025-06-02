use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Game {
    cash: u64,
    turns_left: u64,
    round: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            cash: 1,
            turns_left: 5,
            round: 1,
        }
    }
}
