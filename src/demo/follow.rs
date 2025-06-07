use bevy::prelude::*;
use bevy_tweening::{Lens, Targetable};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, fixup_followers);
}

#[derive(Component, Debug)]
pub struct Follows {
    pub target: Entity,
    pub offset: Vec3,
}

fn fixup_followers(
    mut commands: Commands,
    followers: Query<(Entity, &Follows)>,
    mut trs: Query<&mut Transform>,
) {
    for (follower, follows) in followers.iter() {
        if let Ok(&target_tr) = trs.get(follows.target) {
            // dbg!(follows.offset);
            let mut follower_tr = trs.get_mut(follower).unwrap();
            follower_tr.translation = target_tr.translation + follows.offset;
        } else {
            commands.entity(follower).despawn();
        }
    }
}

pub struct FollowerLens {
    pub start: Vec3,
    pub end: Vec3,
}

impl Lens<Follows> for FollowerLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Follows>, ratio: f32) {
        dbg!(ratio);
        let start = self.start;
        let end = self.end;
        target.offset = start + (end - start) * ratio;
    }
}
// impl Lens<Follows> for FollowerLens {
//     fn lerp(&mut self, target: &mut dyn Targetable<Follows>, ratio: f32) {
//         dbg!(ratio);
//         let start = self.start;
//         let end = self.end;
//         target.offset = start + (end - start) * ratio;
//     }
// }
