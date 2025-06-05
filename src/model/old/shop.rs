use bevy::prelude::*;

use super::{
    actor_type::{ActorType, ActorTypes},
    game::Game,
};

const RESTOCK_ITEM_COUNT: usize = 3;

#[derive(Debug, Resource, Clone)]
pub struct Shop {
    pub(super) restock_cost: f32,
    pub(super) restock_multiplier: f32,
    pub(super) items: Vec<String>,
}

impl Shop {
    pub fn new(restock_multiplier: f32) -> Self {
        Self {
            restock_cost: 0.0,
            restock_multiplier,
            items: Vec::new(),
        }
    }

    pub fn restock(&mut self, game: &mut Game, actor_types: &ActorTypes) {
        if game.gold < self.restock_cost as u64 {
            return;
        }

        let mut valid_actors: Vec<(&String, &ActorType)> = actor_types
            .0
            .iter()
            .filter(|(_name, actor_type)| {
                (actor_type.cost > 0) && (actor_type.cost as u64 <= game.gold)
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
            game.gold -= self.restock_cost as u64;
            self.restock_cost *= self.restock_multiplier
        }
    }

    pub fn can_restock(&self, game: &Game) -> bool {
        game.gold >= self.restock_cost as u64
    }

    pub fn index_of(&self, actor_type_id: &str) -> Option<usize> {
        self.items.iter().position(|a| a.as_str() == actor_type_id)
    }

    pub fn take_item(
        &mut self,
        actor_type_id: &str,
        game: &mut Game,
        actor_types: &ActorTypes,
    ) -> bool {
        if let Some(index) = self.index_of(actor_type_id) {
            if let Some(actor_type) = actor_types.get(actor_type_id) {
                if game.gold >= actor_type.cost as u64 {
                    game.gold -= actor_type.cost as u64;
                    self.items.remove(index);
                    return true;
                }
            }
        }
        false
    }

    pub fn return_item(
        &mut self,
        actor_type_id: &str,
        index: usize,
        game: &mut Game,
        actor_types: &ActorTypes,
    ) -> bool {
        if let Some(actor_type) = actor_types.get(actor_type_id) {
            game.gold += actor_type.cost as u64;
            self.items.insert(index, actor_type_id.to_string());
            return true;
        }
        false
    }

    pub fn restock_cost(&self) -> u64 {
        self.restock_cost as u64
    }
}
