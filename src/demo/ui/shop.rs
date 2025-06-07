use bevy::prelude::{Val::*, *};

use crate::{
    demo::{
        GameplayState,
        drag::{DragSource, StartDragEvent},
        ui::smart_text::{SmartText, UpdateNamedValueEvent},
    },
    model::{
        actor_type::{ActorType, ActorTypeId},
        game::Game,
    },
    theme::{
        interaction::SetButtonSelectedEvent,
        palette::HEADER_TEXT,
        widget::{self, ButtonClick, content_button, set_enabled},
    },
};

use super::ContentPanel;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SelectedActorType>();
    app.add_systems(OnEnter(GameplayState::Shop), (spawn_shop, enter).chain());
    app.add_systems(OnExit(GameplayState::Shop), exit);
    app.add_systems(
        Update,
        items_panel_added.run_if(component_added::<ShopItemsPanel>),
    );
    // app.add_systems(Update, systems)
    app.add_observer(on_populate_shop_items);
    app.add_observer(on_update_buy_button);
    app.add_observer(on_update_restock_button);
}

pub fn component_added<T: Component>(query: Query<(), Added<T>>) -> bool {
    !query.is_empty()
}

fn enter(mut commands: Commands) {
    commands.trigger(UpdateBuyButtonEvent);
    commands.trigger(UpdateRestockButtonEvent);
}

fn exit(mut selected_actor_type: ResMut<SelectedActorType>) {
    selected_actor_type.0 = None;
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UpdateBuyButtonEvent;

fn on_update_buy_button(
    _: Trigger<UpdateBuyButtonEvent>,
    selected_actor_type: Res<SelectedActorType>,
    mut commands: Commands,
    game: Res<Game>,
) {
    if let Some(ActorSelection { actor_type_id, .. }) = selected_actor_type.0.as_ref() {
        let actor_type = game
            .actor_types()
            .get(actor_type_id)
            .expect("actor_type_id should be good!");
        if actor_type.cost as u64 > game.gold() {
            set_enabled::<BuyButton>(&mut commands, false);
        } else {
            set_enabled::<BuyButton>(&mut commands, true);
        }
    } else {
        set_enabled::<BuyButton>(&mut commands, false);
    }
}

#[derive(Resource, Default, Clone, Debug)]
struct SelectedActorType(Option<ActorSelection>);

#[derive(Clone, Debug)]
struct ActorSelection {
    actor_type_id: ActorTypeId,
    shop_index: usize,
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PopulateShopItemsEvent;

fn on_populate_shop_items(
    trigger: Trigger<PopulateShopItemsEvent>,
    mut commands: Commands,
    game: Res<Game>,
    mut q_hideable: Query<&mut Node, With<InfoHideable>>,
) {
    let panel_entity = trigger.target();
    commands.entity(panel_entity).despawn_related::<Children>();
    commands.entity(panel_entity).with_children(|commands| {
        for actor_type_id in game.shop().stock() {
            let actor_type = game.actor_types().get(actor_type_id).unwrap();
            commands.spawn(shop_item(actor_type, actor_type_id.clone()));
        }
    });
    for mut node in q_hideable.iter_mut() {
        node.display = Display::None;
    }
}

const ITEM_ICON_SIZE: f32 = 45.;

#[derive(Debug, Clone, Component)]
struct ShopItem;

fn shop_item(actor_type: &ActorType, actor_type_id: ActorTypeId) -> impl Bundle {
    (
        ShopItem,
        actor_type_id,
        actor_type.clone(),
        content_button(shop_item_button_content(actor_type), on_shop_item_clicked),
    )
}

fn on_shop_item_clicked(
    trigger: Trigger<ButtonClick>,
    mut commands: Commands,
    child_of: Query<&ChildOf>,
    shop_items: Query<(Entity, &ActorTypeId), With<ShopItem>>,
    mut selected_actor_type: ResMut<SelectedActorType>,
    mut q_hideable: Query<&mut Node, With<InfoHideable>>,
    game: Res<Game>,
) {
    let target = trigger.target();
    let parent = child_of.get(target).unwrap().parent();

    for (item, actor_type_id) in shop_items.iter() {
        if item == parent {
            commands.trigger_targets(SetButtonSelectedEvent(true), item);
            selected_actor_type.0 = Some(ActorSelection {
                actor_type_id: actor_type_id.clone(),
                shop_index: game
                    .shop()
                    .stock()
                    .position(|a| a == actor_type_id)
                    .unwrap(),
            });
            if let Some(actor_type) = game.actor_types().get(actor_type_id) {
                commands.trigger(UpdateNamedValueEvent {
                    name: "actor_type_name".to_string(),
                    value: actor_type.name.clone(),
                });
                commands.trigger(UpdateNamedValueEvent {
                    name: "max_activations".to_string(),
                    value: actor_type.max_activations.to_string(),
                });
                commands.trigger(UpdateNamedValueEvent {
                    name: "prize".to_string(),
                    value: actor_type.prize.to_string(),
                });
                commands.trigger(UpdateNamedValueEvent {
                    name: "actor_description".to_string(),
                    value: actor_type.description.clone(),
                });
                for mut node in q_hideable.iter_mut() {
                    node.display = Display::Flex;
                }
            }
        } else {
            commands.trigger_targets(SetButtonSelectedEvent(false), item);
        }
    }
    commands.trigger(UpdateBuyButtonEvent);
}

fn shop_item_button_content(actor_type: &ActorType) -> impl Bundle {
    let font_size = 12.;
    (
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
                SmartText {
                    text: format!("{}{}", actor_type.cost, "{icon:coin}"),
                    font_size,
                },
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

fn spawn_shop(mut commands: Commands, content_panel: Single<Entity, With<ContentPanel>>) {
    let content_panel = content_panel.into_inner();
    commands.entity(content_panel).with_children(|commands| {
        commands.spawn(shop_window());
    });
}

const TITLE_TEXT_SIZE: f32 = 30.;

fn shop_window() -> impl Bundle {
    (
        Name::new("Shop Window"),
        Node {
            // width: Auto,
            height: Percent(100.),
            margin: UiRect::new(Px(20.0), Px(20.0), Px(0.0), Px(20.0)),
            padding: UiRect::all(Px(20.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            // flex_grow: 1.0,
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(GameplayState::Shop),
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
            column_gap: Px(20.),
            ..default()
        },
        children![items_panel(), info_panel()],
    )
}

#[derive(Component)]
pub struct ShopItemsPanel;

fn items_panel() -> impl Bundle {
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
            (
                RestockButton,
                widget::content_button(restock_button_content(), on_restock_button_clicked),
            )
        ],
    )
}

#[derive(Debug, Component)]
struct InfoHideable;

fn info_panel() -> impl Bundle {
    (
        Node {
            height: Percent(100.),
            width: Px(400.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            ..default()
        },
        children![
            (
                InfoHideable,
                Node {
                    display: Display::None,
                    margin: UiRect::new(Px(0.), Px(0.), Px(0.), Px(5.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
                children![(
                    Node {
                        min_height: Px(30.),
                        ..default()
                    },
                    SmartText::new("{named:actor_type_name}", 24.),
                )]
            ),
            info_numbers(),
            info_text(),
        ],
    )
}

fn info_text() -> impl Bundle {
    (
        InfoHideable,
        Node {
            display: Display::None,
            margin: UiRect::new(Px(0.), Px(0.), Px(0.), Px(5.)),
            padding: UiRect::all(Px(10.)),
            flex_grow: 1.0,
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
        children![(
            Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Start,
                //align_items: AlignItems::Center,
                ..default()
            },
            // Text(
            //     "bonjour je regarde comment faire avec des tres long texts. J'espere que ca va wrap !".to_string()
            // ),
            SmartText::new("{named:actor_description}", 18.),
        )],
    )
}

fn info_numbers() -> impl Bundle {
    (
        InfoHideable,
        Node {
            display: Display::None,
            width: Percent(100.),
            min_height: Px(30.),
            margin: UiRect::vertical(Px(5.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Stretch,
            flex_direction: FlexDirection::Row,
            column_gap: Px(10.),
            ..default()
        },
        children![
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_grow: 1.0,
                    padding: UiRect::all(Px(5.)),
                    ..default()
                },
                BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
                children![(
                    Node::DEFAULT,
                    SmartText::new("{named:max_activations} {icon:activation}/{icon:turn}", 14.),
                )]
            ),
            (
                Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Px(5.)),
                    ..default()
                },
                BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
                children![(
                    Node::DEFAULT,
                    SmartText::new("{named:prize} {icon:coin}/{icon:activation}", 14.),
                )]
            )
        ],
    )
}

#[derive(Component)]
pub struct RestockCostText;

#[derive(Component)]
pub struct RestockButton;

fn restock_button_content() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        SmartText {
            text: "Restock: {named:restock_cost}{icon:coin}".to_string(), //"coucou mamma brains".to_string(),
            font_size: 14.,
        },
    )
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UpdateRestockButtonEvent;

fn on_update_restock_button(
    _: Trigger<UpdateRestockButtonEvent>,
    mut commands: Commands,
    game: Res<Game>,
) {
    commands.trigger(UpdateNamedValueEvent {
        name: "restock_cost".to_string(),
        value: format!("{}", game.shop().restock_cost()),
    });
    if game.can_restock() {
        set_enabled::<RestockButton>(&mut commands, true);
    } else {
        set_enabled::<RestockButton>(&mut commands, false);
    }
}

fn on_restock_button_clicked(
    _: Trigger<ButtonClick>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    shop_items_panel: Single<Entity, With<ShopItemsPanel>>,
    mut selected_actor_type: ResMut<SelectedActorType>,
) {
    game.restock();
    selected_actor_type.0 = None;
    commands.trigger(UpdateRestockButtonEvent);
    commands.trigger(UpdateBuyButtonEvent);
    commands.trigger_targets(PopulateShopItemsEvent, shop_items_panel.into_inner());
}

#[derive(Component)]
struct BuyButton;
#[derive(Component)]
struct CloseButton;

fn buttons() -> impl Bundle {
    (
        Node {
            width: Percent(100.),
            justify_content: JustifyContent::End,
            column_gap: Px(20.),
            ..default()
        },
        children![
            (
                BuyButton,
                widget::button_small("Buy", on_buy_button_clicked),
            ),
            (
                CloseButton,
                widget::button_small("Close", on_close_shop_button_clicked)
            )
        ],
    )
}

fn on_close_shop_button_clicked(
    _: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<GameplayState>>,
) {
    next_state.set(GameplayState::Placement);
}

fn on_buy_button_clicked(
    _: Trigger<ButtonClick>,
    selected_actor_type: Res<SelectedActorType>,
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    if let Some(ActorSelection {
        actor_type_id,
        shop_index,
    }) = selected_actor_type.0.as_ref()
    {
        if game.buy_item(actor_type_id) {
            commands.trigger(StartDragEvent {
                actor_type_id: actor_type_id.clone(),
                source: DragSource::Shop {
                    shop_index: *shop_index,
                },
            });
        }
    }
}
