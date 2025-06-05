//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    data::game_config::{GameConfig, GameConfigHandle},
    model::actor_types::ActorTypesHandle,
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<GameConfig>();
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        enter_gameplay_screen.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
    );
}

fn spawn_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        widget::center_ui_root("Loading Screen"),
        StateScoped(Screen::Loading),
        children![widget::label("Loading...")],
    ));
    let game_config_handle = GameConfigHandle(asset_server.load("game.config.toml"));
    let actor_types_handle = ActorTypesHandle(asset_server.load("all.actor_types.toml"));
    commands.insert_resource(game_config_handle);
    commands.insert_resource(actor_types_handle);
}

fn enter_gameplay_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
