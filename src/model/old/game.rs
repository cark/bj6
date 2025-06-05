use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct Game {
    pub gold: u64,
    pub turns_left: u64,
    pub round: u32,
    pub required_gold: u64,
}

impl Game {
    pub fn new(start_gold: u64) -> Self {
        let mut result = Self {
            gold: 1,
            turns_left: 5,
            round: 1,
            required_gold: 10,
        };
        result.gold = start_gold;
        result
    }
}
