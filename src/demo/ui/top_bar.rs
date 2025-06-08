use bevy::prelude::*;

use crate::{
    demo::{
        GameplayState,
        ui::smart_text::{SmartText, UpdateNamedValueEvent},
    },
    model::game::Game,
    screens::Screen,
    theme::widget::{self, ButtonClick, set_enabled},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameplayState::Placement), enable_shop_button);
    app.add_systems(OnExit(GameplayState::Placement), disable_shop_button);
    app.add_systems(Update, update_top_bar.run_if(in_state(Screen::Gameplay)));
    app.add_observer(on_update_top_bar);
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateTopBarEvent;

fn on_update_top_bar(_: Trigger<UpdateTopBarEvent>, mut commands: Commands, game: Res<Game>) {
    // warn!("*************** update topbar");
    commands.trigger(UpdateNamedValueEvent {
        name: "current_gold".to_string(),
        value: game.gold().to_string(),
    });

    commands.trigger(UpdateNamedValueEvent {
        name: "required_gold".to_string(),
        value: game.required_gold().to_string(),
    });
    commands.trigger(UpdateNamedValueEvent {
        name: "turns_left".to_string(),
        value: game.turns_left().to_string(),
    });

    commands.trigger(UpdateNamedValueEvent {
        name: "round".to_string(),
        value: game.round().to_string(),
    });
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

pub(super) fn top_bar_ui() -> impl Bundle {
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
        children![gold_ui(), shop_button_part_ui(), turns_left_ui()],
    )
}

#[derive(Component)]
struct CurrentGoldText;

#[derive(Component)]
struct RequiredGoldText;

const TEXT_SIZE: f32 = 25.;

pub(super) fn gold_ui() -> impl Bundle {
    (
        Name::new("Gold Part"),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        SmartText {
            font_size: TEXT_SIZE,
            text: "{named:current_gold}{icon:coin} / {named:required_gold}{icon:coin}".to_string(),
        },
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

pub(super) fn turns_left_ui() -> impl Bundle {
    (
        Name::new("Turns left part"),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        SmartText {
            font_size: TEXT_SIZE,
            text: "{named:turns_left}{icon:turn} left on {icon:round}{named:round}".to_string(),
        },
    )
}
