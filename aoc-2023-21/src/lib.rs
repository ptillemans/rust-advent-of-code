use aoc_common::{grid::Grid, position::Position};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub garden: Garden,
    pub start: Position,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut garden = s.parse::<Garden>()?;
        let start = garden
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &tile)| tile == Tile::Start)
                    .map(move |(x, _)| Position::new(x as i32, y as i32))
            })
            .next()
            .ok_or(AocError::ParseError)?;
        garden.set(start, Tile::Garden);
        Ok(InputModel { garden, start })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Garden,
    Rock,
    Start,
}

impl FromStr for Tile {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Garden),
            "#" => Ok(Tile::Rock),
            "S" => Ok(Tile::Start),
            _ => Err(AocError::ParseError),
        }
    }
}

type Garden = Grid<Tile>;

fn reachable(garden: &Garden, current: &HashSet<Position>) -> HashSet<Position> {
    current
        .iter()
        .flat_map(|&pos| {
            pos.neighbors().into_iter().filter(|&new_pos| {
                garden
                    .get(new_pos)
                    .map(|&tile| tile == Tile::Garden)
                    .unwrap_or(false)
            })
        })
        .collect()
}

pub fn reachable_in_steps(garden: &Garden, start: Position, steps: usize) -> HashSet<Position> {
    let mut current = HashSet::new();
    current.insert(start);

    for _ in 0..steps {
        current = reachable(garden, &current);
    }

    current
}

fn get_cyclic(garden: &Garden, pos: Position) -> Option<&Tile> {
    let (x, y) = (
        pos.x.rem_euclid(garden.width() as i32),
        pos.y.rem_euclid(garden.height() as i32),
    );
    let npos = Position::new(x, y);
    garden.get(npos)
}

pub fn cyclic_reachable(garden: &Garden, current: &HashSet<Position>) -> HashSet<Position> {
    current
        .iter()
        .flat_map(|&pos| {
            pos.neighbors().into_iter().filter(|&new_pos| {
                get_cyclic(garden, new_pos)
                    .map(|&tile| tile == Tile::Garden)
                    .unwrap_or(false)
            })
        })
        .collect()
}

pub fn cyclic_in_steps(garden: &Garden, start: Position, steps: usize) -> HashSet<Position> {
    let mut current = HashSet::new();
    current.insert(start);

    for _ in 0..steps {
        current = cyclic_reachable(garden, &current);
    }

    current
}

pub fn count_cis(garden: &Garden, start: Position, steps: usize) -> usize {
    let mut current = HashSet::new();
    current.insert(start);

    for _ in 0..steps {
        current = cyclic_reachable(garden, &current);
    }

    current.len()
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    use super::*;

    #[test]
    fn test_parse() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();

        assert_eq!(input.garden.width(), 11);
        assert_eq!(input.garden.height(), 11);
        assert_eq!(input.start, Position::new(5, 5));

        assert_eq!(input.garden.get(Position::new(4, 5)), Some(&Tile::Garden));
    }

    #[test]
    fn test_reachable() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let reachable = reachable(&input.garden, &vec![input.start].into_iter().collect());

        assert_eq!(reachable.len(), 2);
        assert!(reachable.contains(&Position::new(4, 5)));
        assert!(reachable.contains(&Position::new(5, 4)));
    }

    #[test]
    fn test_reachable_in_steps() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let reachable = reachable_in_steps(&input.garden, input.start, 6);

        assert_eq!(reachable.len(), 16);
    }

    #[test]
    fn test_cyclic_reachable_in_steps() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let reachable = cyclic_in_steps(&input.garden, input.start, 6);
        assert_eq!(reachable.len(), 16);
        let reachable = cyclic_in_steps(&input.garden, input.start, 10);
        println!("reachable: {:?}", reachable);
        assert_eq!(reachable.len(), 50);
        let reachable = cyclic_in_steps(&input.garden, input.start,50);
        assert_eq!(reachable.len(), 1594);
        let reachable = cyclic_in_steps(&input.garden, input.start,100);
        assert_eq!(reachable.len(), 6536);
        // let reachable = cyclic_in_steps(&input.garden, input.start,500);
        // assert_eq!(reachable.len(), 167004);
        // let reachable = cyclic_in_steps(&input.garden, input.start,1000);
        // assert_eq!(reachable.len(), 668697);
        // let reachable = cyclic_in_steps(&input.garden, input.start,5000);
       // assert_eq!(reachable.len(), 16733044);
    }
}
