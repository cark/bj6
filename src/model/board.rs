use bevy::prelude::*;
use thiserror::*;

use crate::model::{
    actor::{Actor, ActorId},
    actor_type::ActorTypeId,
    actor_types::ActorTypes,
};

#[derive(Debug, Clone)]
pub struct Board {
    next_actor_id: usize,
    actor_id_to_actor: im::HashMap<ActorId, Actor>,
    coord_to_actor_id: im::HashMap<IVec2, ActorId>,
    start_actor_id: ActorId,
}

#[derive(Error, Debug)]
pub enum BoardError {
    #[error("coord already taken")]
    CoordAlreadyTaken,
}

impl Board {
    pub fn new(actor_types: &ActorTypes) -> Self {
        let id = ActorTypeId::new("start".to_string());
        let start_actor_type = actor_types.get(&id).unwrap();
        let start_actor = Actor::from_actor_type(&id, start_actor_type, ivec2(0, 0));

        let mut result = Self {
            next_actor_id: 1,
            actor_id_to_actor: im::HashMap::new(),
            coord_to_actor_id: im::HashMap::new(),
            start_actor_id: ActorId::new(1),
        };
        result.add_actor(start_actor).unwrap();
        result
    }

    pub fn start_actor_id(&self) -> ActorId {
        self.start_actor_id
    }

    pub fn actor_ids(&self) -> impl Iterator<Item = &ActorId> {
        self.actor_id_to_actor.keys()
    }

    pub fn actor_id_to_actor(&self, actor_id: &ActorId) -> Option<&Actor> {
        self.actor_id_to_actor.get(actor_id)
    }

    pub fn coord_to_actor_id(&self, coord: &IVec2) -> Option<ActorId> {
        self.coord_to_actor_id.get(coord).copied()
    }

    pub fn coord_to_actor(&self, coord: IVec2) -> Option<&Actor> {
        self.coord_to_actor_id(&coord)
            .and_then(|actor_id| self.actor_id_to_actor(&actor_id))
    }

    pub fn add_actor(&mut self, actor: Actor) -> Result<ActorId, BoardError> {
        let coord = actor.coord;
        if self.coord_to_actor_id.contains_key(&coord) {
            return Err(BoardError::CoordAlreadyTaken);
        }

        let actor_id = ActorId::new(self.next_actor_id);
        self.next_actor_id += 1;

        self.actor_id_to_actor.insert(actor_id, actor);
        self.coord_to_actor_id.insert(coord, actor_id);

        Ok(*self.coord_to_actor_id.get(&coord).unwrap())
    }

    /// updates a clone of the existing actor, then place that clone back in the store
    pub fn update_actor(&mut self, actor_id: &ActorId, f: impl FnOnce(&mut Actor)) {
        if let Some(mut actor) = self.actor_id_to_actor.get(actor_id).cloned() {
            let old_coord = actor.coord;
            f(&mut actor);
            let new_coord = actor.coord;

            actor.coord = old_coord;
            self.actor_id_to_actor.insert(*actor_id, actor);
            if old_coord != new_coord {
                self.swap_coords(old_coord, new_coord);
            }
        }
    }

    pub fn swap_coords(&mut self, coord1: IVec2, coord2: IVec2) {
        let actor1_id = self.coord_to_actor_id.get(&coord1).copied();
        let actor2_id = self.coord_to_actor_id.get(&coord2).copied();

        match (actor1_id, actor2_id) {
            (Some(id1), Some(id2)) => {
                // Swap coords in actor_id_to_actor
                let mut actor1 = self.actor_id_to_actor.get(&id1).unwrap().clone();
                let mut actor2 = self.actor_id_to_actor.get(&id2).unwrap().clone();
                actor1.coord = coord2;
                actor2.coord = coord1;
                self.actor_id_to_actor.insert(id1, actor1);
                self.actor_id_to_actor.insert(id2, actor2);

                // Swap actor_ids in coord_to_actor_id
                self.coord_to_actor_id.insert(coord1, id2);
                self.coord_to_actor_id.insert(coord2, id1);
            }
            (Some(id1), None) => {
                let mut actor1 = self.actor_id_to_actor.get(&id1).unwrap().clone();
                actor1.coord = coord2;
                self.actor_id_to_actor.insert(id1, actor1);
                self.coord_to_actor_id.remove(&coord1);
                self.coord_to_actor_id.insert(coord2, id1);
            }
            (None, Some(id2)) => {
                let mut actor2 = self.actor_id_to_actor.get(&id2).unwrap().clone();
                actor2.coord = coord1;
                self.actor_id_to_actor.insert(id2, actor2);
                self.coord_to_actor_id.remove(&coord2);
                self.coord_to_actor_id.insert(coord1, id2);
            }
            (None, None) => {} // Do nothing if both coords are empty
        }
    }
}
