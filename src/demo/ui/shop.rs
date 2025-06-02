use bevy::prelude::{Val::*, *};

use crate::{
    demo::GameplayState,
    theme::{
        palette::HEADER_TEXT,
        widget::{self, ButtonClick},
    },
};

use super::ContentPanel;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameplayState::Shop), spawn_shop);
}

fn spawn_shop(mut commands: Commands, content_panel: Single<Entity, With<ContentPanel>>) {
    let content_panel = content_panel.into_inner();
    commands.entity(content_panel).with_children(|commands| {
        // commands.spawn((
        //     widget::center_ui_root("Gameplay Ui"),
        //     GlobalZIndex(3),
        //     StateScoped(GameplayState::Shop),
        //     children![shop_window()],
        // ));
        commands.spawn(shop_window());
    });
}

const TITLE_TEXT_SIZE: f32 = 30.;

fn shop_window() -> impl Bundle {
    (
        Name::new("Shop Window"),
        Node {
            // width: Auto,
            // height: Auto,
            margin: UiRect::new(Px(20.0), Px(20.0), Px(0.0), Px(20.0)),
            padding: UiRect::all(Px(20.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            flex_grow: 1.0,
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
        children![title_bar(), content(), buttons()],
    )
}

fn title_bar() -> impl Bundle {
    (
        TextFont::from_font_size(TITLE_TEXT_SIZE),
        Node {
            width: Percent(100.),
            padding: UiRect::all(Px(7.0)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
        children![(
            Text("Shop".into()),
            Node::default(),
            TextColor(HEADER_TEXT),
            TextFont::from_font_size(TITLE_TEXT_SIZE),
        )],
    )
}

fn content() -> impl Bundle {
    (
        Node {
            flex_grow: 1.0,
            width: Percent(100.),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::FlexStart,
            ..default()
        },
        children![(items_panel())],
    )
}

fn items_panel() -> impl Bundle {
    (
        Node {
            height: Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            row_gap: Px(20.),
            ..default()
        },
        children![
            (
                Node {
                    width: Px(250.),
                    height: Percent(100.),
                    // flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
            ),
            (widget::button_small("Restock", on_restock_button_clicked),)
        ],
    )
}

fn on_restock_button_clicked(_: Trigger<ButtonClick>) {
    warn!("restock !");
}

fn buttons() -> impl Bundle {
    (
        Node {
            width: Percent(100.),
            justify_content: JustifyContent::End,
            column_gap: Px(20.),
            ..default()
        },
        children![
            widget::button_small("Buy", on_buy_button_clicked),
            widget::button_small("Close", on_close_shop_button_clicked)
        ],
    )
}

fn on_close_shop_button_clicked(_: Trigger<Pointer<Click>>) {
    warn!("close !");
}

fn on_buy_button_clicked(_: Trigger<Pointer<Click>>) {
    warn!("buy !");
}
