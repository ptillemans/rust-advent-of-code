use std::str::FromStr;

use aoc_common::{direction::Direction, grid::Grid, position::Position};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub grid: Grid<Cell>,
    pub start: Position,
    pub end: Position,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.parse::<Grid<Cell>>()?;
        let start = grid[0].iter().position(|c| *c == Cell::Path)
            .ok_or(AocError::ParseError)? as i32;
        let l = grid.height()-1;
        let end = grid[l].iter().position(|c| *c == Cell::Path)
            .ok_or(AocError::ParseError)? as i32;
        
        
        Ok(InputModel { grid, start: Position::new(start, 0), end: Position::new(end, l as i32) })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Forrest,
    Path,
    Slope(Direction)
}

impl FromStr for Cell {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "." => Ok(Cell::Path),
                    "#" => Ok(Cell::Forrest),
                    "^" => Ok(Cell::Slope(Direction::North)),
                    ">" => Ok(Cell::Slope(Direction::East)),
                    "v" => Ok(Cell::Slope(Direction::South)),
                    "<" => Ok(Cell::Slope(Direction::West)),
                    _ => Err(AocError::ParseError),
                }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str="#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_parse() {

        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = InputModel {
            grid: TEST_INPUT.parse().unwrap(),
            start: Position::new(1, 0),
            end: Position::new(21, 22),

        };

        assert_eq!(expected, actual);
    }
}
