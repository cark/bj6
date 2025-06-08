use std::time::Duration;

use bevy::prelude::*;

use rand::Rng;

use crate::{
    data::game_config::GameConfig,
    demo::{
        level::LevelAssets,
        particle::{ParticleConfig, ParticleEvent, spawn_particle},
        sprite_animate::SpriteAnim,
        tile::tile_coord_to_world_coord,
    },
};

use super::particle::ParticleTweener;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_spawn_puff);
    app.add_observer(on_spawn_drop_particles);
    app.add_observer(on_spawn_hit_particles);
    app.add_systems(Startup, setup);
    // app.add_systems(Update, click);
}

#[derive(Event, Debug, Clone)]
struct SpawnPuffEvent;

#[derive(Resource, Debug, Clone, Deref)]
struct PuffAtlasLayout(Handle<TextureAtlasLayout>);

#[derive(Event, Debug, Clone)]
pub struct SpawDropParticlesEvent(pub IVec2);

#[derive(Event, Debug, Clone)]
pub struct SpawnHitParticlesEvent(pub IVec2);

fn on_spawn_puff(
    trigger: Trigger<ParticleEvent<SpawnPuffEvent>>,
    mut commands: Commands,
    assets: Res<LevelAssets>,
    layout: Res<PuffAtlasLayout>,
) {
    // let ev = trigger.event();
    commands.entity(trigger.target()).insert((
        Sprite {
            image: assets.puff.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout.0.clone(),
                index: (rand::random::<f32>() * 3.0) as usize,
            }),
            ..default()
        },
        SpriteAnim::new(3, 6),
    ));
}

fn setup(mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, mut commands: Commands) {
    let layout = TextureAtlasLayout::from_grid(uvec2(16, 16), 3, 1, None, None);
    let handle = texture_atlas_layouts.add(layout);
    commands.insert_resource(PuffAtlasLayout(handle));
}

// fn click(mut commands: Commands, input: Res<ButtonInput<MouseButton>>) {
//     if input.just_pressed(MouseButton::Right) {
//         // commands.trigger(SpawDropParticlesEvent(ivec2(0, 0)));
//         // spawn_particle(
//         //     commands.reborrow(),
//         //     SpawnPuffEvent,
//         //     ParticleConfig::new(Duration::from_secs_f32(2.))
//         //         .add_tweener(ParticleTweener::Speed {
//         //             start: Isometry2d::new(Vec2::X * 50., Rot2::degrees(60.0)),
//         //             end: Isometry2d::new(Vec2::ZERO, Rot2::degrees(60.0)),
//         //             ease: EaseFunction::CubicOut,
//         //         })
//         //         .add_tweener(ParticleTweener::Scale {
//         //             start: 0.2,
//         //             end: 1.5,
//         //             ease: EaseFunction::QuadraticOut,
//         //         })
//         //         .add_tweener(ParticleTweener::Color {
//         //             start: Color::WHITE,
//         //             end: Color::WHITE.with_alpha(0.0),
//         //             ease: EaseFunction::CircularIn,
//         //         }),
//         //     Vec3::splat(0.).with_z(3.0),
//         //     1.,
//         // );
//     }
// }

fn on_spawn_drop_particles(
    trigger: Trigger<SpawDropParticlesEvent>,
    mut commands: Commands,
    config: Res<GameConfig>,
) {
    let ev = trigger.event();
    let mut rng = rand::thread_rng();

    let world_coord = tile_coord_to_world_coord(ev.0, config.checker.tile_size).extend(1.5);

    for _i in 0..config.particles.drop_count {
        let angle = rng.gen_range(0.0..360.);
        spawn_particle(
            commands.reborrow(),
            SpawnPuffEvent,
            ParticleConfig::new(Duration::from_secs_f32(rng.gen_range(
                config.particles.drop_duration * 0.5..config.particles.drop_duration * 1.5,
            )))
            .add_tweener(ParticleTweener::Speed {
                start: Isometry2d::new(
                    Vec2::X
                        * rng.gen_range(
                            config.particles.drop_magnitude * 0.5
                                ..config.particles.drop_magnitude * 1.5,
                        ),
                    Rot2::degrees(angle),
                ),
                end: Isometry2d::new(Vec2::ZERO, Rot2::degrees(angle)),
                ease: EaseFunction::CubicOut,
            })
            .add_tweener(ParticleTweener::Scale {
                start: 0.1,
                end: rng.gen_range(1.0..3.0),
                ease: EaseFunction::QuadraticOut,
            })
            .add_tweener(ParticleTweener::Color {
                start: Color::WHITE,
                end: Color::WHITE.with_alpha(0.0),
                ease: EaseFunction::QuadraticOut,
            }),
            world_coord,
            1.,
        );
    }
}

fn on_spawn_hit_particles(
    trigger: Trigger<SpawnHitParticlesEvent>,
    mut commands: Commands,
    config: Res<GameConfig>,
) {
    let ev = trigger.event();
    let mut rng = rand::thread_rng();

    let world_coord = tile_coord_to_world_coord(ev.0, config.checker.tile_size).extend(3.5);
    // let world_coord = Vec3(x, y, z);

    for _i in 0..15 {
        let angle = rng.gen_range(0.0..360.);
        spawn_particle(
            commands.reborrow(),
            SpawnPuffEvent,
            ParticleConfig::new(Duration::from_secs_f32(rng.gen_range(
                config.particles.drop_duration * 0.5..config.particles.drop_duration * 1.5,
            )))
            .add_tweener(ParticleTweener::Speed {
                start: Isometry2d::new(
                    Vec2::X
                        * rng.gen_range(
                            config.particles.drop_magnitude * 0.5
                                ..config.particles.drop_magnitude * 1.5,
                        ),
                    Rot2::degrees(angle),
                ),
                end: Isometry2d::new(Vec2::ZERO, Rot2::degrees(angle)),
                ease: EaseFunction::CubicOut,
            })
            .add_tweener(ParticleTweener::Scale {
                start: 0.1,
                end: rng.gen_range(1.0..3.0),
                ease: EaseFunction::QuadraticOut,
            })
            .add_tweener(ParticleTweener::Color {
                start: Color::linear_rgba(1.0, 0.0, 0.0, 1.0),
                end: Color::WHITE.with_alpha(0.0),
                ease: EaseFunction::QuadraticOut,
            }),
            world_coord,
            1.,
        );
    }
}
