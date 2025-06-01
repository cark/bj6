use bevy::prelude::*;

#[derive(Debug, Resource, Clone)]
pub struct Board {
    tiles: im::HashMap<IVec2, Entity>,
    start_actor: Entity,
}

impl Board {
    pub fn new(start_actor: Entity) -> Self {
        let mut tiles = im::HashMap::new();
        tiles.insert(ivec2(0, 0), start_actor);
        Self {
            tiles: im::HashMap::new(),
            start_actor,
        }
    }
}
