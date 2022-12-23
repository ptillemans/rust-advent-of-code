use std::str::FromStr;
use grid::{Grid, Move, parse_moves};

mod grid;
pub mod cube;
pub mod part1;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("Invalid Direction")]
    DirectionError,
}
      

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    grid: Grid,
    moves: Vec<Move>,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let parts = lines.as_slice()
            .split(|line| line.is_empty())
            .collect::<Vec<_>>();

        let grid = Grid::new(parts[0]);

        let moves = parse_moves(parts[1][0])?;
        Ok(InputModel { grid, moves })
    }
}


pub const TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        assert_eq!(input.grid.len(), 12);
        assert_eq!(input.moves, vec![
            Move::Forward(10),
            Move::TurnRight,
            Move::Forward(5),
            Move::TurnLeft,
            Move::Forward(5),
            Move::TurnRight,
            Move::Forward(10),
            Move::TurnLeft,
            Move::Forward(4),
            Move::TurnRight,
            Move::Forward(5),
            Move::TurnLeft,
            Move::Forward(5),
        ]);
    }
    
}
