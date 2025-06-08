use bevy::prelude::*;

// use crate::{demo::turn::TurnState, theme::widget};

pub(super) fn plugin(_app: &mut App) {
    //hoy
}

// fn spwn_end_turn_ui(mut commands: Commands) {
//     commands.spawn((
//         widget::center_ui_root("End turn Ui"),
//         GlobalZIndex(6),
//         StateScoped(TurnState::EndTurn),
//         children![window()],
//     ));
// }

// fn window() -> impl Bundle {
//     (
//         Node {
//             flex_direction: FlexDirection::Column,
//             align_items: AlignItems::Center,
//             justify_content: JustifyContent::Center,
//             ..default()
//         },
//         BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
//     )
// }
