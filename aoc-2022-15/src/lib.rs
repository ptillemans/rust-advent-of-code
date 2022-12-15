use std::collections::HashSet;
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

    pub fn range(&self) -> i32 {
        self.location.manhattan(&self.beacon)
    }

    pub fn intersect_y(&self, y: i32) -> Option<(i32, i32)> {
        let dy = (y - self.location.y).abs();
        let dx = self.range() - dy;
        if dx < 0 {
            return None;
        }
        Some((self.location.x - dx, self.location.x + dx))
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

pub struct Cave {
    sensors: Vec<Sensor>,
    taken_positions: HashSet<Position>,
}

impl Cave {

    pub fn new(sensors: Vec<Sensor>) -> Self {
        let mut taken_positions = HashSet::new();
        for sensor in &sensors {
            taken_positions.insert(sensor.location);
            taken_positions.insert(sensor.beacon);
        }
        Self {
            sensors,
            taken_positions,
        }
    }

    // finc unique positions on line y for all sensors
    pub fn covered_positions(&self, y: i32) -> i32 {

        let covered = self.covered_parts(y);

        let taken = self.taken_positions.iter()
            .filter(|p| p.y == y)
            .filter(|p| covered.iter().map(|(l, r)| p.x >= *l && p.x <= *r).any(|b| b))
            .count();

        covered.iter().map(|p| p.1 - p.0 + 1).sum::<i32>() - taken as i32
    }

    pub fn covered_parts(&self, y: i32) -> Vec<(i32, i32)> {
        let mut covered_parts: Vec<(i32, i32)>= self.sensors.iter()
            .filter_map(|sensor| sensor.intersect_y(y))
            .collect();
        covered_parts.sort();

        let mut covered = vec![covered_parts[0]];
        for curr in covered_parts.iter().skip(1) {
            let prev = covered.pop().unwrap();
            if prev.1 >= curr.0 {
                if prev.1 < curr.1 {
                    covered.push((prev.0, curr.1));
                } else {
                    covered.push(prev);
                }
            } else {
                covered.push(prev);
                covered.push(*curr);
            }
        }
        covered
    }

    pub fn uncovered_areas(&self, bound: i32) -> Vec<(i32, Vec<(i32, i32)>)> {
        (0.. bound)
            .map(|y| (y, self.covered_parts(y)))
            .filter(|(_, parts)| parts.len() > 1)
            .map(|(y, parts)| (y, parts.iter()
                 .zip(parts.iter().skip(1))
                 .map(|(l, r)| (l.1 + 1, r.0 - 1))
                 .filter(|(l, r)| l <= r)
                 .collect::<Vec<(i32, i32)>>()))
            .filter(|(_, parts)| !parts.is_empty())
            .collect()
    }

    pub fn tuning_frequency(&self, bound: i32) -> Option<i64> {
        let uncovered_areas = self.uncovered_areas(bound);
        uncovered_areas.first()
            .map(|(y, r)| (r.first().unwrap().0, y))
            .filter(|(x, _)| *x > 0 && *x <= bound)
            .map(|(x, y)| (x as i64) * 4_000_000 + (*y as i64))
    }
}

pub const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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

    #[test]
    fn test_intersect() {
        let sensors = input_data().sensors;
        let cave = Cave::new(sensors);
        let actual = cave.covered_positions(10);
        let expected = 26;

        println!("covered positions: {:?}", actual);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_uncovered_areas() {
        let sensors = input_data().sensors;
        let cave = Cave::new(sensors);
        let actual = cave.uncovered_areas(20);
        let expected = vec![(11, vec![(14, 14)])];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_tuning_frequency() {
        let sensors = input_data().sensors;
        let cave = Cave::new(sensors);
        let actual = cave.tuning_frequency(20);
        let expected = Some(56000011);
        assert_eq!(actual, expected);
    }
}
