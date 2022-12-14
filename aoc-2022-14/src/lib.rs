#![feature(result_option_inspect)]
use std::{
    collections::HashMap,
    str::FromStr,
    cmp::{min, max},
};
use aoc_common::position::Position;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputModel  {
    pub paths: Vec<Path>
}

type Path = Vec<Position>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cave {
    grid: HashMap<Position, Material>,
    depth: i32,
}

impl Cave {
    pub fn new(paths: &[Path]) -> Cave {
        let grid = expand_paths(paths);
        let depth = grid.keys()
            .map(|pos| pos.y)
            .max()
            .unwrap_or(0);
        Cave { grid, depth }
    }

    fn get(&self, pos: &Position) -> Material {
        self.grid
            .get(pos)
            .unwrap_or(&Material::Air)
            .clone()
    }

    fn put(&mut self, pos: &Position, material: Material) {
        self.grid.insert(*pos, material);
    }

    pub fn drop_sand(&mut self, pos: &Position) -> Option<Position> {
        rest_position(self, pos, self.depth)
            .inspect(|pos| self.put(pos, Material::Sand))
    }
    
    pub fn drop_sand_with_floor(&mut self, pos: &Position) -> Option<Position> {
        rest_position_with_floor(self, pos, self.depth + 2)
            .inspect(|pos| self.put(pos, Material::Sand))
    }
    
}


// expand the paths to a grid of positions
fn expand_paths(paths: &[Path]) -> HashMap<Position, Material> {
    paths.iter()
        .flat_map(expand_path)
        .map(|position| (position, Material::Rock))
        .collect()

}

// return all positions between successive points in a path
fn expand_path(path: &Path) -> Vec<Position> {
    path.windows(2)
        .flat_map(|window| (min(window[0].x,window[1].x)..=max(window[0].x,window[1].x))
                    .flat_map(|x| 
                        (min(window[0].y,window[1].y)..=max(window[0].y,window[1].y))
                           .map(move |y| Position::new(x, y))))
        .collect()
}

fn rest_position(cave: &Cave, pos: &Position, max_y: i32) -> Option<Position> {
    let mut last = *pos;
    let mut next = Some(*pos);
    while let Some(pos) = next {
        if pos.y > max_y {
            return None;
        }
        last = pos;
        next = vec![Position::new(pos.x, pos.y + 1),
             Position::new(pos.x - 1, pos.y + 1),
             Position::new(pos.x + 1, pos.y + 1)]
         .into_iter()
         .find(|pos| cave.get(pos) == Material::Air);
    }
    Some(last)
}

fn rest_position_with_floor(cave: &Cave, pos: &Position, floor: i32) -> Option<Position> {
    let mut last = *pos;
    let mut next = Some(*pos);

    // stop if drop location is blocked
    if cave.get(&Position::new(pos.x, pos.y)) != Material::Air {
        return None;
    }

    while let Some(pos) = next {
        if pos.y >= floor {
            break;
        }
        last = pos;
        next = vec![Position::new(pos.x, pos.y + 1),
             Position::new(pos.x - 1, pos.y + 1),
             Position::new(pos.x + 1, pos.y + 1)]
         .into_iter()
         .find(|pos| cave.get(pos) == Material::Air);
    }
    Some(last)
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        separated_list1(line_ending, parse_path)(s)
            .map_err(|_| AocError::ParseError)
            .map(|(_, paths)| InputModel { paths })
    }

}

fn parse_path(s: &str) -> IResult<&str, Path> {
   separated_list1(tag(" -> "), parse_position)(s)
}

fn parse_position(s: &str) -> IResult<&str, Position> {
    separated_pair(
        digit1.map(|x: &str| x.parse::<i32>().unwrap()),
        char(','),
        digit1.map(|x: &str| x.parse::<i32>().unwrap()),
    ).map(|(x, y)| Position::new(x, y)).parse(s)
}

pub const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

pub fn test_input() -> InputModel {
    InputModel {
        paths: vec![
            vec![
                Position::new(498, 4),
                Position::new(498, 6),
                Position::new(496, 6),
            ],
            vec![
                Position::new(503, 4),
                Position::new(502, 4),
                Position::new(502, 9),
                Position::new(494, 9),
            ],
        ],
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = test_input();

        assert_eq!(actual, expected);
    }


    #[test]
    fn test_paths_to_grid() {

        let actual = Cave::new(&test_input().paths);
        
        assert_eq!(actual.get(&Position::new(498,3)), Material::Air);
        assert_eq!(actual.get(&Position::new(498,4)), Material::Rock);
        assert_eq!(actual.get(&Position::new(498,5)), Material::Rock);
        assert_eq!(actual.get(&Position::new(498,6)), Material::Rock);
        assert_eq!(actual.get(&Position::new(498,7)), Material::Air);
        assert_eq!(actual.get(&Position::new(497,6)), Material::Rock);
        assert_eq!(actual.get(&Position::new(496,6)), Material::Rock);
        assert_eq!(actual.get(&Position::new(495,6)), Material::Air);

        assert_eq!(actual.get(&Position::new(503,3)), Material::Air);
        assert_eq!(actual.get(&Position::new(503,4)), Material::Rock);
        assert_eq!(actual.get(&Position::new(502,7)), Material::Rock);
        assert_eq!(actual.get(&Position::new(500,9)), Material::Rock);

    }

    #[test]
    fn test_drop_sand() {
        let mut cave = Cave::new(&test_input().paths);
        let data = vec![
            Position::new(500, 8),
            Position::new(499, 8),
            Position::new(501, 8),
            Position::new(500, 7),
            Position::new(498, 8),
        ];

        let drop_pos = Position::new(500, 0);
        for pos in data {
            if let Some(actual) = cave.drop_sand(&drop_pos) {
                assert_eq!(actual, pos);
            } else {
                panic!();
            }
        }
    }
}
