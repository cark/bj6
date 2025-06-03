use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Game {
    pub cash: u64,
    pub turns_left: u64,
    pub round: u32,
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
