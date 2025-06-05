use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    demo::{GameplayState, ui::smart_text::SmartText},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(Screen::Gameplay), bleh)
    let mut actions = HashMap::new();
    for action in ALL_ACTIONS {
        actions.insert(action.to_string(), false);
    }
    app.insert_resource(ActiveActions(actions));
    app.add_observer(on_set_active_action);

    app.add_systems(OnEnter(GameplayState::Placement), enter);
    app.add_systems(OnExit(GameplayState::Placement), exit);
    app.add_systems(
        Update,
        update_active_actions.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetActiveActionEvent(pub String, pub bool);

#[derive(Resource, Debug, Default)]
struct ActiveActions(HashMap<String, bool>);

const ALL_ACTIONS: [&str; 8] = [
    "mmb_pan",
    "lmb_mmb_pan",
    "mmb_zoom",
    "lmb_drag",
    "lmb_drop",
    "rmb_cancel_drag",
    "r_rotate",
    "start_turn",
];

fn on_set_active_action(
    trigger: Trigger<SetActiveActionEvent>,
    mut active_actions: ResMut<ActiveActions>,
) {
    let ev = trigger.event();
    if let Some(value) = active_actions.0.get_mut(&ev.0) {
        *value = ev.1;
    }
}

fn update_active_actions(
    active_actions: Res<ActiveActions>,
    mut q_nodes: Query<(&Name, &mut Node)>,
) {
    for (name, mut node) in &mut q_nodes {
        if let Some(active) = active_actions.0.get(name.as_str()) {
            if *active {
                if node.display == Display::None {
                    node.display = Display::Flex;
                }
            } else if node.display != Display::None {
                node.display = Display::None;
            }
        }
    }
}

fn enter(mut commands: Commands) {
    let font_size: f32 = 18.;
    commands.spawn((
        Name::new("commands"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            width: Val::Percent(100.0),
            // height: Val::Percent(20.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.)),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        children![
            (
                Name::new("mmb_pan"),
                Node {
                    margin: UiRect::axes(Val::Px(20.), Val::Px(5.)),
                    ..default()
                },
                SmartText {
                    font_size,
                    text: "{icon:mmb} Pan".to_string()
                }
            ),
            (
                Name::new("lmb_mmb_pan"),
                Node {
                    margin: UiRect::axes(Val::Px(20.), Val::Px(5.)),
                    ..default()
                },
                SmartText {
                    font_size,
                    text: "{icon:lmb}{icon:mmb} Pan".to_string()
                }
            ),
            (
                Name::new("mmb_zoom"),
                Node {
                    margin: UiRect::axes(Val::Px(20.), Val::Px(5.)),
                    ..default()
                },
                SmartText {
                    font_size,
                    text: "{icon:mmb} Zoom".to_string()
                }
            ),
            (
                Name::new("lmb_drag"),
                Node {
                    margin: UiRect::axes(Val::Px(20.), Val::Px(5.)),
                    ..default()
                },
                SmartText {
                    font_size,
                    text: "{icon:lmb} Drag".to_string()
                }
            ),
            (
                Name::new("lmb_drop"),
                Node {
                    margin: UiRect::axes(Val::Px(20.), Val::Px(5.)),
                    ..default()
                },
                SmartText {
                    font_size,
                    text: "{icon:lmb} Drop".to_string()
                }
            ),
            (
                Name::new("rmb_cancel_drag"),
                Node {
                    margin: UiRect::axes(Val::Px(20.), Val::Px(5.)),
                    ..default()
                },
                SmartText {
                    font_size,
                    text: "{icon:rmb} Cancel drag".to_string()
                }
            ),
            (
                Name::new("r_rotate"),
                Node {
                    margin: UiRect::axes(Val::Px(20.), Val::Px(5.)),
                    ..default()
                },
                SmartText {
                    font_size,
                    text: "{icon:rotate} Rotate".to_string()
                }
            ),
            (
                Name::new("start_turn"),
                Node {
                    margin: UiRect::axes(Val::Px(20.), Val::Px(5.)),
                    ..default()
                },
                SmartText {
                    font_size,
                    text: "{icon:turn}{icon:lmb} Start turn".to_string()
                }
            )
        ],
    ));
}

fn exit(mut cmd: Commands) {
    for action in ALL_ACTIONS {
        cmd.trigger(SetActiveActionEvent(action.to_string(), false));
    }
}
