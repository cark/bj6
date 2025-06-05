#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum Direction {
    Front,
    Behind,
    Left,
    Right,
}

impl Direction {
    pub fn rotate(self) -> Self {
        match self {
            Direction::Front => Direction::Right,
            Direction::Right => Direction::Behind,
            Direction::Behind => Direction::Left,
            Direction::Left => Direction::Front,
        }
    }

    //for instance, Behind from Left is Right,
    // Left of Left is Behind,
    // Front of Front is Front
    #[allow(dead_code)]
    pub fn relative_direction(self, other: Direction) -> Direction {
        match self {
            Direction::Front => other,
            Direction::Behind => other.rotate().rotate(),
            Direction::Left => other.rotate().rotate().rotate(),
            Direction::Right => other.rotate(),
        }
    }
}
