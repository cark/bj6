use bevy::prelude::*;

use crate::{
    AppSystems,
    camera::MainCamera,
    demo::{Paused, ui::actions::SetActiveActionEvent},
    screens::Screen,
    theme::interaction::ButtonHovering,
};

use super::{GameplayState, tile::HoveredActor};

pub fn plugin(app: &mut App) {
    app.init_state::<MouseState>();
    app.insert_resource(PanButton(MouseButton::Left));
    app.init_resource::<MouseCoords>();
    app.init_resource::<MouseWorldCoords>();
    app.add_systems(
        Update,
        (
            start_panning.run_if(in_state(MouseState::Normal).and(in_state(Paused(false)))),
            stop_panning.run_if(in_state(MouseState::Pan)),
            update_mouse_coords,
            update_actions
                .run_if(in_state(GameplayState::Placement).or(in_state(GameplayState::Drag))),
        )
            .in_set(AppSystems::RecordInput)
            .run_if(in_state(Screen::Gameplay)),
    );
    //app.add_systems()
}

fn update_actions(
    mut commands: Commands,
    button_hovering: Res<ButtonHovering>,
    hovered_actor_entity: Res<HoveredActor>,
    gameplay_state: Res<State<GameplayState>>,
) {
    let button_hover = button_hovering.is_hovering();
    let dragging = gameplay_state.get() == &GameplayState::Drag;
    let actor_hover = hovered_actor_entity.is_some();

    let (mmb_pan, lmb_mmb_pan) = match (button_hover, actor_hover, dragging) {
        (true, _, _) => (false, false),
        (false, true, _) => (true, false),
        (false, _, true) => (true, false),
        (false, false, false) => (false, true),
    };
    commands.trigger(SetActiveActionEvent("mmb_pan".to_string(), mmb_pan));
    commands.trigger(SetActiveActionEvent("lmb_mmb_pan".to_string(), lmb_mmb_pan));
    commands.trigger(SetActiveActionEvent("mmb_zoom".to_string(), true));
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

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PanButton(MouseButton);

#[derive(Resource, Debug, Default, Deref)]
pub struct MouseCoords(Option<Vec2>);

#[derive(Resource, Debug, Default, Deref)]
pub struct MouseWorldCoords(pub Option<Vec2>);

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
    button_hovering: Res<ButtonHovering>,
    gameplay_state: Res<State<GameplayState>>,
    mut pan_button: ResMut<PanButton>,
    hovered_actor_entity: Res<HoveredActor>,
    // mut window: Single<&mut Window>,
) {
    if !button_hovering.is_hovering() {
        if mouse_buttons.just_pressed(MouseButton::Left)
            && hovered_actor_entity.is_none()
            && gameplay_state.get() == &GameplayState::Placement
        {
            next_mouse_state.set(MouseState::Pan);
            *pan_button = PanButton(MouseButton::Left);
        } else if mouse_buttons.just_pressed(MouseButton::Middle) {
            next_mouse_state.set(MouseState::Pan);
            *pan_button = PanButton(MouseButton::Middle);
        } else {
            next_mouse_state.set(MouseState::Normal);
        }
    } else {
        next_mouse_state.set(MouseState::Normal);
    };
}

fn stop_panning(
    mouse_state: Res<State<MouseState>>,
    mut next_mouse_state: ResMut<NextState<MouseState>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    pan_button: ResMut<PanButton>,
) {
    if *mouse_state.get() == MouseState::Pan && mouse_buttons.just_released(pan_button.0) {
        next_mouse_state.set(MouseState::Normal);
    }
}
