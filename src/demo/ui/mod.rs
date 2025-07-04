pub mod actions;
pub mod end_turn;
pub mod game_over;
pub mod shop;
pub mod smart_text;
pub mod startup_bar;
pub mod top_bar;

use bevy::prelude::*;
use top_bar::top_bar_ui;

use crate::{screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_ui);
    app.add_plugins((
        top_bar::plugin,
        shop::plugin,
        smart_text::plugin,
        actions::plugin,
        startup_bar::plugin,
        end_turn::plugin,
        game_over::plugin,
    ));
    // app.add_observer(on_shop_button_clicked);
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        widget::gameplay_ui_root("Gameplay Ui"),
        GlobalZIndex(2),
        StateScoped(Screen::Gameplay),
        children![top_bar_ui(), content()],
    ));
}

#[derive(Component)]
pub struct ContentPanel;

fn content() -> impl Bundle {
    (
        ContentPanel,
        Name::new("Content"),
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Pickable::IGNORE,
    )
}
