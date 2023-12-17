use aoc_common::position::Position;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub cave: Cave,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InstrumentType {
    MirrorP45,
    MirrorN45,
    SplitterEW,
    SplitterNS,
}

impl Display for InstrumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstrumentType::MirrorP45 => write!(f, "/"),
            InstrumentType::MirrorN45 => write!(f, "\\"),
            InstrumentType::SplitterEW => write!(f, "-"),
            InstrumentType::SplitterNS => write!(f, "|"),
        }
    }
}

impl FromStr for InstrumentType {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "/" => Ok(InstrumentType::MirrorP45),
            "\\" => Ok(InstrumentType::MirrorN45),
            "-" => Ok(InstrumentType::SplitterEW),
            "|" => Ok(InstrumentType::SplitterNS),
            _ => Err(AocError::ParseError),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Cave {
    pub width: usize,
    pub length: usize,
    pub floor: HashMap<Position, InstrumentType>,
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.length as i32 {
            for col in 0..self.width as i32 {
                if let Some(instrument) = self.floor.get(&Position::new(row, col)) {
                    write!(f, "{}", instrument)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Cave {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();
        let floor = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(col, c)| {
                    let position = Position::new(col as i32, row as i32);
                    c.to_string()
                        .parse::<InstrumentType>()
                        .ok()
                        .map(|instrument| (position, instrument))
                })
            })
            .collect::<HashMap<_, _>>();

        Ok(Cave {
            width,
            length,
            floor,
        })
    }
}

impl Cave {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_instrument(&self, position: &Position) -> Option<&InstrumentType> {
        self.floor.get(position)
    }

    pub fn get_instrument_mut(&mut self, position: &Position) -> Option<&mut InstrumentType> {
        self.floor.get_mut(position)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn in_bounds(&self, position: Position) -> bool {
        position.x >= 0
            && position.x < self.width as i32
            && position.y >= 0
            && position.y < self.length as i32
    }

    pub fn get_beams(&self, position: &Position, direction: &Direction) -> HashSet<Beam> {
        let mut beams = HashSet::new();
        let mut stack = vec![(*position, *direction)];
        let mut seen = HashSet::new();

        while let Some((start, direction)) = stack.pop() {
            let mut push_stack = |position: &Position, direction: &Direction| {
                let position = *position;
                let direction = *direction;
                if self.in_bounds(position) && !seen.contains(&(position, direction)) {
                    stack.push((position, direction));
                    seen.insert((position, direction));
                }
            };

            let mut pos = start;

            let keep_going = |position: &Position, direction: &Direction| {
                    if let Some(instrument) = self.get_instrument(position) {
                        match instrument {
                            InstrumentType::MirrorP45 => false,
                            InstrumentType::MirrorN45 => false,
                            InstrumentType::SplitterNS => {
                                *direction == Direction::North || *direction == Direction::South
                            }
                            InstrumentType::SplitterEW => {
                                *direction == Direction::East || *direction == Direction::West
                            }
                        }
                    } else {
                        true
                    }
            };

            while self.in_bounds(pos + direction.into()) && keep_going(&pos, &direction) {
                pos = pos + direction.into();
            }
            beams.insert(Beam::new(&start, &pos));

            if let Some(instrument) = self.get_instrument(&pos) {
                match instrument {
                    InstrumentType::MirrorP45 => {
                        let new_direction = match direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::North,
                            Direction::South => Direction::West,
                            Direction::West => Direction::South,
                        };
                        push_stack(&(pos + new_direction.into()), &new_direction);
                    }
                    InstrumentType::MirrorN45 => {
                        let new_direction = match direction {
                            Direction::North => Direction::West,
                            Direction::East => Direction::South,
                            Direction::South => Direction::East,
                            Direction::West => Direction::North,
                        };
                        push_stack(&(pos + new_direction.into()), &new_direction);
                    }
                    InstrumentType::SplitterEW => {
                        if direction == Direction::North || direction == Direction::South {
                            push_stack(&(pos + Direction::East.into()), &Direction::East);
                            push_stack(&(pos + Direction::West.into()), &Direction::West);
                        }
                    }
                    InstrumentType::SplitterNS => {
                        if direction == Direction::East || direction == Direction::West {
                            push_stack(&(pos + Direction::North.into()), &Direction::North);
                            push_stack(&(pos + Direction::South.into()), &Direction::South);
                        }
                    }
                }
            }
        }
        beams
    }

    pub fn energized_tiles (&self, position: Position, direction: Direction) -> HashSet<Position> {
        let beams = self.get_beams(&position, &direction);
        beams.iter().flat_map(|beam| beam.energized_tiles()).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<&Direction> for Position {
    fn from(val: &Direction) -> Self {
        match val {
            Direction::North => Position::new(0, -1),
            Direction::East => Position::new(1, 0),
            Direction::South => Position::new(0, 1),
            Direction::West => Position::new(-1, 0),
        }
    }
}

impl From<Direction> for Position {
    fn from(val: Direction) -> Position {
        match val  {
            Direction::North => Position::new(0, -1),
            Direction::East => Position::new(1, 0),
            Direction::South => Position::new(0, 1),
            Direction::West => Position::new(-1, 0),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Beam {
    start: Position,
    end: Position,
}

impl Beam {
    fn new(start: &Position, end: &Position) -> Beam {
        Beam {
            start: *start,
            end: *end,
        }
    }

    pub fn energized_tiles (&self) -> Vec<Position> {
        if self.start == self.end {
            return vec![self.start];
        }
        let len = self.start.manhattan(&self.end);
        let delta = self.end - self.start;
        let unit_delta = (delta.x / len, delta.y / len);
        (0..=len)
            .map(|i| self.start + Position::new(unit_delta.0 * i, unit_delta.1 * i))
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cave = s.parse::<Cave>()?;
        Ok(InputModel { cave })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    pub fn input_data() -> InputModel {
        let cave = TEST_INPUT.parse::<Cave>().unwrap();
        InputModel { cave }
    }

    #[test]
    fn test_beams() {
        let input = input_data();
        let beams = input.cave.get_beams(&Position::new(0, 0), &Direction::East);
        assert!(beams.contains(&Beam::new(&Position::new(0, 0), &Position::new(1, 0))));
        assert!(beams.contains(&Beam::new(&Position::new(1, 1), &Position::new(1, 7))));
        assert!(beams.contains(&Beam::new(&Position::new(0, 7), &Position::new(0, 7))));
        assert!(beams.contains(&Beam::new(&Position::new(2, 7), &Position::new(4, 7))));
        assert!(beams.contains(&Beam::new(&Position::new(4, 6), &Position::new(4, 6))));
        assert!(beams.contains(&Beam::new(&Position::new(5, 6), &Position::new(6, 6))));

        assert_eq!(beams.len(), 20);
    }

    #[test]
    fn test_energized_tiles() {
        let beam = Beam::new(&Position::new(0, 0), &Position::new(1, 0));
        assert_eq!(beam.energized_tiles(), vec![Position::new(0, 0), Position::new(1, 0)]);

        let beam = Beam::new(&Position::new(0, 0), &Position::new(2, 0));
        assert_eq!(beam.energized_tiles(), vec![Position::new(0, 0), Position::new(1, 0), Position::new(2, 0)]);

        let beam = Beam::new(&Position::new(0, 0), &Position::new(0, 2));
        assert_eq!(beam.energized_tiles(), vec![Position::new(0, 0), Position::new(0, 1), Position::new(0, 2)]);
    }
}
