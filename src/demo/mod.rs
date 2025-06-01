pub mod actor;
mod camera;
pub mod level;
mod mouse;
pub mod tile;
pub mod ui;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
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
