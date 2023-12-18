use std::{str::FromStr, collections::BTreeSet};
use itertools::Itertools;

use aoc_common::{direction::Direction, position::Position};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub instructions: Vec<DigInstruction>,
    pub instructions2: Vec<DigInstruction>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.lines()
            .map(parse_line)
            .collect::<Result<Vec<_>, _>>()?;
        
        let instructions2 = s.lines()
            .map(parse_line_2)
            .collect::<Result<Vec<_>, _>>()?;
            
        Ok(InputModel { instructions, instructions2 })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DigInstruction {
    direction: Direction,
    distance: usize,
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Segment {
    start: Position,
    end: Position,
}

pub fn dig(input: &[DigInstruction]) -> Vec<Segment> {
    let mut position = Position::default();
    let mut segments = Vec::new();
    for instruction in input {
        let end = instruction.direction.steps(position, instruction.distance);
        segments.push(Segment {
            start: position,
            end,
        });
        position = end;
    }
    assert!(segments.len() == input.len());
    assert!(position == Position::default());
    segments
}


fn parse_line(line: &str) -> Result<DigInstruction, AocError> {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() != 3 {
        return Err(AocError::ParseError);
    }
    let direction = match parts[0].chars().next() {
        Some('R') => Direction::East,
        Some('L') => Direction::West,
        Some('U') => Direction::North,
        Some('D') => Direction::South,
        _ => return Err(AocError::ParseError),
    };
    let distance = parts[1] 
        .parse::<usize>()
        .map_err(|_| AocError::ParseError)?;

    Ok(DigInstruction {
        direction,
        distance,
    })
}

fn parse_line_2(line: &str) -> Result<DigInstruction, AocError> {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() != 3 {
        return Err(AocError::ParseError);
    }
    let s = parts[2].chars().skip(2).take(6).collect::<String>();
    let distance = usize::from_str_radix(&s[..5], 16)
        .map_err(|_| AocError::ParseError)?;

    let direction = match s.chars().last() {
        Some('0') => Direction::East,
        Some('2') => Direction::West,
        Some('3') => Direction::North,
        Some('1') => Direction::South,
        _ => return Err(AocError::ParseError),
    };

    Ok(DigInstruction {
        direction,
        distance,
    })
}



fn overlap(s: &Segment, y1: i32, y2: i32) -> Option<(i32, i32)> {
    let (sy_min, sy_max) = if s.start.y < s.end.y {(s.start.y, s.end.y)} else {(s.end.y, s.start.y)};
    if y2 < sy_min || y1 > sy_max {
        None 
    } else {
        let y_min = y1.max(sy_min);
        let y_max = y2.min(sy_max);
        if y_max > y_min {Some((y_min, y_max))} else {None}
    }
}

pub fn area(segments: &[Segment]) -> i64 {

    let y_points = segments.iter()
        .map(|s| s.start.y)
        .collect::<BTreeSet<_>>();

    let y_parts = y_points.iter()
        .zip(y_points.iter().skip(1))
        .collect::<Vec<_>>();

    // collect NS segments in ordered list
    let ns_segments = segments.iter()
        .filter(|s| s.start.x == s.end.x)
        .map(|s| Segment {
            start: Position::new(s.start.x, s.start.y.min(s.end.y)),
            end: Position::new(s.start.x, s.start.y.max(s.end.y)),
        })
        .collect::<BTreeSet<Segment>>();

    let rectangles = y_parts.into_iter()
        .flat_map(|(&y1, &y2)| {
            ns_segments.iter()
                .filter_map(move |s| {
                    overlap(s, y1, y2)
                        .map(|(y1, y2)| (s.start.x, y1, y2) )})
                .tuples()
                .map(|((x1, y1_1, y2_1), (x2, _, _))| {
                    assert!(x1 < x2);
                    assert!(y1_1 < y2_1);
                    (x1, y1_1, x2, y2_1)
                })

        })
        .collect::<BTreeSet<_>>();

    let area = rectangles.iter()
        .map(|(x1, y1, x2, y2)| (x2 - x1 + 1) as i64 * (y2 - y1 + 1) as i64)
        .sum::<i64>();
    
    let overlaps = rectangles.iter()
        .map(|(ax, _, bx, by)| {
            rectangles.iter()
                .filter(|(_, cy, _, _)| cy == by)
                .map(|(cx, _, dx, _)| {
                    if bx < cx || ax > dx {
                        0
                    } else {
                        (bx.min(dx) - ax.max(cx) + 1) as i64
                    }
                })
                .sum::<i64>()
        })
    .sum::<i64>();

     area - overlaps 
    
}


#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";


    #[test]
    fn test_parse() {
        let input = "R 6 (#70c710)";
        let actual = parse_line(input).unwrap();
        let expected = DigInstruction {
            direction: Direction::East,
            distance: 6,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse2() {
        let input = "R 6 (#70c710)";
        let actual = parse_line_2(input).unwrap();
        let expected = DigInstruction {
            direction: Direction::East,
            distance: 461937,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dig() {
        let instructions = TEST_INPUT.parse::<InputModel>().unwrap().instructions;
        let actual = dig(&instructions);
        let expected = vec![
            Segment {
                start: Position::default(),
                end: Position::new(6, 0),
            },
            Segment {
                start: Position::new(6, 0),
                end: Position::new(6, 5),
            },
            Segment {
                start: Position::new(6, 5),
                end: Position::new(4, 5),
            },
            Segment {
                start: Position::new(4, 5),
                end: Position::new(4, 7),
            },
            Segment {
                start: Position::new(4, 7),
                end: Position::new(6, 7),
            },
            Segment {
                start: Position::new(6, 7),
                end: Position::new(6, 9),
            },
            Segment {
                start: Position::new(6, 9),
                end: Position::new(1, 9),
            },
            Segment {
                start: Position::new(1, 9),
                end: Position::new(1, 7),
            },
            Segment {
                start: Position::new(1, 7),
                end: Position::new(0, 7),
            },
            Segment {
                start: Position::new(0, 7),
                end: Position::new(0, 5),
            },
            Segment {
                start: Position::new(0, 5),
                end: Position::new(2, 5),
            },
            Segment {
                start: Position::new(2, 5),
                end: Position::new(2, 2),
            },
            Segment {
                start: Position::new(2, 2),
                end: Position::new(0, 2),
            },
            Segment {
                start: Position::new(0, 2),
                end: Position::new(0, 0),
            },
        ];

        assert_eq!(actual, expected);
       
    }


    #[test]
    fn test_overlap() {
        let segment = Segment {
            start: (7, 11).into(),
            end: (7, 15).into(),
        };

        let actual = overlap(&segment, 1, 5);
        assert_eq!(None, actual);

        let actual = overlap(&segment, 1, 25);
        assert_eq!(Some((11, 15)), actual);

        let actual = overlap(&segment, 1, 13);
        assert_eq!(Some((11, 13)), actual);

        let actual = overlap(&segment, 13, 25);
        assert_eq!(Some((13, 15)), actual);
        
        let actual = overlap(&segment, 12, 13);
        assert_eq!(Some((12, 13)), actual);
        
    }

    #[test]
    fn test_area() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let instructions = input.instructions;

        let expected = 62;
            let actual = area(&dig(&instructions));
        assert_eq!(expected, actual)
        
    }
}
