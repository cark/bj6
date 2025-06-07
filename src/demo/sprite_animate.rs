use std::time::Duration;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, execute_animations);
}

#[derive(Component, Clone)]
pub struct SpriteAnim {
    frame_count: u8,
    // fps: u8,
    frame_timer: Timer,
}

impl SpriteAnim {
    pub fn new(frame_count: u8, fps: u8) -> Self {
        Self {
            frame_count,
            // fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / (fps as f32)),
            TimerMode::Repeating,
        )
    }
}

fn execute_animations(time: Res<Time>, mut query: Query<(&mut SpriteAnim, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index += 1;
                if atlas.index >= config.frame_count as usize {
                    atlas.index = 0;
                }
            }
        }
    }
}
