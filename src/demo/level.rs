//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    AppSystems,
    asset_tracking::LoadResource,
    camera::MainCamera,
    data::game_config::GameConfig,
    demo::{GameplayState, ui::actions::SetActiveActionEvent},
    model::{actor_types::ActorTypes, game::Game},
    screens::Screen,
};

use super::tile::tile_coord_to_world_coord;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();

    app.add_systems(
        Update,
        spawn_center_checker
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        OnEnter(Screen::Gameplay),
        enter.in_set(AppSystems::TickTimers),
    );
    app.add_systems(OnExit(Screen::Gameplay), exit);
    app.add_systems(
        Update,
        update_actions.run_if(in_state(GameplayState::Placement)),
    );
}

fn update_actions(mut commands: Commands) {
    commands.trigger(SetActiveActionEvent("start_turn".to_string(), true));
}

#[derive(Component)]
struct Checker;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    pub checker: Handle<Image>,
    #[dependency]
    pub read_only: Handle<Image>,
    #[dependency]
    pub coin: Handle<Image>,
    #[dependency]
    pub turn: Handle<Image>,
    #[dependency]
    pub round: Handle<Image>,
    #[dependency]
    pub poke: Handle<Image>,
    #[dependency]
    pub actor_rect: Handle<Image>,
    #[dependency]
    pub lmb: Handle<Image>,
    #[dependency]
    pub rmb: Handle<Image>,
    #[dependency]
    pub mmb: Handle<Image>,
    #[dependency]
    pub rotate: Handle<Image>,
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
            read_only: assets.load("images/read_only.png"),
            coin: assets.load("images/coin.png"),
            turn: assets.load("images/turn.png"),
            round: assets.load("images/round.png"),
            poke: assets.load("images/poke.png"),
            actor_rect: assets.load("images/actor_rect.png"),
            lmb: assets.load("images/lmb.png"),
            rmb: assets.load("images/rmb.png"),
            mmb: assets.load("images/mmb.png"),
            rotate: assets.load("images/rotate.png"),
            //music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

pub fn enter(mut commands: Commands, actor_types: Res<ActorTypes>, game_config: Res<GameConfig>) {
    let mut game = Game::new(&game_config.game, actor_types.clone());
    game.restock();
    let start_actor_id = game.board().start_actor_id();
    commands.insert_resource(game);
    commands.spawn(start_actor_id);
}

pub fn exit(mut commands: Commands) {
    commands.remove_resource::<Game>();
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    config: Res<GameConfig>,
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
        children![()],
    ));

    commands.spawn((
        StateScoped(Screen::Gameplay),
        Name::new("ReadOnlyTile"),
        Transform::from_translation(
            tile_coord_to_world_coord(ivec2(0, 0), config.checker.tile_size).extend(1.),
        ),
        Visibility::default(),
        Sprite {
            image: level_assets.read_only.clone(),
            custom_size: Some(Vec2::splat(config.checker.tile_size)),

            ..default()
        },
    ));
}

fn spawn_center_checker(
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
