use bevy::prelude::*;

// Removed: use bevy_common_assets::toml::TomlAssetPlugin;
use game_config::GameConfig;

use crate::model::actor_types::ActorTypes; // Adjusted order for consistency

pub mod game_config;

fn setup_static_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load GameConfig
    const GAME_CONFIG_STR: &str = include_str!("../../assets/config.toml");
    match toml::from_str::<GameConfig>(GAME_CONFIG_STR) {
        Ok(config) => {
            info!("GameConfig loaded via include_str!");
            commands.insert_resource(config);
        }
        Err(e) => {
            error!("Failed to parse embedded game.gcfg: {}", e);
        }
    }

    // Load ActorTypes
    const ACTOR_TYPES_STR: &str = include_str!("../../assets/actor_types.toml");
    match toml::from_str::<ActorTypes>(ACTOR_TYPES_STR) {
        Ok(mut actor_types) => {
            info!("ActorTypes loaded via include_str!");
            for (_name, actor_type) in actor_types.0.iter_mut() {
                if !actor_type.sprite_name.is_empty() {
                    actor_type.sprite_handle =
                        Some(asset_server.load(format!("images/{}", actor_type.sprite_name)));
                }
            }
            commands.insert_resource(actor_types);
        }
        Err(e) => {
            error!("Failed to parse embedded all.atypes: {}", e);
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_static_data);
    // The reload_files system, which relied on AssetEvents for these, is no longer suitable for them.
    // Hot reloading for these specific files is removed with this approach.
    // app.add_systems(Update, reload_files.in_set(AppSystems::TickTimers));
}

// fn reload_files(
//     mut cmd: Commands,
//     mut config_asset_events: EventReader<AssetEvent<GameConfig>>,
//     mut actor_types_asset_events: EventReader<AssetEvent<ActorTypes>>,
//     config_asset: Res<Assets<GameConfig>>,
//     actor_types_asset: Res<Assets<ActorTypes>>,
//     asset_server: Res<AssetServer>,
// ) {
//     for ev in config_asset_events.read() {
//         // This part will no longer trigger for GameConfig if loaded via include_str
//         if let AssetEvent::LoadedWithDependencies { .. } = ev {
//             info!("config loaded.");
//             let (_, config) = config_asset.iter().next().unwrap();
//             cmd.insert_resource(config.clone());
//         }
//     }
//     for ev in actor_types_asset_events.read() {
//         // This part will no longer trigger for ActorTypes if loaded via include_str
//         if let AssetEvent::LoadedWithDependencies { .. } = ev {
//             info!("actor types loaded.");
//             let (_, actor_types) = actor_types_asset.iter().next().unwrap();
//             let mut actor_types = actor_types.clone();
//             for (_name, actor_type) in actor_types.0.iter_mut() {
//                 actor_type.sprite_handle =
//                     Some(asset_server.load(format!("images/{}", actor_type.sprite_name)));
//                 // Note: Sprite loading logic is now duplicated in setup_static_data.
//                 // Consider removing this loop if reload_files is kept for other asset types,
//                 // or ensure it doesn't conflict. For now, setup_static_data handles initial load.
//             }
//             cmd.insert_resource(actor_types);
//         }
//     }
// }

// #[derive(Resource, Asset, Clone, Reflect)]
// #[reflect(Resource)]
// pub struct DataAssets {
//     #[dependency]
//     pub checker: Handle<Image>,
// }
