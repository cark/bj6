use bevy::prelude::*;

#[derive(Resource)]
#[allow(dead_code)]
pub struct GameConfigHandle(pub Handle<GameConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Clone)]
pub struct GameConfig {
    pub camera: CameraConfig,
    pub checker: Checker,
    pub game: GameGameConfig,
    pub drag: DragConfig,
    pub ui: UiConfig,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct CameraConfig {
    pub follow_decay: f32,
    pub zoom_min_tiles: f32,
    pub zoom_max_tiles: f32,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct Checker {
    pub tile_size: f32,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct GameGameConfig {
    pub start_gold: u64,
    pub restock_multiplier: f32,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct DragConfig {
    pub scale: f32,
    pub alpha: f32,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct UiConfig {
    pub turn_startup_duration: f32,
    pub turn_startup_shake: f32,
}
