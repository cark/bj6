use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Game {
    cash: u64,
}

impl Default for Game {
    fn default() -> Self {
        Self { cash: 1 }
    }
}
