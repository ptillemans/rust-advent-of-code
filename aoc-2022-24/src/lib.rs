use std::{str::FromStr, fmt::{Display, Formatter}};
use aoc_common::position::Position;

#[derive(Debug, PartialEq, Eq)]
pub struct Valley {
    start: Position,
    finish: Position,
    width: i32,
    height: i32,
    blizzards: Vec<Blizzard>,
}

impl Valley {
    pub(crate) fn blizzards_positions(&self, i: i32) -> Vec<Position> {
        self.blizzards.iter()
            .map(|b| self.blizzard_position(b, i))
            .collect()
    }

    pub(crate) fn blizzard_position(&self, blizzard: &Blizzard, time: i32) -> Position {
        let (mut x, mut y) = (blizzard.start.x, blizzard.start.y);
        let floor_height = self.height - 2;
        let floor_width = self.height - 2;
        match blizzard.direction {
            Direction::Up =>  y = (y - time - 1).rem_euclid(floor_height) + 1,
            Direction::Down => y = (y + time - 1).rem_euclid(floor_height) + 1,
            Direction::Left => x = (x - time - 1).rem_euclid(floor_width) + 1,
            Direction::Right => x = (x + time - 1).rem_euclid(floor_width) + 1,
        }
        (x, y).into()
    }
}

impl Display for Valley {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![vec!['.'; self.width as usize]; self.height as usize];
        (0..self.width as usize).for_each(|x| {
            lines[0][x] = '#';
            lines[self.height as usize -1][x] = '#';
        });
        (0..self.height as usize).for_each(|y| {
            lines[y][0] = '#';
            lines[y][self.width as usize - 1] = '#';
        });
        lines[self.start.y as usize][self.start.x as usize] = '.';
        lines[self.finish.y as usize][self.finish.x as usize] = '.';
        self.blizzards.iter().for_each(|b| {
            let symbol = match b.direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Right => '>',
                Direction::Left => '<',
            };
            lines[b.start.y as usize][b.start.x as usize] = symbol;
        });

        for line in lines {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        };
        Ok(())
    }
}


#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for Valley {
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
        Ok(Valley {
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
        let model = input.parse::<Valley>().unwrap();
        let expected = Valley {
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

    #[test]
    fn test_display() {
        let input = SIMPLE_INPUT;
        let model = input.parse::<Valley>().unwrap();
        let actual = format!("{model}");
        assert_eq!(actual.trim(), input);
    }

    #[test]
    fn test_blizzard_position() {
        let input = SIMPLE_INPUT;
        let model = input.parse::<Valley>().unwrap();
        let expected= vec![
            vec![Position::new(1,2), Position::new(4,4)],
            vec![Position::new(2,2), Position::new(4,5)],
            vec![Position::new(3,2), Position::new(4,1)],
            vec![Position::new(4,2), Position::new(4,2)],
            vec![Position::new(5,2), Position::new(4,3)],
            vec![Position::new(1,2), Position::new(4,4)],
            vec![Position::new(2,2), Position::new(4,5)],
            vec![Position::new(3,2), Position::new(4,1)],
            vec![Position::new(4,2), Position::new(4,2)],
            vec![Position::new(5,2), Position::new(4,3)],
        ];
        let actual = (0..10)
            .map(|i| model.blizzards_positions(i))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}
