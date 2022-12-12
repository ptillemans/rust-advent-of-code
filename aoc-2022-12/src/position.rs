use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position { x: x as i32, y: y as i32 }
    }
}

impl Position {

    pub fn manhattan(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

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
        let actual: Position = (1 as i32, 2 as i32).into();
        let expected = Position { x: 1, y: 2 };

        assert_eq!(actual, expected);
    }  

    #[test]
    fn test_into_usize() {
        let actual: Position = (1 as usize, 2 as usize).into();
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
