use bevy::prelude::*;

use crate::{
    demo::{
        GameplayState,
        ui::smart_text::{SmartText, UpdateNamedValueEvent},
    },
    model::game::Game,
    screens::Screen,
    theme::widget::{self, center_ui_root},
};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(GameplayState::Placement), right_to_game_over);

    app.add_observer(on_game_over);
}

// fn right_to_game_over(mut commands: Commands) {
//     commands.trigger(GameOverEvent);
// }

#[derive(Debug, Clone, Copy, Event)]
pub struct GameOverEvent;

fn on_game_over(
    _: Trigger<GameOverEvent>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameplayState>>,
    game: Res<Game>,
) {
    next_state.set(GameplayState::GameOver);

    commands.spawn((
        center_ui_root("End turn Ui"),
        GlobalZIndex(6),
        StateScoped(GameplayState::GameOver),
        children![window()],
    ));
    commands.trigger(UpdateNamedValueEvent {
        name: "total_gold".to_string(),
        value: game.total_gold().to_string(),
    });
    commands.trigger(UpdateNamedValueEvent {
        name: "round".to_string(),
        value: game.total_gold().to_string(),
    });
}

fn window() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Px(10.)),
            row_gap: Val::Px(10.0),
            // justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
        children![title(), content(), buttons()],
    )
}

fn title() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,

            align_items: AlignItems::Center,
            justify_content: JustifyContent::Stretch,
            width: Val::Percent(100.),
            // flex_grow: 1.,
            // margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
        children![(
            Node {
                flex_grow: 1.,
                margin: UiRect::axes(Val::Px(20.), Val::Px(10.0)),
                // width: Val::Px(100.0),
                ..default()
            },
            SmartText::new("Game Over.", 30.)
        )],
    )
}

const COMMENTS: [&str; 5] = [
    "Sucks to be you !",
    "Better luck next time !",
    "Turing will take over now...",
    "Get with the program !",
    "I'm so disapointed...",
];

fn content() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            // margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
        children![comment(), game_stats()],
    )
}

fn comment() -> impl Bundle {
    (
        Node {
            margin: UiRect::axes(Val::Px(20.), Val::Px(10.0)),
            ..default()
        },
        SmartText::new(COMMENTS[rand::random::<usize>() % COMMENTS.len()], 20.),
    )
}

fn game_stats() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        // BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
        children![(
            Node {
                margin: UiRect::axes(Val::Px(20.), Val::Px(10.0)),
                ..default()
            },
            SmartText::new(
                "But don't worry ! You made {named:total_gold}{icon:coin} and reached round {named:round}{icon:round}.",
                20.
            )
        )],
    )
}

fn buttons() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(20.),
            // margin: UiRect::bottom(Val::Px(10.)),
            ..default()
        },
        children![(widget::button_small("Sadness", on_sadness_button_clicked),),],
    )
}

fn on_sadness_button_clicked(
    _: Trigger<widget::ButtonClick>,
    mut next_screen_state: ResMut<NextState<Screen>>,
    mut next_gameplay_state: ResMut<NextState<GameplayState>>,
) {
    next_gameplay_state.set(GameplayState::WorkaroundBugs);
    next_screen_state.set(Screen::Title);
}
