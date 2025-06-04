use bevy::prelude::*;

use crate::{
    demo::{GameplayState, level::LevelAssets},
    model::game::Game,
    screens::Screen,
    theme::{
        palette::HEADER_TEXT,
        widget::{self, ButtonClick, set_enabled},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameplayState::Placement), enable_shop_button);
    app.add_systems(OnExit(GameplayState::Placement), disable_shop_button);
    app.add_systems(Update, update_top_bar.run_if(in_state(Screen::Gameplay)));
    app.add_observer(on_update_top_bar);
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UpdateTopBarEvent;

fn on_update_top_bar(
    _: Trigger<UpdateTopBarEvent>,
    game: Res<Game>,
    current_gold_text: Single<
        &mut Text,
        (
            With<CurrentGoldText>,
            Without<RequiredGoldText>,
            Without<TurnsLeftText>,
            Without<RoundText>,
        ),
    >,
    required_gold_text: Single<
        &mut Text,
        (
            With<RequiredGoldText>,
            Without<CurrentGoldText>,
            Without<TurnsLeftText>,
            Without<RoundText>,
        ),
    >,
    turns_left_text: Single<
        &mut Text,
        (
            With<TurnsLeftText>,
            Without<RoundText>,
            Without<CurrentGoldText>,
            Without<RequiredGoldText>,
        ),
    >,
    round_text: Single<
        &mut Text,
        (
            With<RoundText>,
            Without<TurnsLeftText>,
            Without<CurrentGoldText>,
            Without<RequiredGoldText>,
        ),
    >,
) {
    current_gold_text.into_inner().0 = game.gold.to_string();
    required_gold_text.into_inner().0 = game.required_gold.to_string();
    turns_left_text.into_inner().0 = game.turns_left.to_string();
    round_text.into_inner().0 = game.round.to_string();
}

fn update_top_bar(mut commands: Commands) {
    commands.trigger(UpdateTopBarEvent);
}

fn enable_shop_button(mut commands: Commands) {
    set_enabled::<ShopButton>(&mut commands, true);
}

fn disable_shop_button(mut commands: Commands) {
    set_enabled::<ShopButton>(&mut commands, false);
}

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
struct RequiredGoldText;

const ICON_SIZE: f32 = 40.;
const TEXT_SIZE: f32 = 25.;

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
                TextFont::from_font_size(TEXT_SIZE),
                TextColor(HEADER_TEXT)
            ),
            (
                Text("/".into()),
                TextFont::from_font_size(TEXT_SIZE),
                TextColor(HEADER_TEXT)
            ),
            (
                RequiredGoldText,
                Text("1256".into()),
                TextFont::from_font_size(TEXT_SIZE),
                TextColor(HEADER_TEXT)
            ),
            (
                ImageNode {
                    image: assets.coin.clone(),
                    // image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                Node {
                    width: Val::Px(ICON_SIZE),
                    height: Val::Px(ICON_SIZE),
                    ..default()
                }
            ),
        ],
    )
}

#[derive(Component)]
struct ShopButtonPart;
#[derive(Component)]
pub struct ShopButton;

fn shop_button_part_ui() -> impl Bundle {
    (
        ShopButtonPart,
        Visibility::default(),
        Name::new("Shop Button Part"),
        Node {
            flex_grow: 1.,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            widget::button_small("Shop", on_shop_button_clicked),
            ShopButton
        )],
    )
}

fn on_shop_button_clicked(
    _: Trigger<ButtonClick>,

    mut next_state: ResMut<NextState<GameplayState>>,
) {
    next_state.set(GameplayState::Shop);
}

#[derive(Component)]
struct TurnsLeftText;

#[derive(Component)]
struct RoundText;

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
                TextFont::from_font_size(TEXT_SIZE),
                TextColor(HEADER_TEXT)
            ),
            (
                ImageNode {
                    image: assets.turn.clone(),
                    ..default()
                },
                Node {
                    width: Val::Px(ICON_SIZE),
                    height: Val::Px(ICON_SIZE),
                    ..default()
                }
            ),
            (
                Text("left on".into()),
                TextFont::from_font_size(TEXT_SIZE),
                TextColor(HEADER_TEXT)
            ),
            (
                ImageNode {
                    image: assets.round.clone(),
                    ..default()
                },
                Node {
                    width: Val::Px(ICON_SIZE),
                    height: Val::Px(ICON_SIZE),
                    ..default()
                }
            ),
            (
                RoundText,
                Text("1".into()),
                TextFont::from_font_size(TEXT_SIZE),
                TextColor(HEADER_TEXT)
            ),
        ],
    )
}
