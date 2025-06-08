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

#[derive(Resource, Clone, Debug, Default)]
pub struct Game {
    gold: u64,
    gold_this_turn: u64,
    total_gold: u64,
    turns_left: u64,
    round: u32,
    required_gold: u64,
    gold_required_multiplier: f32,
    board: Board,
    actor_types: ActorTypes,
    shop: Shop,
}

impl Game {
    pub fn new(game_config: &GameGameConfig, actor_types: ActorTypes) -> Self {
        let board = Board::new(&actor_types);
        let mut result = Self {
            gold: 1,
            gold_this_turn: 0,
            turns_left: 5,
            round: 1,
            required_gold: game_config.start_required_gold,
            gold_required_multiplier: game_config.gold_required_multiplier,
            board,
            actor_types,
            total_gold: 0,
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

    pub fn update_actor(&mut self, actor_id: &ActorId, f: impl FnOnce(&mut Actor)) {
        self.board.update_actor(actor_id, f);
    }

    pub fn rotate_actor(&mut self, actor_id: &ActorId) {
        self.update_actor(actor_id, |actor| actor.rotate());
    }

    pub fn actor_types(&self) -> &ActorTypes {
        &self.actor_types
    }

    pub fn shop(&self) -> &Shop {
        &self.shop
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }

    pub fn new_turn(&mut self) {
        self.gold_this_turn = 0;
        self.turns_left -= 1;
    }

    pub fn earn_prize_gold(&mut self, amount: u64) {
        self.gold += amount;
        self.gold_this_turn += amount;
        self.total_gold += amount;
    }

    pub fn total_gold(&self) -> u64 {
        self.total_gold
    }

    pub fn is_round_end(&self) -> bool {
        self.turns_left == 0
    }

    pub fn can_go_next_round(&self) -> bool {
        self.turns_left == 0 && self.gold >= self.required_gold
    }

    pub fn next_round(&mut self) {
        self.round += 1;
        self.turns_left = 5;
        self.required_gold = (self.required_gold as f32 * self.gold_required_multiplier) as u64;
    }
}
