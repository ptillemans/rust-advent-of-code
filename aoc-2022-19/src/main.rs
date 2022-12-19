#![feature(test)]
use std::collections::HashMap;

use aoc_2022_19::{AocError, InputModel};
use nom::{
    IResult, Parser, 
    sequence::tuple,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
};

const INPUT: &str = include_str!("../data/input.txt");


fn part1(_input: &InputModel) -> Result<String,AocError> {
    return Ok("Not implemented".to_string())
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    return Ok("Not implemented".to_string())
}

fn main() -> Result<(), AocError> {
    let input:InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Resource {
    fn from_str(s: &str) -> Result<Self, AocError> {
        match s {
            "ore" => Ok(Resource::Ore),
            "clay" => Ok(Resource::Clay),
            "obsidian" => Ok(Resource::Obsidian),
            "geode" => Ok(Resource::Geode),
            _ => Err(AocError::ParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BluePrint {
    pub id: u32,
    pub name: String,
    pub recipes: HashMap<Resource, u32>,
}

impl BluePrint {
    pub fn new(name: String, recipes: HashMap<Resource, u32>) -> Self {
        Self { name, recipes }
    }

    pub fn parser(input: &str) -> IResult<&str, Self> {
        tuple((
            tag("Blueprint "),
            digit1.map(|s: &str| s.parse::<u32>().unwrap()),
            tag(": "),
            separated_list1(
                tag(" "), 
                tuple((
                    digit1.map(|s: &str| s.parse::<u32>().unwrap()),
                    recipe_parser,
                ))
        )).map(|(_, id, _ ))| {
        
}


fn recipe_parser(input: &str) -> IResult<&str, Self> {
    alt((
        tuple((
            tag("Each "),
            digit1.map(|s: &str| s.parse::<Resource>().unwrap()),

            tag("costs "),
            digit1.map(|s: &str| s.parse::<u32>().unwrap()),
            tag(" "),
            alpha1.map(|s: &str| s.to_string()),
        )).map(|(_, cost, _, name)| (name, cost)),
        tuple((
            digit1.map(|s: &str| s.parse::<u32>().unwrap()),
            tag(" "),
            alpha1.map(|s: &str| s.to_string()),
        )).map(|(cost, _, name)| (name, cost)),
    )).map(|(name, cost)| (Resource::from(name), cost))
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "";

    pub fn input_data() -> InputModel {
        InputModel {
        }
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(&input_data()))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(&input_data()))
    }

}
