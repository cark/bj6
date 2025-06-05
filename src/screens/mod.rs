//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;

use crate::demo::GameplayState;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));

    app.add_systems(OnEnter(Screen::Gameplay), set_placement_substate);
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    Splash,
    Title,
    #[default]
    Loading,
    Gameplay,
}

fn set_placement_substate(mut next_state: ResMut<NextState<GameplayState>>) {
    next_state.set(GameplayState::Placement);
}
