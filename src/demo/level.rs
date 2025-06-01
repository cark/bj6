//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    AppSystems, asset_tracking::LoadResource, camera::MainCamera, data::game_config::GameConfig,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();

    app.add_systems(
        Update,
        center_checker
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
struct Checker;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    checker: Handle<Image>,
    // music: Handle<AudioSource>,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Level;

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            checker: assets.load("images/checker.png"),
            //music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    // player_assets: Res<PlayerAssets>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Level,
        Name::new("Checker"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        Checker,
        Sprite {
            image: level_assets.checker.clone(),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 20.,
            },
            custom_size: Some(Vec2::splat(4000.0)),
            ..default()
        },
        // children![
        //     player(400.0, &player_assets, &mut texture_atlas_layouts),
        //     (
        //         Name::new("Gameplay Music"),
        //         music(level_assets.music.clone())
        //     )
        // ],
    ));
}

fn center_checker(
    camera: Single<&Transform, (With<MainCamera>, Without<Checker>)>,
    checker: Single<&mut Transform, (With<Checker>, Without<MainCamera>)>,
    config: Res<GameConfig>,
) {
    let camera_transform = camera.into_inner();
    let mut checker_transform = checker.into_inner();
    let tile_size = config.checker.tile_size;
    checker_transform.translation = camera_transform
        .translation
        .div_euclid(Vec3::splat(tile_size * 2.0))
        * Vec3::splat(tile_size * 2.0);
}
