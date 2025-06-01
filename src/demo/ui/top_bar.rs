use bevy::prelude::*;

use crate::{
    demo::level::LevelAssets,
    screens::Screen,
    theme::{palette::HEADER_TEXT, widget},
};

pub(super) fn top_bar_ui(assets: &LevelAssets) -> impl Bundle {
    (
        Name::new("Top Bar"),
        Node {
            min_height: Val::Px(80.0),
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            padding: UiRect {
                left: Val::Px(20.),
                right: Val::Px(20.),
                top: Val::Auto,
                bottom: Val::Auto,
            },
            ..default()
        },
        Pickable::IGNORE,
        children![
            gold_ui(assets),
            shop_button_part_ui(),
            turns_left_ui(assets)
        ],
    )
}

#[derive(Component)]
struct CurrentGoldText;

#[derive(Component)]
struct MissingGoldText;

pub(super) fn gold_ui(assets: &LevelAssets) -> impl Bundle {
    (
        Name::new("Gold Part"),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        },
        children![
            (
                CurrentGoldText,
                Text("1256".into()),
                TextFont::from_font_size(40.0),
                TextColor(HEADER_TEXT)
            ),
            (
                Text("/".into()),
                TextFont::from_font_size(40.0),
                TextColor(HEADER_TEXT)
            ),
            (
                MissingGoldText,
                Text("1256".into()),
                TextFont::from_font_size(40.0),
                TextColor(HEADER_TEXT)
            ),
            (
                ImageNode {
                    image: assets.coin.clone(),
                    // image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    ..default()
                }
            ),
        ],
    )
}

fn shop_button_part_ui() -> impl Bundle {
    (
        Node {
            flex_grow: 1.,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        children![widget::button_small("Shop", on_shop_button_clicked)],
    )
}

pub fn on_shop_button_clicked(
    _: Trigger<Pointer<Click>>,
    mut _next_screen: ResMut<NextState<Screen>>,
) {
    warn!("shop button click")
}

#[derive(Component)]
struct TurnsLeftText;

pub(super) fn turns_left_ui(assets: &LevelAssets) -> impl Bundle {
    (
        Name::new("Turns left part"),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        },
        children![
            (
                TurnsLeftText,
                Text("5".into()),
                TextFont::from_font_size(40.0),
                TextColor(HEADER_TEXT)
            ),
            (
                ImageNode {
                    image: assets.turn.clone(),
                    // image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    ..default()
                }
            ),
            (
                TurnsLeftText,
                Text("left".into()),
                TextFont::from_font_size(40.0),
                TextColor(HEADER_TEXT)
            ),
        ],
    )
}
