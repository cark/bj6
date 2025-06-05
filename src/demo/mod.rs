pub mod actor;
mod camera;
pub mod drag;
pub mod level;
mod mouse;
pub mod tile;
pub mod ui;

use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        // animation::plugin,
        mouse::plugin,
        camera::plugin,
        level::plugin,
        actor::plugin,
        tile::plugin,
        ui::plugin,
        drag::plugin,
    ));
    app.init_state::<GameplayState>();
}

#[derive(SubStates, Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum GameplayState {
    #[default]
    WorkaroundBugs,
    Placement,
    Shop,
    Drag,
    // Run,
}
