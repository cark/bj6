mod camera;
pub mod level;
mod mouse;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        // animation::plugin,
        mouse::plugin,
        camera::plugin,
        level::plugin,
    ));
}
