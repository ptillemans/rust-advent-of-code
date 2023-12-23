use std::{collections::HashMap, str::FromStr};

use aoc_common::{direction::Direction, grid::Grid, position::Position};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
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
        let start = grid[0]
            .iter()
            .position(|c| *c == Cell::Path)
            .ok_or(AocError::ParseError)? as i32;
        let l = grid.height() - 1;
        let end = grid[l]
            .iter()
            .position(|c| *c == Cell::Path)
            .ok_or(AocError::ParseError)? as i32;

        Ok(InputModel {
            grid,
            start: Position::new(start, 0),
            end: Position::new(end, l as i32),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Forrest,
    Path,
    Slope(Direction),
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

fn direction(a: Position, b: Position) -> Direction {
    if a.x < b.x {
        Direction::East
    } else if a.x > b.x {
        Direction::West
    } else if a.y < b.y {
        Direction::South
    } else {
        Direction::North
    }
}

pub fn find_longest_path(grid: &Grid<Cell>, start: Position, end: Position) -> usize {
    let mut todo = vec![(0, start)];
    let mut best = HashMap::<Position, usize>::new();
    let mut came_from = HashMap::<Position, Position>::new();
    came_from.insert(start, start);

    while let Some((steps, pos)) = todo.pop() {
        let &prev = came_from.get(&pos).unwrap();
        let dir = direction(prev, pos);

        let next = pos
            .neighbors()
            .into_iter()
            .filter(|&p| grid.in_bounds(p))
            .filter(|&p| p != prev)
            .filter(|&p| {
                let c = grid.get(p).unwrap();
                let dir = direction(pos, p);
                c == &Cell::Path || c == &Cell::Slope(dir)
            })
            .filter(|p| best.get(p).map(|s| *s < steps + 1).unwrap_or(true))
            .collect::<Vec<_>>();

        next.iter().for_each(|&p| {
            came_from.insert(p, pos);
            best.insert(p, steps + 1);
            todo.push((steps + 1, p));
        });
    }

    let (pos, steps) = best.iter().max_by_key(|(_, steps)| *steps).unwrap();

    *steps
}

pub fn find_longest_path2(grid: &Grid<Cell>, start: Position, end: Position) -> usize {
    let mut todo = vec![(0, start)];
    let mut best = HashMap::<Position, usize>::new();
    let mut came_from = HashMap::<Position, Position>::new();
    came_from.insert(start, start);

    while let Some((steps, pos)) = todo.pop() {
        let &prev = came_from.get(&pos).unwrap();
        let dir = direction(prev, pos);

        let next = pos
            .neighbors()
            .into_iter()
            .filter(|&p| grid.in_bounds(p))
            .filter(|&p| p != prev)
            .filter(|&p| {
                let c = grid.get(p).unwrap();
                c == &Cell::Path || matches!(c, Cell::Slope(_))
            })
            .filter(|p| best.get(p).map(|s| *s < steps + 1).unwrap_or(true))
            .collect::<Vec<_>>();

        next.iter().for_each(|&p| {
            came_from.insert(p, pos);
            best.insert(p, steps + 1);
            todo.push((steps + 1, p));
        });
    }

    let (pos, steps) = best.iter().max_by_key(|(_, steps)| *steps).unwrap();

    *steps
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "#.#####################
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

    #[test]
    fn test_longest_path() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let steps = find_longest_path(&input.grid, input.start, input.end);

        assert_eq!(steps, 94);
    }

    #[test]
    fn test_longest_path2() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let steps = find_longest_path2(&input.grid, input.start, input.end);

        assert_eq!(steps, 94);
    }
}
