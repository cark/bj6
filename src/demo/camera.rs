use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{AppSystems, data::game_config::GameConfig, screens::Screen};

use super::mouse::MouseState;

pub fn plugin(app: &mut App) {
    app.insert_resource(CameraDestination::default());
    app.add_systems(
        Update,
        move_camera
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        Update,
        move_destination
            .in_set(AppSystems::Update)
            .run_if(in_state(MouseState::Pan)),
    );
}

#[derive(Debug, Default, Clone, Resource)]

struct CameraDestination(Vec2);

fn move_camera(
    mut _cmd: Commands,
    destination: Res<CameraDestination>,
    camera: Single<(&Camera2d, &mut Transform)>,
    time: Res<Time>,
    config: Res<GameConfig>,
    // config: Res<Handle<GameConfig>>,
) {
    // warn!(config.camera.follow_speed);
    let (_, mut camera_transform) = camera.into_inner();
    let target = destination.0.extend(0.0);
    let decay_rate = f32::ln(config.camera.follow_decay);
    let delta = time.delta_secs();
    let mut camera_pos = camera_transform.translation;
    camera_pos.smooth_nudge(&target, decay_rate, delta);
    *camera_transform = Transform::from_translation(camera_pos);
}

fn move_destination(
    mut destination: ResMut<CameraDestination>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let delta = if cfg!(target_family = "wasm") {
        mouse_motion.delta
    } else {
        mouse_motion.delta * 2.
    };
    let x = destination.0.x - delta.x;
    let y = destination.0.y + delta.y;

    *destination = CameraDestination(vec2(x, y));
}
