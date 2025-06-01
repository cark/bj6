use bevy::prelude::*;

use crate::{AppSystems, camera::MainCamera, screens::Screen};

pub fn plugin(app: &mut App) {
    app.init_state::<MouseState>();
    app.init_resource::<MouseCoords>();
    app.init_resource::<MouseWorldCoords>();
    app.add_systems(
        Update,
        (
            start_panning.run_if(in_state(MouseState::Normal)),
            stop_panning.run_if(in_state(MouseState::Pan)),
            update_mouse_coords,
        )
            .in_set(AppSystems::RecordInput)
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// gameplay mouse states
#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum MouseState {
    #[default]
    Normal,
    Pan,
}

#[derive(Resource, Debug, Default, Deref)]
pub struct MouseCoords(Option<Vec2>);

#[derive(Resource, Debug, Default, Deref)]
pub struct MouseWorldCoords(Option<Vec2>);

fn update_mouse_coords(
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Single<&Window>,
    mut mouse_coords: ResMut<MouseCoords>,
    mut mouse_world_coords: ResMut<MouseWorldCoords>,
) {
    mouse_coords.0 = window.cursor_position();
    mouse_world_coords.0 = window.cursor_position().map(|pos| {
        let (camera, camera_transform) = camera.into_inner();
        camera
            .viewport_to_world_2d(camera_transform, pos)
            .unwrap_or(vec2(0.0, 0.0))
    });
}

fn start_panning(
    // mut cmd: Commands,
    mut next_mouse_state: ResMut<NextState<MouseState>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    // mut window: Single<&mut Window>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        next_mouse_state.set(MouseState::Pan);
        // window.cursor_options.visible = false;
        // window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }
}

fn stop_panning(
    // mut cmd: Commands,
    mut next_mouse_state: ResMut<NextState<MouseState>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    // mut window: Single<&mut Window>,
) {
    if mouse_buttons.just_released(MouseButton::Left) {
        next_mouse_state.set(MouseState::Normal);
        // window.cursor_options.visible = true;
        // window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}
