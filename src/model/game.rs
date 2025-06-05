use bevy::prelude::*;

use crate::{
    data::game_config::GameGameConfig,
    model::{
        actor::{Actor, ActorId, ActorView},
        actor_type::ActorTypeId,
        actor_types::ActorTypes,
        board::Board,
        shop::Shop,
    },
};

#[derive(Resource, Clone, Debug)]
pub struct Game {
    gold: u64,
    turns_left: u64,
    round: u32,
    required_gold: u64,
    board: Board,
    actor_types: ActorTypes,
    shop: Shop,
}

impl Game {
    pub fn new(game_config: &GameGameConfig, actor_types: ActorTypes) -> Self {
        let board = Board::new(&actor_types);
        let mut result = Self {
            gold: 1,
            turns_left: 5,
            round: 1,
            required_gold: 10,
            board,
            actor_types,
            shop: Shop::new(game_config.restock_multiplier),
        };
        result.gold = game_config.start_gold;
        result
    }

    pub fn gold(&self) -> u64 {
        self.gold
    }

    pub fn turns_left(&self) -> u64 {
        self.turns_left
    }

    pub fn round(&self) -> u32 {
        self.round
    }

    pub fn required_gold(&self) -> u64 {
        self.required_gold
    }

    pub fn actor_view(&self, actor_id: &ActorId) -> Option<ActorView> {
        if let Some(actor) = self.board.actor_id_to_actor(actor_id) {
            if let Some(actor_type) = self.actor_types.get(&actor.actor_type_id) {
                return Some(ActorView {
                    actor: actor.clone(),
                    actor_type: actor_type.clone(),
                    actor_id: *actor_id,
                });
            }
        }
        None
    }

    pub fn restock(&mut self) {
        self.shop.restock(&self.actor_types, &mut self.gold);
    }

    pub fn can_restock(&self) -> bool {
        self.gold >= self.shop.restock_cost()
    }

    pub fn buy_item(&mut self, actor_type_id: &ActorTypeId) -> bool {
        if let Some(actor_type) = self.actor_types.get(actor_type_id) {
            return self
                .shop
                .buy_item(actor_type_id, actor_type, &mut self.gold);
        }
        false
    }

    pub fn return_item(&mut self, actor_type_id: &ActorTypeId, index: usize) {
        if let Some(actor_type) = self.actor_types.get(actor_type_id) {
            self.shop
                .return_item(actor_type_id, actor_type, index, &mut self.gold);
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn swap_coords(&mut self, coord1: IVec2, coord2: IVec2) {
        self.board.swap_coords(coord1, coord2);
    }

    pub fn new_actor(&mut self, actor_type_id: &ActorTypeId, coord: IVec2) -> Option<ActorId> {
        let actor_type = self.actor_types.get(actor_type_id)?;
        let actor = Actor::from_actor_type(actor_type_id, actor_type, coord);
        self.board.add_actor(actor).ok()
    }

    pub fn rotate_actor(&mut self, actor_id: &ActorId) {
        self.board.update_actor(actor_id, |actor| actor.rotate());
    }

    pub fn actor_types(&self) -> &ActorTypes {
        &self.actor_types
    }

    pub fn shop(&self) -> &Shop {
        &self.shop
    }
}
