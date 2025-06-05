use crate::model::{
    actor_type::{ActorType, ActorTypeId},
    actor_types::ActorTypes,
};

const RESTOCK_ITEM_COUNT: usize = 3;

#[derive(Debug, Clone)]
pub struct Shop {
    restock_cost: f32,
    restock_multiplier: f32,
    stock: Vec<ActorTypeId>,
}

impl Shop {
    pub(super) fn new(restock_multiplier: f32) -> Self {
        Self {
            restock_cost: 0.0,
            restock_multiplier,
            stock: Vec::new(),
        }
    }

    pub fn stock(&self) -> impl Iterator<Item = &ActorTypeId> {
        self.stock.iter()
    }

    pub fn restock_cost(&self) -> u64 {
        self.restock_cost as u64
    }

    pub(super) fn restock(&mut self, actor_types: &ActorTypes, game_gold: &mut u64) {
        if *game_gold < self.restock_cost as u64 {
            return;
        }

        let mut valid_actors: Vec<(&ActorTypeId, &ActorType)> = actor_types
            .iter()
            .filter(|(_name, actor_type)| {
                (actor_type.cost > 0) && (actor_type.cost as u64 <= *game_gold)
            })
            .collect();

        self.stock.clear();

        for _i in 0..RESTOCK_ITEM_COUNT {
            if valid_actors.is_empty() {
                break;
            }

            let index = rand::random::<usize>() % valid_actors.len();
            let (actor_type_id, _actor_type) = valid_actors.remove(index);
            self.stock.push(actor_type_id.clone());
        }

        // first restock is free
        if self.restock_cost == 0.0 {
            self.restock_cost = 1.0
        } else {
            *game_gold -= self.restock_cost as u64;
            self.restock_cost *= self.restock_multiplier
        }
    }

    pub(super) fn return_item(
        &mut self,
        actor_type_id: &ActorTypeId,
        actor_type: &ActorType,
        index: usize,
        game_gold: &mut u64,
    ) {
        *game_gold += actor_type.cost as u64;
        self.stock.insert(index, actor_type_id.clone());
    }

    pub(super) fn buy_item(
        &mut self,
        actor_type_id: &ActorTypeId,
        actor_type: &ActorType,
        game_gold: &mut u64,
    ) -> bool {
        if let Some(index) = self.stock.iter().position(|ati| ati == actor_type_id) {
            if *game_gold >= actor_type.cost as u64 {
                *game_gold -= actor_type.cost as u64;
                self.stock.remove(index);
                return true;
            }
        }
        false
    }
}
