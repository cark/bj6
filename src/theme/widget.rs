//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    // ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
};

use crate::theme::{interaction::InteractionPalette, palette::*};

use super::interaction::BackgroundChangeRequest;

#[derive(Component)]
pub struct Disabled;

#[derive(Component)]
pub struct Selected;

/// A root UI node that fills the window and centers its content.
pub fn center_ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// A root UI node that fills the window and centers its content.
pub fn gameplay_ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            row_gap: Px(0.0),

            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(40.0),
        TextColor(HEADER_TEXT),
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(24.0),
        TextColor(LABEL_TEXT),
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    button_base(
        action,
        (
            Node {
                width: Px(380.0),
                height: Px(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
        (
            Name::new("Button Text"),
            Text(text),
            TextFont::from_font_size(40.),
            TextColor(BUTTON_TEXT),
            // Don't bubble picking events from the text up to the button.
        ),
        InteractionPalette {
            none: BUTTON_BACKGROUND,
            disabled: BUTTON_DISABLED_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        },
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    button_base(
        action,
        Node {
            width: Px(30.0),
            height: Px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::axes(Px(40.0), Px(20.0)),
            flex_grow: 1.0,
            ..default()
        },
        (
            Name::new("Button Text"),
            Text(text),
            TextFont::from_font_size(30.),
            TextColor(BUTTON_TEXT),
        ),
        InteractionPalette {
            none: BUTTON_BACKGROUND,
            disabled: BUTTON_DISABLED_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        },
    )
}

/// A small square button with content and an action defined as an [`Observer`].
pub fn content_button<E, B, M, I>(content: impl Bundle, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        action,
        Node {
            // width: Px(30.0),
            // height: Px(30.0),
            // padding: UiRect::axes(Px(40.0), Px(20.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::axes(Px(20.0), Px(10.0)),
            flex_grow: 1.0,
            ..default()
        },
        content,
        InteractionPalette {
            none: BUTTON_BACKGROUND,
            disabled: BUTTON_DISABLED_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        },
    )
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonClick;

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, B, M, I>(
    action: I,
    button_bundle: impl Bundle,
    content_bundle: impl Bundle,
    palette: InteractionPalette,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let action_system = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node {
            border: UiRect::all(Px(2.0)),
            ..default()
        },
        BorderColor::DEFAULT,
        // Disabled,
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            let main_button_entity = parent.target_entity();

            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BackgroundColor(BUTTON_BACKGROUND),
                    palette,
                    children![(content_bundle, Pickable::IGNORE,)],
                ))
                .insert(button_bundle)
                .observe(action_system)
                .observe(move |trigger: Trigger<Pointer<Click>>, world: &mut World| {
                    let is_disabled = world.get::<Disabled>(main_button_entity).is_some();
                    if !is_disabled {
                        world.trigger_targets(ButtonClick, trigger.target());
                    }
                });
        })),
    )
}

pub fn set_enabled<T: Component>(commands: &mut Commands, enabled: bool) {
    if enabled {
        commands.run_system_cached(
            |mut commands: Commands, query: Query<(Entity, &Children), With<T>>| {
                for (ent, children) in query.iter() {
                    commands.entity(ent).remove::<Disabled>();
                    for child in children.iter() {
                        commands.trigger_targets(BackgroundChangeRequest, child);
                    }
                }
            },
        );
    } else {
        commands.run_system_cached(
            |mut commands: Commands, query: Query<(Entity, &Children), With<T>>| {
                for (ent, children) in query.iter() {
                    commands.entity(ent).insert(Disabled);
                    for child in children.iter() {
                        commands.trigger_targets(BackgroundChangeRequest, child);
                    }
                }
            },
        );
    }
}
