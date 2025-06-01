mod top_bar;

use bevy::prelude::*;
use top_bar::{on_shop_button_clicked, top_bar_ui};

use crate::{screens::Screen, theme::widget};

use super::level::LevelAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_ui);
    // app.add_observer(on_shop_button_clicked);
}

fn spawn_ui(mut commands: Commands, assets: Res<LevelAssets>) {
    commands.spawn((
        widget::gameplay_ui_root("Gameplay Ui"),
        GlobalZIndex(2),
        StateScoped(Screen::Gameplay),
        children![top_bar_ui(&assets), content()],
    ));
}

fn content() -> impl Bundle {
    (
        Name::new("Top Bar"),
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        Pickable::IGNORE,
    )
}
