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

use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameplayState>();
    app.add_computed_state::<Paused>();
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
    // app.add_systems(OnEnter(Paused(true)), enter_paused);
    // app.add_systems(OnExit(Paused(true)), exit_paused);
}

// fn enter_paused() {
//     warn!("Entering paused state");
// }
// fn exit_paused() {
//     warn!("Exiting paused state");
// }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Paused(bool);

impl ComputedStates for Paused {
    type SourceStates = Menu;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            Menu::Settings | Menu::Pause => Some(Paused(true)),
            _ => Some(Paused(false)),
        }
    }
}
