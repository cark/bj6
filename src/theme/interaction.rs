use bevy::{platform::collections::HashSet, prelude::*};

use crate::{asset_tracking::LoadResource, audio::sound_effect};

use super::{
    palette::BUTTON_SELECTED_BORDER,
    widget::{Disabled, Selected},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.init_resource::<ButtonHovering>();
    app.add_systems(Update, apply_interaction_palette);

    app.register_type::<InteractionAssets>();
    app.load_resource::<InteractionAssets>();
    app.add_observer(play_on_hover_sound_effect);
    app.add_observer(play_on_click_sound_effect);
    app.add_observer(on_background_change_request);
    app.add_observer(on_set_button_selected);
    app.add_observer(on_button_removed);
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetButtonSelectedEvent(pub bool);

fn on_set_button_selected(
    trigger: Trigger<SetButtonSelectedEvent>,
    mut commands: Commands,
    // name: Query<&Name>,
    mut border_color: Query<&mut BorderColor>,
) {
    let button = trigger.target();
    let SetButtonSelectedEvent(selected) = trigger.event();
    // let name = name.get(button).unwrap();
    let mut border_color = border_color.get_mut(button).unwrap();
    if *selected {
        // warn!("true {} {button:?}", name);
        commands.entity(button).insert(Selected);
        *border_color = BorderColor(BUTTON_SELECTED_BORDER);
    } else {
        // warn!("false {} {button:?}", name);
        commands.entity(button).remove::<Selected>();
        *border_color = BorderColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.0));
    }
}

#[derive(Resource, Clone, Debug, Default)]
pub struct ButtonHovering(HashSet<Entity>);

impl ButtonHovering {
    pub fn is_hovering(&self) -> bool {
        self.hover_count() > 0
    }

    pub fn hover_count(&self) -> usize {
        self.0.len()
    }
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub disabled: Color,
    pub hovered: Color,
    pub pressed: Color,
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackgroundChangeRequest;

fn on_background_change_request(
    trigger: Trigger<BackgroundChangeRequest>,
    mut palette_query: Query<(
        &Interaction,
        &InteractionPalette,
        &ChildOf,
        &mut BackgroundColor,
    )>,
    disabled_parent: Query<&Disabled>,
) {
    // warn!("1");
    let entity = trigger.target();
    if let Ok((interaction, palette, childof, mut background)) = palette_query.get_mut(entity) {
        // warn!("2");
        let parent = childof.parent();
        let parent_disabled = disabled_parent.get(parent).is_ok();
        *background = if parent_disabled {
            palette.disabled
        } else {
            match interaction {
                Interaction::None => palette.none,
                Interaction::Hovered => palette.hovered,
                Interaction::Pressed => palette.pressed,
            }
        }
        .into();
    }
}

fn apply_interaction_palette(
    mut commands: Commands,
    palette_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut button_hovering: ResMut<ButtonHovering>,
) {
    for (entity, interaction) in &palette_query {
        match interaction {
            Interaction::None => {
                button_hovering.0.remove(&entity);
                // info!("hovering: {}", button_hovering.hover_count());
            }
            Interaction::Hovered => {
                button_hovering.0.insert(entity);
                // info!("hovering: {}", button_hovering.hover_count());
            }
            Interaction::Pressed => (),
        }
        commands.trigger_targets(BackgroundChangeRequest, entity);
    }
}

fn on_button_removed(
    trigger: Trigger<OnRemove, Button>,
    mut button_hovering: ResMut<ButtonHovering>,
) {
    button_hovering.0.remove(&trigger.target());
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct InteractionAssets {
    #[dependency]
    hover: Handle<AudioSource>,
    #[dependency]
    click: Handle<AudioSource>,
}

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load("audio/sound_effects/button_hover.ogg"),
            click: assets.load("audio/sound_effects/button_click.ogg"),
        }
    }
}

fn play_on_hover_sound_effect(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<&ChildOf, With<Interaction>>,
    disabled: Query<&Disabled>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if let Ok(child_of) = interaction_query.get(trigger.target()) {
        if !disabled.contains(child_of.parent()) && interaction_query.contains(trigger.target()) {
            commands.spawn(sound_effect(interaction_assets.hover.clone()));
        }
    }
}

fn play_on_click_sound_effect(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<&ChildOf, With<Interaction>>,
    disabled: Query<&Disabled>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if let Ok(child_of) = interaction_query.get(trigger.target()) {
        if !disabled.contains(child_of.parent()) && interaction_query.contains(trigger.target()) {
            commands.spawn(sound_effect(interaction_assets.click.clone()));
        }
    }
}
