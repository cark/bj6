use std::time::Duration;

use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy_tween::{
    interpolate::*,
    prelude::*,
    tween::{AnimationTarget, TargetComponent},
};

// use bevy_tweening::{Animator, RepeatCount, RepeatStrategy, Tween, lens::TransformScaleLens};

use crate::{
    AppSystems,
    data::game_config::GameConfig,
    model::{actor::ActorView, game::Game},
    screens::Screen,
};

use super::{GameplayState, actor::ActorEntities, level::LevelAssets, mouse::MouseWorldCoords};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<HoveredTileCoord>();
    app.init_resource::<HoveredActor>();
    app.add_systems(
        Update,
        (
            (
                update_hovered_tile_coord,
                update_hovered_actor,
                show_selected_actor_tile.run_if(in_state(GameplayState::Placement)),
            )
                .chain(),
            show_hovered_tile_debug,
        )
            .chain()
            .in_set(AppSystems::TickTimers)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Debug, Default, Deref)]
pub struct HoveredTileCoord(Option<IVec2>);

#[derive(Resource, Debug, Default, Deref)]
pub struct HoveredActor(Option<(Entity, ActorView)>);

fn update_hovered_tile_coord(
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

fn show_hovered_tile_debug(
    mut _cmd: Commands,
    mut gizmos: Gizmos,
    hovered_tile_coord: Res<HoveredTileCoord>,
    ui_debug_options: Res<UiDebugOptions>,
    config: Res<GameConfig>,
) {
    if ui_debug_options.enabled {
        if let Some(coord) = hovered_tile_coord.0 {
            gizmos.rect_2d(
                tile_coord_to_world_coord(coord, config.checker.tile_size),
                Vec2::splat(config.checker.tile_size),
                RED_400,
            );
        }
    }
}

pub fn tile_coord_to_world_coord(coord: IVec2, tile_size: f32) -> Vec2 {
    (coord.as_vec2() + Vec2::splat(0.5)) * tile_size
}

#[allow(dead_code)]
pub fn world_coord_to_tile_coord(coord: Vec2, tile_size: f32) -> IVec2 {
    (coord / tile_size).floor().as_ivec2()
}

pub fn update_hovered_actor(
    hovered_tile_coord: Res<HoveredTileCoord>,
    game: Res<Game>,
    mut hovered_actor: ResMut<HoveredActor>,
    actor_entities: Res<ActorEntities>,
) {
    hovered_actor.0 = hovered_tile_coord.and_then(|coord| {
        let actor_id = game.board().coord_to_actor_id(&coord)?;
        let actor_view = game.actor_view(&actor_id)?;
        let entity = actor_entities.get(&actor_id)?;
        Some((entity, actor_view))
    });
}

#[derive(Component, Debug, Default)]
struct SelectedActorRect;

fn show_selected_actor_tile(
    mut commands: Commands,
    hovered_actor_entity: Res<HoveredActor>,
    hovered_tile_coord: Res<HoveredTileCoord>,
    config: Res<GameConfig>,
    mut rects: Query<(Entity, &mut Transform), With<SelectedActorRect>>,
    assets: Res<LevelAssets>,
) {
    if hovered_actor_entity.is_some() {
        if let Some(coord) = **hovered_tile_coord {
            let translation =
                tile_coord_to_world_coord(coord, config.checker.tile_size).extend(1.5);
            if let Ok((_e, mut tr)) = rects.single_mut() {
                *tr = Transform::from_translation(translation);
            } else {
                let target = TargetComponent::marker();
                commands
                    .spawn((
                        Name::new("SelectedActorRect"),
                        Transform::from_translation(translation),
                        SelectedActorRect,
                        StateScoped(GameplayState::Placement),
                        Sprite {
                            image: assets.actor_rect.clone(),
                            custom_size: Some(Vec2::splat(config.checker.tile_size)),
                            ..default()
                        },
                        AnimationTarget,
                    ))
                    .animation()
                    .repeat(Repeat::Infinitely)
                    .repeat_style(RepeatStyle::PingPong)
                    .insert_tween_here(
                        Duration::from_secs_f32(0.2),
                        EaseKind::CircularOut,
                        target.with(scale(Vec3::splat(0.90), Vec3::splat(1.05))),
                    );
            }
        }
    } else {
        for (e, _) in &rects {
            commands.entity(e).despawn();
        }
    }
}
