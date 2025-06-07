use bevy::math::{IVec2, ivec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum RelDir {
    Front,
    Back,
    Left,
    Right,
}

impl Dir {
    pub fn rotate(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    //for instance, Behind from Left is Right,
    // Left of Left is Behind,
    // Front of Front is Front
    #[allow(dead_code)]
    pub fn apply_relative(self, rel_dir: RelDir) -> Dir {
        match rel_dir {
            RelDir::Front => self,
            RelDir::Back => self.rotate().rotate(),
            RelDir::Left => self.rotate().rotate().rotate(),
            RelDir::Right => self.rotate(),
        }
    }

    pub fn apply_to(self, coord: IVec2) -> IVec2 {
        match self {
            Dir::Up => coord + IVec2::Y,
            Dir::Down => coord - IVec2::Y,
            Dir::Left => coord - IVec2::X,
            Dir::Right => coord + IVec2::X,
        }
    }

    // (1,0) is to the right
    // (0,1) is down
    pub fn rel_coord_to_coord(self, base_coord: IVec2, rel_coord: IVec2) -> IVec2 {
        match self {
            Dir::Up => base_coord + ivec2(-rel_coord.y, rel_coord.x),
            Dir::Down => base_coord + ivec2(rel_coord.y, -rel_coord.x),
            Dir::Right => base_coord + rel_coord,

            Dir::Left => base_coord - rel_coord,
        }
    }
}

// pub fn to_quat(self) -> Quat {
//     match self {
//         Dir::Up => Quat::from_rotation_z(std::f32::consts::PI),
//         Dir::Down =>  Quat::from_rotation_z(-std::f32::consts::PI / 2.0),
//         Dir::Left => Quat::from_rotation_z(std::f32::consts::PI / 2.0),
//         Dir::Right => ,
//     }
