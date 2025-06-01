use bevy::{platform::collections::HashSet, prelude::*};

use crate::{asset_tracking::LoadResource, audio::sound_effect};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.init_resource::<ButtonHovering>();
    app.add_systems(Update, apply_interaction_palette);

    app.register_type::<InteractionAssets>();
    app.load_resource::<InteractionAssets>();
    app.add_observer(play_on_hover_sound_effect);
    app.add_observer(play_on_click_sound_effect);
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
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (
            Entity,
            &Interaction,
            &InteractionPalette,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
    mut button_hovering: ResMut<ButtonHovering>,
) {
    for (entity, interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => {
                button_hovering.0.remove(&entity);
                // info!("hovering: {}", button_hovering.hover_count());
                palette.none
            }
            Interaction::Hovered => {
                button_hovering.0.insert(entity);
                // info!("hovering: {}", button_hovering.hover_count());
                palette.hovered
            }
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
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
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.target()) {
        commands.spawn(sound_effect(interaction_assets.hover.clone()));
    }
}

fn play_on_click_sound_effect(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.target()) {
        commands.spawn(sound_effect(interaction_assets.click.clone()));
    }
}
