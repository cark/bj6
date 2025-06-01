use bevy::prelude::*;

#[derive(Resource)]
#[allow(dead_code)]
pub struct GameConfigHandle(pub Handle<GameConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Clone)]
pub struct GameConfig {
    pub camera: CameraConfig,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct CameraConfig {
    pub follow_speed: f32,
}
