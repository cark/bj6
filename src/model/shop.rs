use bevy::prelude::*;

use super::{
    actor_type::{ActorType, ActorTypes},
    game::Game,
};

const RESTOCK_MULTIPLIER: f32 = 1.5;
const RESTOCK_ITEM_COUNT: usize = 3;

#[derive(Debug, Resource, Clone, Default)]
pub struct Shop {
    pub restock_cost: f32,
    pub items: Vec<String>,
}

impl Shop {
    pub fn restock(&mut self, game: &mut Game, actor_types: &ActorTypes) {
        if game.cash < self.restock_cost as u64 {
            return;
        }

        let mut valid_actors: Vec<(&String, &ActorType)> = actor_types
            .0
            .iter()
            .filter(|(_name, actor_type)| {
                (actor_type.cost > 0) && (actor_type.cost as u64 <= game.cash)
            })
            .collect();

        self.items.clear();

        for _i in 0..RESTOCK_ITEM_COUNT {
            if valid_actors.is_empty() {
                break;
            }

            let index = rand::random::<usize>() % valid_actors.len();
            let (name, _actor_type) = valid_actors.remove(index);
            self.items.push(name.clone());
        }

        // first restock is free
        if self.restock_cost == 0.0 {
            self.restock_cost = 1.0
        } else {
            game.cash -= self.restock_cost as u64;
            self.restock_cost *= RESTOCK_MULTIPLIER
        }
    }
}
