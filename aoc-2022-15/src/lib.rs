use std::str::FromStr;
use aoc_common::position::Position;
use nom::{
    IResult, Parser, 
    bytes::complete::tag,
    character::complete::{digit1, line_ending}, 
    combinator::opt,
    multi::separated_list1,
    sequence::{pair, tuple,}
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub sensors: Vec<Sensor>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        separated_list1(line_ending, Sensor::parser)(s)
            .map(|(_, sensors)| InputModel { sensors })
            .map_err(|_| AocError::ParseError)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sensor {
    location: Position,
    beacon: Position,
}

impl Sensor {
    pub fn new(location: Position, beacon: Position) -> Self {
        Self {
            location,
            beacon,
        }
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        tuple((
          tag("Sensor at "),
          position_parser,
          tag(": closest beacon is at "),
          position_parser,
        ))
        .map(|(_, location, _, beacon)| Self::new(location, beacon))
        .parse(input)
    }
}

fn position_parser(input: &str) -> IResult<&str, Position> {
    tuple((
        tag("x="), 
        integer_parser,
        tag(", y="),
        integer_parser,
    ))
        .map(|(_, x, _, y)| Position::new(x, y))
        .parse(input)
}

fn integer_parser(input: &str) -> IResult<&str, i32> {
    pair(
        opt(tag("-")),
        digit1,
    ).map(|(sign, digits)| {
        let sign = sign.map(|_:&str| -1).unwrap_or(1);
        let digits = digits.parse::<i32>().unwrap();
        sign * digits
    })
    .parse(input)
}


const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

pub fn input_data() -> InputModel {
    InputModel {
        sensors: vec![
            Sensor::new(Position{ x: 2, y: 18}, Position{x: -2, y: 15}),
            Sensor::new(Position{ x: 9, y: 16}, Position{x: 10, y: 16}),
            Sensor::new(Position{ x: 13, y: 2}, Position{x: 15, y: 3}),
            Sensor::new(Position{ x: 12, y: 14}, Position{x: 10, y: 16}),
            Sensor::new(Position{ x: 10, y: 20}, Position{x: 10, y: 16}),
            Sensor::new(Position{ x: 14, y: 17}, Position{x: 10, y: 16}),
            Sensor::new(Position{ x: 8, y: 7}, Position{x: 2, y: 10}),
            Sensor::new(Position{ x: 2, y: 0}, Position{x: 2, y: 10}),
            Sensor::new(Position{ x: 0, y: 11}, Position{x: 2, y: 10}),
            Sensor::new(Position{ x: 20, y: 14}, Position{x: 25, y: 17}),
            Sensor::new(Position{ x: 17, y: 20}, Position{x: 21, y: 22}),
            Sensor::new(Position{ x: 16, y: 7}, Position{x: 15, y: 3}),
            Sensor::new(Position{ x: 14, y: 3}, Position{x: 15, y: 3}),
            Sensor::new(Position{ x: 20, y: 1}, Position{x: 15, y: 3}),
        ]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_sensor() {
        let actual = Sensor::parser("Sensor at x=2, y=18: closest beacon is at x=-2, y=15");
        let expected = Sensor::new(Position{ x: 2, y: 18}, Position{x: -2, y: 15});

        println!("parsed sensor: {:?}", actual);
        let (rest, sensor) = actual.unwrap();
        assert_eq!(rest, "");
        assert_eq!(sensor, expected);
    }
}
