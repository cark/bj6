use bevy::{platform::collections::HashMap, prelude::*};

use crate::model::actor_type::{ActorType, ActorTypeId};

#[derive(Debug, Clone, serde::Deserialize, Resource, Asset, TypePath)]
pub struct ActorTypes(pub HashMap<ActorTypeId, ActorType>);

impl ActorTypes {
    pub fn get(&self, actor_type_id: &ActorTypeId) -> Option<&ActorType> {
        self.0.get(actor_type_id)
    }

    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ActorTypeId, &ActorType)> {
        self.0.iter()
    }
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct ActorTypesHandle(pub Handle<ActorTypes>);
