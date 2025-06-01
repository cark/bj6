use bevy::prelude::*;

use bevy_common_assets::toml::TomlAssetPlugin;
use game_config::GameConfig;

use crate::AppSystems;

pub mod game_config;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<game_config::GameConfig>::new(&[
        "config.toml",
    ]));
    app.add_systems(Update, reload_config.in_set(AppSystems::TickTimers));
}

fn reload_config(
    mut cmd: Commands,
    mut config_asset_events: EventReader<AssetEvent<GameConfig>>,
    config_asset: Res<Assets<GameConfig>>,
) {
    for ev in config_asset_events.read() {
        warn!("{ev:#?}");
        if let AssetEvent::LoadedWithDependencies { .. } = ev {
            info!("config loaded!");
            let (_, config) = config_asset.iter().next().unwrap();
            cmd.insert_resource(config.clone());
        }
    }
}
