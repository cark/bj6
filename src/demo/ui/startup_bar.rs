use std::time::Duration;

use bevy::prelude::*;

use crate::{
    AppSystems,
    data::game_config::GameConfig,
    demo::{
        GameplayState,
        camera::{CameraDestination, calc_scale_bounds},
        tile::{HoveredActor, tile_coord_to_world_coord},
    },
    model::game::Game,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        check_start_click.run_if(in_state(GameplayState::Placement)),
    );
    app.add_systems(
        Update,
        tick_timer
            .in_set(AppSystems::TickTimers)
            .run_if(in_state(GameplayState::TurnStartup)),
    );
    app.add_systems(OnExit(GameplayState::TurnStartup), exit);
}

#[derive(Resource, Debug, Clone)]
struct StartupInfo {
    timer: Timer,
    saved_camera_destination: CameraDestination,
}

fn check_start_click(
    mut commands: Commands,
    game: Res<Game>,
    hovered_actor: Res<HoveredActor>,
    button_input: Res<ButtonInput<MouseButton>>,
    config: Res<GameConfig>,
    mut next_state: ResMut<NextState<GameplayState>>,
    camera_destination: Res<CameraDestination>,
) {
    if button_input.just_pressed(MouseButton::Left) {
        if let Some((_, actor_view)) = &**hovered_actor {
            if game.board().start_actor_id() == actor_view.actor_id {
                commands.insert_resource(StartupInfo {
                    timer: Timer::new(
                        Duration::from_secs_f32(config.ui.turn_startup_duration),
                        TimerMode::Once,
                    ),
                    saved_camera_destination: *camera_destination,
                });
                next_state.set(GameplayState::TurnStartup);
            }
        }
    }
}

fn tick_timer(
    mut startup_info: ResMut<StartupInfo>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameplayState>>,
    window: Single<&Window>,
    config: Res<GameConfig>,
    mut camera_destination: ResMut<CameraDestination>,
    button_input: Res<ButtonInput<MouseButton>>,
) {
    startup_info.timer.tick(time.delta());
    if startup_info.timer.just_finished() {
        next_state.set(GameplayState::Turn);
        return;
    }
    if button_input.just_released(MouseButton::Left) {
        next_state.set(GameplayState::Placement);
        return;
    }

    let (min_scale, _scale) = calc_scale_bounds(window.width(), window.height(), &config);
    let t = startup_info.timer.fraction();
    let scale = EasingCurve::new(
        startup_info.saved_camera_destination.scale,
        min_scale,
        EaseFunction::Linear,
    )
    .sample_clamped(t);
    let dest = EasingCurve::new(
        startup_info.saved_camera_destination.translation,
        tile_coord_to_world_coord(IVec2::ZERO, config.checker.tile_size),
        EaseFunction::CubicOut,
    )
    .sample_clamped(t);
    let shake_scale = EasingCurve::new(
        0.0,
        config.checker.tile_size * config.ui.turn_startup_shake,
        EaseFunction::Linear,
    )
    .sample_clamped(t);
    let shake = vec2(
        rand::random::<f32>() * shake_scale,
        rand::random::<f32>() * shake_scale,
    );

    camera_destination.scale = scale;
    camera_destination.translation = dest + (shake - shake_scale / 2.0);
}

fn exit(
    mut commands: Commands,
    mut camera_destination: ResMut<CameraDestination>,
    startup_info: Res<StartupInfo>,
) {
    *camera_destination = startup_info.saved_camera_destination;
    commands.remove_resource::<StartupInfo>();
}
