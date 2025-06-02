pub mod actor;
mod camera;
pub mod level;
mod mouse;
pub mod tile;
pub mod ui;

use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameplayState>();
    app.add_plugins((
        // animation::plugin,
        mouse::plugin,
        camera::plugin,
        level::plugin,
        actor::plugin,
        tile::plugin,
        ui::plugin,
    ));
}

/// gameplay mouse states
#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum GameplayState {
    #[default]
    Placement,
    Shop,
    Run,
}
