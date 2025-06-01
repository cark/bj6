use bevy::prelude::*;

#[derive(Resource)]
#[allow(dead_code)]
pub struct GameConfigHandle(pub Handle<GameConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Clone)]
pub struct GameConfig {
    pub camera: CameraConfig,
    pub checker: Checker,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct CameraConfig {
    pub follow_decay: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct Checker {
    pub tile_size: f32,
}
