use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
};

use crate::{
    AppSystems,
    camera::MainCamera,
    data::game_config::GameConfig,
    demo::{Paused, tile::tile_coord_to_world_coord},
    model::{actor::ActorId, game::Game},
    screens::Screen,
};

use super::{GameplayState, mouse::MouseState};

pub fn plugin(app: &mut App) {
    app.insert_resource(CameraDestination {
        translation: vec2(0.0, 0.0),
        scale: 1.0,
    });
    app.insert_resource(CameraDecay(1.0));
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
        (zoom_destination, apply_zoom_limits)
            .chain()
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay).and(in_state(Paused(false)))),
    );
    app.add_systems(OnEnter(GameplayState::Turn), lower_decay);
    app.add_systems(OnEnter(GameplayState::Placement), higher_decay);

    app.add_observer(on_camera_to_actor);
}

#[derive(Debug, Clone, Copy, Event)]
pub struct CameraToActorEvent(pub ActorId);

#[derive(Debug, Clone, Copy, Resource)]

pub struct CameraDestination {
    pub translation: Vec2,
    pub scale: f32,
}

#[derive(Debug, Clone, Copy, Resource)]
struct CameraDecay(f32);

fn lower_decay(mut decay: ResMut<CameraDecay>, config: Res<GameConfig>) {
    decay.0 = config.camera.turn_decay;
}

fn higher_decay(mut decay: ResMut<CameraDecay>, config: Res<GameConfig>) {
    decay.0 = config.camera.follow_decay;
}

fn on_camera_to_actor(
    trigger: Trigger<CameraToActorEvent>,
    mut camera_destination: ResMut<CameraDestination>,
    game: Res<Game>,
    config: Res<GameConfig>,
) {
    let ev = trigger.event();
    let actor = game.actor_view(&ev.0).unwrap();
    let translation = tile_coord_to_world_coord(actor.actor.coord, config.checker.tile_size);
    camera_destination.translation = translation;
    camera_destination.scale = 0.5;
}

fn move_camera(
    destination: Res<CameraDestination>,
    camera: Single<(&mut Transform, &mut Projection), With<MainCamera>>,
    time: Res<Time>,
    camera_decay: Res<CameraDecay>,
) {
    let (mut camera_transform, mut projection) = camera.into_inner();
    let target = destination.translation.extend(0.0);
    let decay_rate = f32::ln(camera_decay.0);
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
    gameplay_state: Res<State<GameplayState>>,
    time: Res<Time>,
    decay: Res<CameraDecay>,
    config: Res<GameConfig>,
) {
    if ![GameplayState::Placement, GameplayState::Drag].contains(gameplay_state.get()) {
        return;
    }
    let scroll_amount = match mouse_scroll.unit {
        MouseScrollUnit::Line => mouse_scroll.delta.y * 100.,
        MouseScrollUnit::Pixel => mouse_scroll.delta.y,
    };
    if scroll_amount != 0.0 {
        let window = window.into_inner();
        let (min_scale, max_scale) = calc_scale_bounds(window.width(), window.height(), &config);
        let calculated_scale =
            (destination.scale * 1.0 - scroll_amount / 1000.0).clamp(min_scale, max_scale);
        if destination.scale != calculated_scale {
            destination.scale = calculated_scale;
            if let Some(position) = window.cursor_position() {
                let (camera, camera_tr) = camera.into_inner();
                let mouse_world_pos = camera
                    .viewport_to_world_2d(camera_tr, position)
                    .unwrap_or(destination.translation);
                destination.translation.smooth_nudge(
                    &mouse_world_pos,
                    f32::ln(decay.0),
                    time.delta_secs() * 2.0,
                );
            }
        }
    }
}

pub fn calc_scale_bounds(window_width: f32, window_height: f32, config: &GameConfig) -> (f32, f32) {
    let tile_size = config.checker.tile_size;
    let zoom_min = config.camera.zoom_min_tiles;
    let zoom_max = config.camera.zoom_max_tiles;

    // Compute the minimum‐allowed scale so that at least `zoom_min_tiles` appear
    let min_h = tile_size * zoom_min / window_width;
    let min_v = tile_size * zoom_min / window_height;
    let min_scale = min_h.max(min_v);

    // Compute the maximum‐allowed scale so that at most `zoom_max_tiles` appear
    let max_h = tile_size * zoom_max / window_width;
    let max_v = tile_size * zoom_max / window_height;
    let mut max_scale = max_h.min(max_v);

    // If, under extreme aspect ratios, min_scale > max_scale, force them equal
    if min_scale > max_scale {
        max_scale = min_scale;
    }

    (min_scale, max_scale)
}

fn apply_zoom_limits(
    window: Single<&Window>,
    mut destination: ResMut<CameraDestination>,
    config: Res<GameConfig>,
) {
    let window = window.into_inner();

    let (min_scale, max_scale) = calc_scale_bounds(window.width(), window.height(), &config);

    destination.scale = destination.scale.clamp(min_scale, max_scale);
}
