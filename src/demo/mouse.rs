use bevy::prelude::*;

use crate::{AppSystems, screens::Screen};

pub fn plugin(app: &mut App) {
    app.init_state::<MouseState>();
    app.add_systems(
        Update,
        (
            start_panning.run_if(in_state(MouseState::Normal)),
            stop_panning.run_if(in_state(MouseState::Pan)),
        )
            .in_set(AppSystems::RecordInput),
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
