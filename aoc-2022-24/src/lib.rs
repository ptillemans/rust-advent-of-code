use std::str::FromStr;
use aoc_common::position::Position;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    start: Position,
    finish: Position,
    width: i32,
    height: i32,
    blizzards: Vec<Blizzard>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;
        let start_x = lines[0].find('.').ok_or(AocError::ParseError)?;
        let finish_y = lines[height as usize - 1].find('.').ok_or(AocError::ParseError)?;
        let blizzards: Vec<Blizzard> = lines.iter().enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                        match c {
                            '^' => Some(Direction::Up),
                            'v' => Some(Direction::Down),
                            '<' => Some(Direction::Left),
                            '>' => Some(Direction::Right),
                            _ => None,
                        }.map(|d| Blizzard::new(Position::new(x as i32, y as i32), d))
                    })
                })
            .collect();
        Ok(InputModel {
            start: Position::new(start_x as i32, 0),
            finish: Position::new(finish_y as i32, height - 1),
            width,
            height,
            blizzards,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
struct Blizzard {
    start: Position,
    direction: Direction,
}
impl Blizzard {
    fn new(start: Position, direction: Direction) -> Blizzard {
        Blizzard {
            start,
            direction,
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    const SIMPLE_INPUT: &str = "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";


    #[test]
    fn test_parse() {
        let input = SIMPLE_INPUT;
        let model = input.parse::<InputModel>().unwrap();
        let expected = InputModel {
            start: Position::new(1, 0),
            finish: Position::new(5, 6),
            width: 7,
            height: 7,
            blizzards: vec![
                Blizzard::new(Position::new(1, 2), Direction::Right),
                Blizzard::new(Position::new(4, 4), Direction::Down),
            ],
        };


        assert_eq!(model, expected);
    }
}
