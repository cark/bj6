//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    // ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
};

use crate::theme::{interaction::InteractionPalette, palette::*};

#[derive(Component)]
pub struct Disabled;

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
            row_gap: Px(20.0),

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
    button_base(
        text,
        40.,
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
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        30.,
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
    )
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonClick;

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, B, M, I>(
    text: impl Into<String>,
    font_size: f32,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action_system = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        // Disabled,
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            let main_button_entity = parent.target_entity();

            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        disabled: BUTTON_DISABLED_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font_size(font_size),
                        TextColor(BUTTON_TEXT),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
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
        commands.run_system_cached(|mut commands: Commands, query: Query<Entity, With<T>>| {
            for ent in query.iter() {
                commands.entity(ent).remove::<Disabled>();
            }
        });
    } else {
        commands.run_system_cached(|mut commands: Commands, query: Query<Entity, With<T>>| {
            for ent in query.iter() {
                commands.entity(ent).insert(Disabled);
            }
        });
    }
}
