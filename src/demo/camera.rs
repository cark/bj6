use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
};

use crate::{AppSystems, camera::MainCamera, data::game_config::GameConfig, screens::Screen};

use super::mouse::MouseState;

pub fn plugin(app: &mut App) {
    app.insert_resource(CameraDestination {
        translation: vec2(0.0, 0.0),
        scale: 1.0,
    });
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
    app.add_systems(
        Update,
        zoom_destination
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Debug, Clone, Resource)]

struct CameraDestination {
    translation: Vec2,
    scale: f32,
}

fn move_camera(
    destination: Res<CameraDestination>,
    camera: Single<(&mut Transform, &mut Projection), With<MainCamera>>,
    time: Res<Time>,
    config: Res<GameConfig>,
    // config: Res<Handle<GameConfig>>,
) {
    // warn!(config.camera.follow_speed);
    let (mut camera_transform, mut projection) = camera.into_inner();
    let target = destination.translation.extend(0.0);
    let decay_rate = f32::ln(config.camera.follow_decay);
    let delta = time.delta_secs();
    let mut camera_pos = camera_transform.translation;
    camera_pos.smooth_nudge(&target, decay_rate, delta);
    *camera_transform = Transform::from_translation(camera_pos);

    if let Projection::Orthographic(ref mut ortho) = *projection {
        ortho
            .scale
            .smooth_nudge(&destination.scale, decay_rate, time.delta_secs());
    }
}

fn move_destination(
    mut destination: ResMut<CameraDestination>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    camera: Single<&Projection, With<MainCamera>>,
    // config: Res<GameConfig>,
) {
    if let Projection::Orthographic(ortho) = camera.into_inner() {
        let delta = if cfg!(target_family = "wasm") {
            mouse_motion.delta
        } else {
            mouse_motion.delta * 2.
        };
        let x = destination.translation.x - delta.x * ortho.scale;
        let y = destination.translation.y + delta.y * ortho.scale;

        destination.translation = vec2(x, y);
    }
}

fn zoom_destination(
    mut destination: ResMut<CameraDestination>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    let scroll_amount = match mouse_scroll.unit {
        MouseScrollUnit::Line => mouse_scroll.delta.y * 100.,
        MouseScrollUnit::Pixel => mouse_scroll.delta.y,
    };
    if scroll_amount != 0.0 {
        destination.scale *= 1.0 - scroll_amount / 1000.0;
        if let Some(position) = window.into_inner().cursor_position() {
            let (camera, camera_tr) = camera.into_inner();
            let mouse_world_pos = camera
                .viewport_to_world_2d(camera_tr, position)
                .unwrap_or(destination.translation);
            destination.translation.smooth_nudge(
                &mouse_world_pos,
                f32::ln(config.camera.follow_decay),
                time.delta_secs() * 2.0,
            );
        }
    }
}
