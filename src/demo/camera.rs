use bevy::prelude::*;

use crate::{AppSystems, data::game_config::GameConfig};

pub fn camera_plugin(app: &mut App) {
    app.insert_resource(CameraDestination::default());
    app.add_systems(Update, move_camera.in_set(AppSystems::Update));
}

#[derive(Debug, Default, Clone, Resource)]

struct CameraDestination {
    x: f32,
    y: f32,
}

fn move_camera(
    mut _cmd: Commands,
    camera_dest: Res<CameraDestination>,
    mut camera: Single<(&Camera2d, &mut Transform)>,
    time: Res<Time>,
    config: Res<GameConfig>,
    // config: Res<Handle<GameConfig>>,
) {
    // warn!(config.camera.follow_speed);
    let (_, camera_tranform) = camera.into_inner();
    let delta = time.delta_secs();
}
