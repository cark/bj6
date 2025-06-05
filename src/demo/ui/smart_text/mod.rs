mod parsing;

use bevy::{
    asset::LoadedFolder,
    ecs::relationship::{RelatedSpawnerCommands, Relationship},
    platform::collections::HashMap,
    prelude::*,
};
use thiserror::*;

use crate::demo::ui::smart_text::parsing::ParseNode;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<NamedValues>();
    app.add_systems(Startup, load_all_assets);
    app.add_systems(PostUpdate, (replace_smart_text, update_named).chain());

    app.add_observer(on_update_named_values);
}

#[derive(Debug, Clone, Event, PartialEq, Eq, Hash)]
pub struct UpdateNamedValueEvent {
    pub name: String,
    pub value: String,
}

fn on_update_named_values(
    trigger: Trigger<UpdateNamedValueEvent>,
    mut named_values: ResMut<NamedValues>,
) {
    let ev = trigger.event();
    named_values.0.insert(ev.name.clone(), ev.value.clone());
}

#[derive(Resource, Debug, Default)]
#[allow(dead_code)]
pub struct AllAssets(Handle<LoadedFolder>);

#[derive(Resource, Debug, Default)]
pub struct Icons(HashMap<String, Handle<Image>>);

fn load_all_assets(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load_folder("");
    let items = [
        "coin", "lmb", "mmb", "poke", "rmb", "rotate", "round", "turn",
    ];
    let hm = items
        .iter()
        .map(|&n| (n.to_owned(), asset_server.load(format!("images/{}.png", n))))
        .collect();
    let icons = Icons(hm);
    cmd.insert_resource(icons);
    cmd.insert_resource(AllAssets(handle));
}

#[derive(Error, Debug)]
#[error("smart text parsing error")]
pub struct SmartTextError;

struct SmartTextConfig {
    font_size: f32,
    icons: HashMap<String, Handle<Image>>,
}

fn insert_smart_text(
    e: Entity,
    text: &str,
    config: SmartTextConfig,
    mut commands: Commands,
) -> Result<Entity, SmartTextError> {
    let parse_node = parsing::parse(text).ok_or(SmartTextError)?;
    let id = commands
        .entity(e)
        .with_children(move |cmd| {
            spawn_parse_node(cmd, parse_node, &config);
        })
        .id();
    Ok(id)
}

// #[derive(Component, Debug)]
// struct IconNode(String);

fn spawn_parse_node<R>(
    cmd: &mut RelatedSpawnerCommands<'_, R>,
    parse_node: ParseNode,
    config: &SmartTextConfig,
) where
    R: Relationship,
{
    // use Val::*;
    // const MARGIN_DIVISOR: f32 = 6.;
    match parse_node {
        ParseNode::Nodes(parse_nodes) => {
            for node in parse_nodes.into_iter() {
                spawn_parse_node(cmd, node, config);
            }
        }
        ParseNode::Text(str) => {
            cmd.spawn((
                Pickable::IGNORE,
                Text(str.to_owned()),
                TextFont::from_font_size(config.font_size),
                Node {
                    // margin: UiRect::left(Px(config.font_size / MARGIN_DIVISOR))
                    //     .with_right(Px(config.font_size / MARGIN_DIVISOR)),
                    ..default()
                },
            ));
        }
        ParseNode::Icon(name) => {
            cmd.spawn((
                Pickable::IGNORE,
                Node {
                    width: Val::Px(config.font_size * 1.2),
                    height: Val::Px(config.font_size * 1.2),
                    // margin: UiRect::left(Px(config.font_size / MARGIN_DIVISOR))
                    //     .with_right(Px(config.font_size / MARGIN_DIVISOR)),
                    ..default()
                },
                // IconNode(name.to_owned()),
                ImageNode {
                    image: config.icons[name].clone(),
                    ..default()
                },
            ));
        }
        ParseNode::Named(name) => {
            cmd.spawn((
                Pickable::IGNORE,
                Name::new(name.to_owned()),
                Text("".to_string()),
                TextFont::from_font_size(config.font_size),
                Node {
                    // margin: UiRect::left(Px(config.font_size / MARGIN_DIVISOR))
                    //     .with_right(Px(config.font_size / MARGIN_DIVISOR)),
                    ..default()
                },
            ));
        }
        ParseNode::Space => {
            cmd.spawn((
                Node::DEFAULT,
                Text(" ".to_string()),
                TextFont::from_font_size(config.font_size),
            ));
        }
    }
}

// fn replace_icon_nodes(mut cmd: Commands, nodes: Query<(Entity, &IconNode)>, icons: Res<Icons>) {
//     for (e, IconNode(name)) in &nodes {
//         cmd.entity(e).remove::<IconNode>().insert(ImageNode {
//             image: icons.0[name].clone(),
//             ..default()
//         });
//     }
// }

#[derive(Component, Debug)]
pub struct SmartText {
    pub text: String,
    pub font_size: f32,
}

impl Default for SmartText {
    fn default() -> Self {
        SmartText {
            text: "empty".to_string(),
            font_size: 14.,
        }
    }
}

fn replace_smart_text(
    mut cmd: Commands,
    smart_texts: Query<(Entity, &SmartText)>,
    icons: Res<Icons>,
) {
    for (e, st) in smart_texts {
        let config = SmartTextConfig {
            font_size: st.font_size,
            icons: icons.0.clone(),
        };
        cmd.entity(e).remove::<SmartText>();
        let _ = insert_smart_text(e, &st.text, config, cmd.reborrow());
    }
}

#[derive(Resource, Debug, Default)]
struct NamedValues(HashMap<String, String>);

fn update_named(named_values: Res<NamedValues>, mut q_named: Query<(&mut Text, &Name)>) {
    for (mut text, name) in &mut q_named {
        if let Some(value) = named_values.0.get(name.as_str()) {
            if text.0 != value.as_str() {
                text.0 = value.clone();
            }
        }
    }
}
