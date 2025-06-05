use bevy::prelude::*;
use thiserror::*;

#[derive(Debug, Resource, Clone)]
pub struct Board {
    tiles: im::HashMap<IVec2, Entity>,
    start_actor: Entity,
}

#[derive(Error, Debug)]
pub enum BoardError {
    #[error("coord already taken")]
    CoordAlreadyTaken,
    #[error("cannot move start actor")]
    CannotMoveStartActor,
    #[error("nothing to move")]
    NothingToMove,
}

impl Board {
    pub fn new(start_actor: Entity) -> Self {
        let mut tiles = im::HashMap::new();
        tiles.insert(ivec2(0, 0), start_actor);
        Self { tiles, start_actor }
    }

    pub fn get(&self, coord: IVec2) -> Option<Entity> {
        self.tiles.get(&coord).copied()
    }

    pub fn set(&mut self, coord: IVec2, entity: Entity) -> Result<(), BoardError> {
        if self.tiles.contains_key(&coord) {
            return Err(BoardError::CoordAlreadyTaken);
        }
        self.tiles.insert(coord, entity);
        Ok(())
    }

    pub fn start_actor(&self) -> Entity {
        self.start_actor
    }

    pub fn swap(&mut self, from: IVec2, to: IVec2) -> Result<(), BoardError> {
        match (self.tiles.get(&from), self.tiles.get(&to)) {
            (None, None) => Err(BoardError::NothingToMove),
            (None, Some(&to_e)) => {
                if to_e == self.start_actor {
                    Err(BoardError::CannotMoveStartActor)
                } else {
                    self.tiles.remove(&to);
                    self.tiles.insert(from, to_e);
                    Ok(())
                }
            }
            (Some(&from_e), None) => {
                if from_e == self.start_actor {
                    Err(BoardError::CannotMoveStartActor)
                } else {
                    self.tiles.remove(&from);
                    self.tiles.insert(to, from_e);
                    Ok(())
                }
            }
            (Some(&from_e), Some(&to_e)) => {
                if from_e == self.start_actor || to_e == self.start_actor {
                    Err(BoardError::CannotMoveStartActor)
                } else {
                    self.tiles.remove(&from);
                    self.tiles.remove(&to);
                    self.tiles.insert(to, from_e);
                    self.tiles.insert(from, to_e);
                    Ok(())
                }
            }
        }
    }
}
