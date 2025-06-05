use bevy::prelude::*;

use bevy_common_assets::toml::TomlAssetPlugin;
use game_config::GameConfig;

use crate::{AppSystems, model::actor_types::ActorTypes};

pub mod game_config;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<game_config::GameConfig>::new(&[
        "config.toml",
    ]));
    app.add_plugins(TomlAssetPlugin::<ActorTypes>::new(&["actor_types.toml"]));
    app.add_systems(Update, reload_files.in_set(AppSystems::TickTimers));
}

fn reload_files(
    mut cmd: Commands,
    mut config_asset_events: EventReader<AssetEvent<GameConfig>>,
    mut actor_types_asset_events: EventReader<AssetEvent<ActorTypes>>,
    config_asset: Res<Assets<GameConfig>>,
    actor_types_asset: Res<Assets<ActorTypes>>,
    asset_server: Res<AssetServer>,
) {
    for ev in config_asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = ev {
            info!("config loaded.");
            let (_, config) = config_asset.iter().next().unwrap();
            cmd.insert_resource(config.clone());
        }
    }
    for ev in actor_types_asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = ev {
            info!("actor types loaded.");
            let (_, actor_types) = actor_types_asset.iter().next().unwrap();
            let mut actor_types = actor_types.clone();
            for (_name, actor_type) in actor_types.0.iter_mut() {
                actor_type.sprite_handle =
                    Some(asset_server.load(format!("images/{}", actor_type.sprite_name)));
            }
            cmd.insert_resource(actor_types);
        }
    }
}
