use aoc_common::position::Position;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Valley {
    pub start: Position,
    pub finish: Position,
    width: i32,
    height: i32,
    blizzards: Vec<Blizzard>,
    blizzard_cache: HashMap<i32, HashSet<Position>>,
}

impl Valley {
    pub(crate) fn blizzards_positions(&mut self, i: i32) -> HashSet<Position> {
        let blizzards = self.blizzard_cache.get(&i);
        if let Some(positions) = blizzards {
            return positions.clone();
        }
        let blizzards = self._blizzards_positions(i);
        self.blizzard_cache.insert(i, blizzards.clone());
        blizzards
    }

    fn _blizzards_positions(&self, i: i32) -> HashSet<Position> {
        self.blizzards
            .iter()
            .map(|b| self.blizzard_position(b, i))
            .collect()
    }

    fn blizzard_position(&self, blizzard: &Blizzard, time: i32) -> Position {
        let (mut x, mut y) = (blizzard.start.x, blizzard.start.y);
        let floor_height = self.height - 2;
        let floor_width = self.width - 2;
        match blizzard.direction {
            Direction::Up => y = (y - time - 1).rem_euclid(floor_height) + 1,
            Direction::Down => y = (y + time - 1).rem_euclid(floor_height) + 1,
            Direction::Left => x = (x - time - 1).rem_euclid(floor_width) + 1,
            Direction::Right => x = (x + time - 1).rem_euclid(floor_width) + 1,
        }
        (x, y).into()
    }

    fn is_free(&mut self, pos: Position, time: i32) -> bool {
        // check for walls
        (pos.x > 0 && pos.x < self.width - 1
         && pos.y > 0 && pos.y < self.height - 1
         // check for blizzards
         && !self.blizzards_positions(time).contains(&pos))
        // except start and finish
        || pos == self.start
        || pos == self.finish
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![vec!['.'; self.width as usize]; self.height as usize];
        (0..self.width as usize).for_each(|x| {
            lines[0][x] = '#';
            lines[self.height as usize - 1][x] = '#';
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
        }
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
        let lines = s.trim().lines().collect::<Vec<_>>();
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;

        let start_x = lines[0].find('.');
        let start_x = start_x.or_else(|| lines[0].find('E'));
        let start_x = start_x.ok_or(AocError::ParseError)?;
        let finish_x = lines[height as usize - 1]
            .find('.')
            .ok_or(AocError::ParseError)?;
        let blizzards: Vec<Blizzard> = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    match c {
                        '^' => Some(Direction::Up),
                        'v' => Some(Direction::Down),
                        '<' => Some(Direction::Left),
                        '>' => Some(Direction::Right),
                        _ => None,
                    }
                    .map(|d| Blizzard::new(Position::new(x as i32, y as i32), d))
                })
            })
            .collect();
        let blizzard_cache = HashMap::with_capacity(1000);
        Ok(Valley {
            start: Position::new(start_x as i32, 0),
            finish: Position::new(finish_x as i32, height - 1),
            width,
            height,
            blizzards,
            blizzard_cache,
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

impl FromStr for Direction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "right" => Ok(Direction::Right),
            "down" => Ok(Direction::Down),
            "left" => Ok(Direction::Left),
            _ => Err(AocError::ParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blizzard {
    start: Position,
    direction: Direction,
}
impl Blizzard {
    fn new(start: Position, direction: Direction) -> Blizzard {
        Blizzard { start, direction }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Walker {
    pub time: i32,
    position: Position,
}

impl Walker {
    pub fn new(valley: &Valley) -> Walker {
        Walker {
            position: valley.start,
            time: 0,
        }
    }

    fn next_moves(&self, valley: &mut Valley) -> Vec<Position> {
        let moves = &[(0, -1), (1, 0), (0, 1), (-1, 0), (0, 0)];
        moves
            .iter()
            .map(|(dx, dy)| Position::new(self.position.x + dx, self.position.y + dy))
            .filter(|p| valley.is_free(*p, self.time + 1))
            .collect()
    }

    pub fn best_path(&self, valley: &mut Valley, finish: &Position) -> Option<Walker> {
        let mut open = VecDeque::new();
        let mut seen = HashSet::new();

        open.push_back(*self);
        while let Some(walker) = open.pop_front() {
            if walker.position == *finish {
                return Some(walker);
            }
            if walker.time > 20000 {
                return None;
            }

            for next_position in walker.next_moves(valley) {
                let next = Walker {
                    time: walker.time + 1,
                    position: next_position,
                };
                if seen.contains(&next) {
                    continue;
                } else {
                    open.push_back(next);
                    seen.insert(next);
                }
            }
        }
        None
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

    impl Valley {
        fn print_at_time(&self, time: i32) {
            let mut lines = vec![vec!['.'; self.width as usize]; self.height as usize];
            (0..self.width as usize).for_each(|x| {
                lines[0][x] = '#';
                lines[self.height as usize - 1][x] = '#';
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
                let pos = self.blizzard_position(b, time);
                lines[pos.y as usize][pos.x as usize] = symbol;
            });

            for line in lines {
                println!("{}", line.iter().collect::<String>());
            }
        }
    }

    impl Walker {
        fn format_valley(&self, valley: &Valley) -> String {
            let mut lines = vec![vec!['.'; valley.width as usize]; valley.height as usize];
            (0..valley.width as usize).for_each(|x| {
                lines[0][x] = '#';
                lines[valley.height as usize - 1][x] = '#';
            });
            (0..valley.height as usize).for_each(|y| {
                lines[y][0] = '#';
                lines[y][valley.width as usize - 1] = '#';
            });
            lines[valley.start.y as usize][valley.start.x as usize] = '.';
            lines[valley.finish.y as usize][valley.finish.x as usize] = '.';

            valley.blizzards.iter().for_each(|b| {
                let symbol = match b.direction {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Right => '>',
                    Direction::Left => '<',
                };
                let pos = valley.blizzard_position(b, self.time);
                let (x, y) = pos.into();
                let (x, y) = (x as usize, y as usize);

                lines[y][x] = match lines[y][x] {
                    '.' => symbol,
                    '2' => '3',
                    '3' => '4',
                    '^' => '2',
                    'v' => '2',
                    '>' => '2',
                    '<' => '2',
                    _ => lines[y][x],
                }
            });

            // add the walker
            lines[self.position.y as usize][self.position.x as usize] = 'E';

            lines
                .iter()
                .map(|line| line.iter().collect::<String>())
                .fold("".to_string(), |acc, s| acc + "\n" + &s)
        }

        pub(crate) fn walk(&self, dir: &Direction) -> Walker {
            let (dx, dy) = match dir {
                Direction::Up => (0, -1),
                Direction::Right => (1, 0),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
            };
            Walker {
                time: self.time + 1,
                position: Position::new(self.position.x + dx, self.position.y + dy),
            }
        }

        pub(crate) fn wait(&self) -> Walker {
            Walker {
                time: self.time + 1,
                position: self.position,
            }
        }
    }

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
            blizzard_cache: HashMap::new(),
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
        let mut model = input.parse::<Valley>().unwrap();
        let expected = vec![
            vec![Position::new(1, 2), Position::new(4, 4)],
            vec![Position::new(2, 2), Position::new(4, 5)],
            vec![Position::new(3, 2), Position::new(4, 1)],
            vec![Position::new(4, 2), Position::new(4, 2)],
            vec![Position::new(5, 2), Position::new(4, 3)],
            vec![Position::new(1, 2), Position::new(4, 4)],
            vec![Position::new(2, 2), Position::new(4, 5)],
            vec![Position::new(3, 2), Position::new(4, 1)],
            vec![Position::new(4, 2), Position::new(4, 2)],
            vec![Position::new(5, 2), Position::new(4, 3)],
        ];
        let expected = expected
            .into_iter()
            .map(|v| v.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        let actual = (0..10)
            .map(|i| model.blizzards_positions(i))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_has_blizzard() {
        let mut model = SIMPLE_INPUT.parse::<Valley>().unwrap();
        assert!(!model.is_free(Position::new(1, 2), 0));
        assert!(model.is_free(Position::new(1, 2), 1));
        assert!(!model.is_free(Position::new(2, 2), 1));
        assert!(!model.is_free(Position::new(4, 2), 3));
    }

    #[test]
    fn test_walker_moves() {
        let mut model = SIMPLE_INPUT.parse::<Valley>().unwrap();
        let expected = vec![Position::new(1, 0), Position::new(1, 1)];
        let walker = Walker::new(&model);
        let mut actual = walker.next_moves(&mut model);
        actual.sort();
        assert_eq!(actual, expected);
        let walker = walker.walk(&Direction::Down);
        let actual = walker.next_moves(&mut model);
        let expected = vec![
            Position::new(1, 0),
            Position::new(2, 1),
            Position::new(1, 2),
            Position::new(1, 1),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_shortest_path() {
        let mut model = SIMPLE_INPUT.parse::<Valley>().unwrap();
        let walker = Walker::new(&model);
        let finish = model.finish;
        let actual = walker.best_path(&mut model, &finish);
        assert_eq!(actual.unwrap().time, 10);
    }

    const COMPLEX_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_complex_shortest_path() {
        let mut model = COMPLEX_INPUT.parse::<Valley>().unwrap();
        let walker = Walker::new(&model);
        let finish = model.finish;
        let actual = walker.best_path(&mut model, &finish).unwrap().time;
        assert_eq!(actual, 18);
    }

    #[test]
    fn test_complex_next_moves() {
        let mut model = COMPLEX_INPUT.parse::<Valley>().unwrap();
        let walker = Walker {
            time: 1,
            position: Position::new(1, 0),
        };
        model.print_at_time(0);
        model.print_at_time(1);
        model.print_at_time(2);
        let mut actual = walker.next_moves(&mut model);
        actual.sort();
        let expected = vec![Position::new(1, 0), Position::new(1, 1)];
        assert_eq!(actual, expected);
    }

    const SEQUENCE_INPUT: &str = "Initial state:
#E######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#

Minute 1, move down:
#.######
#E>3.<.#
#<..<<.#
#>2.22.#
#>v..^<#
######.#

Minute 2, move down:
#.######
#.2>2..#
#E^22^<#
#.>2.^>#
#.>..<.#
######.#

Minute 3, wait:
#.######
#<^<22.#
#E2<.2.#
#><2>..#
#..><..#
######.#

Minute 4, move up:
#.######
#E<..22#
#<<.<..#
#<2.>>.#
#.^22^.#
######.#

Minute 5, move right:
#.######
#2Ev.<>#
#<.<..<#
#.^>^22#
#.2..2.#
######.#

Minute 6, move right:
#.######
#>2E<.<#
#.2v^2<#
#>..>2>#
#<....>#
######.#

Minute 7, move down:
#.######
#.22^2.#
#<vE<2.#
#>>v<>.#
#>....<#
######.#

Minute 8, move left:
#.######
#.<>2^.#
#.E<<.<#
#.22..>#
#.2v^2.#
######.#

Minute 9, move up:
#.######
#<E2>>.#
#.<<.<.#
#>2>2^.#
#.v><^.#
######.#

Minute 10, move right:
#.######
#.2E.>2#
#<2v2^.#
#<>.>2.#
#..<>..#
######.#

Minute 11, wait:
#.######
#2^E^2>#
#<v<.^<#
#..2.>2#
#.<..>.#
######.#

Minute 12, move down:
#.######
#>>.<^<#
#.<E.<<#
#>v.><>#
#<^v^^>#
######.#

Minute 13, move down:
#.######
#.>3.<.#
#<..<<.#
#>2E22.#
#>v..^<#
######.#

Minute 14, move right:
#.######
#.2>2..#
#.^22^<#
#.>2E^>#
#.>..<.#
######.#

Minute 15, move right:
#.######
#<^<22.#
#.2<.2.#
#><2>E.#
#..><..#
######.#

Minute 16, move right:
#.######
#.<..22#
#<<.<..#
#<2.>>E#
#.^22^.#
######.#

Minute 17, move down:
#.######
#2.v.<>#
#<.<..<#
#.^>^22#
#.2..2E#
######.#

Minute 18, move down:
#.######
#>2.<.<#
#.2v^2<#
#>..>2>#
#<....>#
######E#";

    #[test]
    fn test_sequence() {
        let expected: Vec<&str> = SEQUENCE_INPUT.lines().collect();
        let expected: Vec<_> = expected.as_slice().chunks(8).collect();

        let expected: Vec<_> = expected
            .into_iter()
            .map(|lines| {
                let head = lines[0];
                let grid: String = (1..=6)
                    .map(|i| lines[i])
                    .fold("".to_string(), |acc, s| acc + "\n" + s);
                let parts: Vec<&str> = head.split(' ').collect();
                let time: i32 = parts[1].trim_end_matches(',').parse().unwrap_or(0);
                let direction: Option<Direction> = if parts.len() < 4 {
                    None
                } else {
                    Some(parts[3].trim_end_matches(':').parse().unwrap())
                };

                (time, direction, grid)
            })
            .collect();

        let initial = expected[0].2.clone();
        let mut model = initial.parse::<Valley>().unwrap();
        let mut walker = Walker {
            time: 0,
            position: model.start,
        };
        let mut next_positions = walker.next_moves(&mut model);
        expected
            .iter()
            .enumerate()
            .for_each(|(i, (_, direction, grid))| {
                if i > 0 {
                    if let Some(dir) = direction {
                        walker = walker.walk(dir);
                        assert!(
                            next_positions.contains(&walker.position),
                            "walker should be in next positions"
                        );
                        next_positions = walker.next_moves(&mut model);
                    } else {
                        walker = walker.wait();
                        assert!(
                            next_positions.contains(&walker.position),
                            "walker should be in next positions"
                        );
                        next_positions = walker.next_moves(&mut model);
                    }
                }
                let actual = walker.format_valley(&model);
                assert_eq!(&actual, grid);
            });
    }
}
