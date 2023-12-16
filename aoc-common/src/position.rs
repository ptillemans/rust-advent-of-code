use std::{ops::{Add, Sub}, fmt::{Display, Formatter}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32, 
    pub y: i32
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        (self.x + other.x, self.y + other.y).into()
    }
}   

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        (self.x - other.x, self.y - other.y).into()
    }
}

impl <T> From<(T, T)> for Position
    where T: Into<i32> {
    fn from((x, y): (T, T)) -> Self {
        Position { x: x.into(), y: y.into() }
    }
}

impl From<Position> for (i32, i32) {
    fn from(position: Position) -> Self {
        (position.x, position.y)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


impl Position {

    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    // calculate the manhattan distance between two positions
    pub fn manhattan(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    // calculate the chebychev distance between two positions
    pub fn chebychev(&self, other: &Position) -> i32 {
        (self.x - other.x).abs().max((self.y - other.y).abs())
    }

    // calculate the euclidean distance between two positions
    pub fn distance(&self, other: &Position) -> f32 {
        ((self.x as f32 - other.x as f32).powf(2.0) 
            + (self.y as f32 - other.y as f32).powf(2.0)).sqrt()
    }

}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_add() {
        let actual = Position { x: 1, y: 2 } + Position { x: 3, y: 4 };
        let expected = Position { x: 4, y: 6 };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_into_i32() {
        let actual: Position = (1, 2).into();
        let expected = Position { x: 1, y: 2 };

        assert_eq!(actual, expected);
    }  

    #[test]
    fn test_into_usize() {
        let actual: Position = (1, 2).into();
        let expected = Position { x: 1, y: 2 };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_manhattan() {
        let actual = Position { x: 1, y: 2 }.manhattan(&Position { x: 3, y: 4 });
        let expected = 4;

        assert_eq!(actual, expected);
    }
}
