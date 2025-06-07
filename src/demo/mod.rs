pub mod actor;
mod camera;
pub mod drag;
pub mod follow;
pub mod level;
mod mouse;
pub mod particle;
pub mod puff;
pub mod sprite_animate;
pub mod tile;
pub mod turn;
pub mod ui;

use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameplayState>();
    app.add_plugins((
        mouse::plugin,
        camera::plugin,
        level::plugin,
        actor::plugin,
        tile::plugin,
        ui::plugin,
        drag::plugin,
        puff::plugin,
        sprite_animate::plugin,
        particle::plugin,
        turn::plugin,
        follow::plugin,
    ));
}

#[derive(SubStates, Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum GameplayState {
    #[default]
    WorkaroundBugs,
    Placement,
    Shop,
    TurnStartup,
    Turn,
    Drag,
    // Run,
}
