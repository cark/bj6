use bevy::prelude::{Val::*, *};

use crate::{
    demo::{GameplayState, level::LevelAssets},
    model::{
        actor_type::{self, ActorType},
        shop::Shop,
    },
    theme::{
        interaction::SetButtonSelectedEvent,
        palette::HEADER_TEXT,
        widget::{self, ButtonClick, content_button},
    },
};

use super::ContentPanel;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameplayState::Shop), spawn_shop);
    app.add_systems(
        Update,
        items_panel_added.run_if(component_added::<ShopItemsPanel>),
    );
    app.add_observer(on_popupalate_shop_items);
}

pub fn component_added<T: Component>(query: Query<(), Added<T>>) -> bool {
    !query.is_empty()
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PopulateShopItemsEvent;

fn on_popupalate_shop_items(
    trigger: Trigger<PopulateShopItemsEvent>,
    mut commands: Commands,
    shop: Res<Shop>,
    actor_types: Res<actor_type::ActorTypes>,
    assets: Res<LevelAssets>,
) {
    let panel_entity = trigger.target();
    commands.entity(panel_entity).with_children(|commands| {
        for name in shop.items.iter() {
            let actor_type = actor_types.get(name).unwrap();
            commands.spawn(shop_item(actor_type, &assets));
        }
    });
}

const ITEM_ICON_SIZE: f32 = 45.;

#[derive(Debug, Clone, Component)]
struct ShopItem;

fn shop_item(actor_type: &ActorType, assets: &LevelAssets) -> impl Bundle {
    (
        // Name::new("Shop item"),
        ShopItem,
        actor_type.clone(),
        content_button(
            shop_item_button_content(actor_type, assets),
            shop_item_clicked,
        ),
        // BorderColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
    )
}

fn shop_item_clicked(
    trigger: Trigger<ButtonClick>,
    mut commands: Commands,
    child_of: Query<&ChildOf>,
    // q_parent: Query<&Name>,
    shop_items: Query<Entity, With<ShopItem>>,
) {
    let target = trigger.target();
    let parent = child_of.get(target).unwrap().parent();
    // let parent_name = q_parent.get(parent).unwrap();
    // warn!("shop_itme_clicked {}", parent_name);

    for item in &shop_items {
        // warn!("bleh");
        if item == parent {
            commands.trigger_targets(SetButtonSelectedEvent(true), item);
        } else {
            commands.trigger_targets(SetButtonSelectedEvent(false), item);
        }
    }
}

fn shop_item_button_content(actor_type: &ActorType, assets: &LevelAssets) -> impl Bundle {
    let font_size = 14.;
    (
        // Name::new("Shop item"),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            flex_grow: 1.0,
            row_gap: Px(5.0),
            ..default()
        },
        children![
            (
                Name::new("Actor type image"),
                ImageNode {
                    image: actor_type.sprite_handle.as_ref().unwrap().clone(),
                    ..default()
                },
                Node {
                    width: Val::Px(ITEM_ICON_SIZE),
                    height: Val::Px(ITEM_ICON_SIZE),
                    ..default()
                },
                Pickable::IGNORE,
            ),
            (
                Name::new("Actor type cost"),
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Pickable::IGNORE,
                children![
                    (
                        Pickable::IGNORE,
                        Text(format!("{} ", actor_type.cost)),
                        TextFont::from_font_size(font_size)
                    ),
                    (
                        Pickable::IGNORE,
                        ImageNode {
                            image: assets.coin.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(font_size),
                            height: Val::Px(font_size),
                            ..default()
                        }
                    ),
                ],
            ),
        ],
    )
}

fn items_panel_added(
    mut commands: Commands,
    shop_items_panel: Single<Entity, With<ShopItemsPanel>>,
) {
    commands.trigger_targets(PopulateShopItemsEvent, shop_items_panel.into_inner());
}

fn spawn_shop(
    mut commands: Commands,
    content_panel: Single<Entity, With<ContentPanel>>,
    assets: Res<LevelAssets>,
) {
    let content_panel = content_panel.into_inner();
    commands.entity(content_panel).with_children(|commands| {
        commands.spawn(shop_window(&assets));
    });
}

const TITLE_TEXT_SIZE: f32 = 30.;

fn shop_window(assets: &LevelAssets) -> impl Bundle {
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
        StateScoped(GameplayState::Shop),
        children![title_bar(), content(assets), buttons()],
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

fn content(assets: &LevelAssets) -> impl Bundle {
    (
        Node {
            flex_grow: 1.0,
            width: Percent(100.),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::FlexStart,
            ..default()
        },
        children![items_panel(assets)],
    )
}

#[derive(Component)]
pub struct ShopItemsPanel;

fn items_panel(assets: &LevelAssets) -> impl Bundle {
    (
        Node {
            height: Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            row_gap: Px(10.),
            ..default()
        },
        children![
            (
                ShopItemsPanel,
                Node {
                    width: Px(250.),
                    height: Percent(100.),
                    // flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Px(10.),
                    ..default()
                },
                BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
            ),
            //(widget::button_small("Restock", on_restock_button_clicked),)
            (
                RestockButton,
                widget::content_button(
                    restock_button_content(1, HEADER_TEXT, assets),
                    on_restock_button_clicked
                ),
            )
        ],
    )
}

#[derive(Component)]
pub struct RestockCostText;

#[derive(Component)]
pub struct RestockButton;

fn restock_button_content(gold: u32, gold_color: Color, assets: &LevelAssets) -> impl Bundle {
    let font_size = 14.;
    (
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        children![
            (
                Pickable::IGNORE,
                Text("Restock: ".to_string()),
                TextFont::from_font_size(font_size),
                TextColor(HEADER_TEXT),
            ),
            (
                Pickable::IGNORE,
                RestockCostText,
                Text(format!("{gold}").to_string()),
                TextFont::from_font_size(font_size),
                TextColor(gold_color),
            ),
            (
                Pickable::IGNORE,
                ImageNode {
                    image: assets.coin.clone(),
                    ..default()
                },
                Node {
                    width: Val::Px(font_size),
                    height: Val::Px(font_size),
                    ..default()
                }
            ),
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
