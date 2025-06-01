use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

use crate::{AppSystems, data::game_config::GameConfig, screens::Screen};

use super::mouse::MouseWorldCoords;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<HoveredTileCoord>();
    // app.init_gizmo_group::<MyGizmos>();
    app.add_systems(
        Update,
        (update_overed_tile_coord, show_hovered_tile)
            .chain()
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Debug, Default, Deref)]
pub struct HoveredTileCoord(Option<IVec2>);

// We can create our own gizmo config group!
// #[derive(Default, Reflect, GizmoConfigGroup)]
// struct MyGizmos {}

fn update_overed_tile_coord(
    mut hovered_tile_coord: ResMut<HoveredTileCoord>,
    mouse_world_coords: Res<MouseWorldCoords>,
    config: Res<GameConfig>,
) {
    let tile_size = config.checker.tile_size;
    hovered_tile_coord.0 = mouse_world_coords.map(|coord| {
        let x = (coord.x / tile_size).floor() as i32;
        let y = (coord.y / tile_size).floor() as i32;
        ivec2(x, y)
    });
}

fn show_hovered_tile(
    mut _cmd: Commands,
    mut gizmos: Gizmos,
    hovered_tile_coord: Res<HoveredTileCoord>,
    ui_debug_options: Res<UiDebugOptions>,
    config: Res<GameConfig>,
) {
    if ui_debug_options.enabled {
        if let Some(coord) = hovered_tile_coord.0 {
            let tile_size = config.checker.tile_size;
            let center: Vec2 = (coord.as_vec2() + Vec2::splat(0.5)) * tile_size;

            gizmos.rect_2d(center, Vec2::splat(tile_size), RED_400);
        }
    }
}
