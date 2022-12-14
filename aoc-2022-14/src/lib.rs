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
    paths: Vec<Path>
}

type Path = Vec<Position>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cave {
    grid: HashMap<Position, Material>,
}

impl Cave {
    fn new(paths: Vec<Path>) -> Cave {
        Cave {
            grid: expand_paths(paths)
        }
    }

    fn get(&self, pos: &Position) -> Material {
        self.grid
            .get(pos)
            .unwrap_or(&Material::Air)
            .clone()
    }
    
}


// expand the paths to a grid of positions
fn expand_paths(paths: Vec<Path>) -> HashMap<Position, Material> {
    paths.into_iter()
        .flat_map(|path| expand_path(path))
        .map(|position| (position, Material::Rock))
        .collect()

}

// return all positions between successive points in a path
fn expand_path(path: Path) -> Vec<Position> {
    path.windows(2)
        .flat_map(|window| (min(window[0].x,window[1].x)..=max(window[0].x,window[1].x))
                    .flat_map(|x| 
                        (min(window[0].y,window[1].y)..=max(window[0].y,window[1].y))
                           .map(move |y| Position::new(x, y))))
        .collect()
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

const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

fn test_input() -> InputModel {
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

        let actual = Cave::new(test_input().paths);
        
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

}
