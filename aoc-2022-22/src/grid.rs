use std::{str::FromStr, fmt::{Display, Formatter}, ops::Index };

use aoc_common::position::Position;

use crate::AocError;

use nom::{
    Parser,IResult,
    multi::many1,
    bytes::complete::tag,
    character::complete::digit1,
    branch::alt,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Void,
    Empty,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    TurnLeft,
    TurnRight,
    Forward(i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<Direction> for i32 {
    type Error = AocError;

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::Right => Ok(0),
            Direction::Down => Ok(1),
            Direction::Left => Ok(2),
            Direction::Up => Ok(3),
        }
    }
}

impl Direction {
    pub fn turn(&self, move_: Move) -> Direction {
        match move_ {
            Move::TurnLeft => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Move::TurnRight => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            _ => *self
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter()
    }

    pub fn step(&self, pos: Position) -> Position {
        self.steps(pos, 1)
    }

    pub fn steps(&self, pos: Position,  steps: usize) -> Position {
        let steps = steps as i32;
        match self {
            Direction::Up => pos + Position::new(0, -steps),
            Direction::Down => pos + Position::new(0, steps),
            Direction::Left => pos + Position::new(-steps, 0),
            Direction::Right => pos + Position::new(steps, 0),
        }
    }

    pub(crate) fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Move {
   pub fn distance(&self) -> i32 {
       match self {
           Move::TurnLeft => 0,
           Move::TurnRight => 0,
           Move::Forward(distance) => *distance,
       }
   }
}

impl FromStr for Move {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            'L' => Ok(Move::TurnLeft),
            'R' => Ok(Move::TurnRight),
            _ => s.parse()
                .map(Move::Forward)
                .map_err(|_| AocError::ParseError),
        }
    }
}

pub fn parse_moves(input: &str) -> Result<Vec<Move>, AocError> {
    let result: IResult<&str, Vec<Move>> = many1(
        alt((tag("L"), tag("R"), digit1))
            .map(|s: &str| s.parse::<Move>().unwrap())
    ).parse(input);
    result
        .map(|(_, moves)| moves)
        .map_err(|_| AocError::ParseError)
}



#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct Grid(Vec<Vec<Tile>>);

impl Grid {
   pub fn new(input: &[&str]) -> Grid {
       Grid(input.iter()
           .map(|row| 
                row.chars().map(|c| 
                    match c {
                       '#' => Tile::Wall,
                       '.' => Tile::Empty,
                       _ => Tile::Void,
                    })
                .collect::<Vec<_>>())
           .collect::<Vec<_>>())
   }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
   
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Vec<Tile>> {
       self.0.iter()
    }
}

impl FromStr for Grid {
   type Err = ();

   fn from_str(s: &str) -> Result<Self, Self::Err> {
       Ok(Grid::new(&s.lines().collect::<Vec<_>>()))
   }

}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for tile in row {
                match tile {
                    Tile::Void => write!(f, " ")?,
                    Tile::Empty => write!(f, ".")?,
                    Tile::Wall => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl From<Vec<Vec<Tile>>> for Grid {
    fn from(value: Vec<Vec<Tile>>) -> Self {
        Grid(value.iter()
            .map(|row| row.to_vec())
            .collect::<Vec<_>>())
    }
}

impl From<&[&[Tile]]> for Grid {
    fn from(value: &[&[Tile]]) -> Self {
        Grid(value.iter()
            .map(|row| row.to_vec())
            .collect::<Vec<_>>())
    }
}

impl IntoIterator for Grid {
    type Item = Vec<Tile>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Index<Position> for Grid {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl Index<usize> for Grid {
    type Output = Vec<Tile>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

