use std::time::Duration;

use bevy::{prelude::*, render::view::visibility, text::cosmic_text::Angle};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, execute_particles);
}

#[derive(Component, Debug, Clone)]
pub struct ParticleConfig {
    timer: Timer,
    speed: Vec2,
    tweeners: Vec<ParticleTweener>,
}

#[derive(Debug, Clone)]
pub enum ParticleTweener {
    // Position {
    //     start: Vec3,
    //     end: Vec3,
    //     ease: EaseFunction,
    // },
    Speed {
        start: Isometry2d,
        end: Isometry2d,
        ease: EaseFunction,
    },
    Scale {
        start: f32,
        end: f32,
        ease: EaseFunction,
    },
    Color {
        start: Color,
        end: Color,
        ease: EaseFunction,
    },
    Rotation {
        start: f32,
        end: f32,
        ease: EaseFunction,
    },
}

impl ParticleConfig {
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
            tweeners: Vec::new(),
            speed: Vec2::X,
        }
    }

    pub fn add_tweener(mut self, tween: ParticleTweener) -> Self {
        self.tweeners.push(tween);
        self
    }
}

#[derive(Event, Debug, Clone)]
pub struct ParticleEvent<E>(E);

pub fn spawn_particle<E: Event>(
    mut cmd: Commands,
    spawn_event: E,
    particle_config: ParticleConfig,
    position: Vec3,
    scale: f32,
) {
    let entity = cmd
        .spawn((
            Transform::from_scale(Vec3::splat(scale)).with_translation(position),
            Visibility::Hidden,
            particle_config,
        ))
        .id();
    cmd.trigger_targets(ParticleEvent(spawn_event), entity);
}

fn execute_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut ParticleConfig,
        &mut Transform,
        &mut Sprite,
        &mut Visibility,
    )>,
) {
    for (entity, mut config, mut tr, mut sprite, mut visibility) in &mut query {
        config.timer.tick(time.delta());
        if config.timer.just_finished() {
            commands.entity(entity).despawn();
            continue;
        }
        *visibility = Visibility::Visible;
        for tween in &config.tweeners {
            match tween {
                // ParticleTweener::Position { start, end, ease } => {
                //     let t = ease.sample_unchecked(config.timer.fraction());
                //     tr.translation = start.lerp(*end, t);
                // }
                ParticleTweener::Speed { start, end, ease } => {
                    // config.speed is Vec2::X
                    let t = ease.sample_unchecked(config.timer.fraction());
                    let iso = Isometry2d::new(
                        start.translation.lerp(end.translation, t),
                        start.rotation.slerp(end.rotation, t),
                    );
                    // Calculate speed as the interpolated translation vector rotated by the interpolated rotation.
                    let speed = iso.rotation * iso.translation;
                    // let z = tr.translation.z;
                    tr.translation += speed.extend(0.0) * time.delta().as_secs_f32();
                }
                ParticleTweener::Scale { start, end, ease } => {
                    let t = ease.sample_unchecked(config.timer.fraction());
                    tr.scale = Vec3::splat(start.lerp(*end, t));
                }
                ParticleTweener::Color { start, end, ease } => {
                    let t = ease.sample_unchecked(config.timer.fraction());
                    sprite.color = start.mix(end, t);
                    //start.lerp(*end, t);
                }
                ParticleTweener::Rotation { start, end, ease } => {
                    let t = ease.sample_unchecked(config.timer.fraction());
                    tr.rotation = Quat::from_rotation_z(start.lerp(*end, t));
                }
            }
        }
    }
}
