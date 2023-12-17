use crate::position::Position;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .into_iter()
    }

    pub fn step(&self, pos: Position) -> Position {
        self.steps(pos, 1)
    }

    pub fn steps(&self, pos: Position, steps: usize) -> Position {
        let steps = steps as i32;
        match self {
            Direction::North => pos + Position::new(0, -steps),
            Direction::South => pos + Position::new(0, steps),
            Direction::West => pos + Position::new(-steps, 0),
            Direction::East => pos + Position::new(steps, 0),
        }
    }

    pub fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}
