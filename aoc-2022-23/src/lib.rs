use aoc_common::position::Position;
use std::{collections::{HashMap, HashSet}, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub elves: Elves,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| match c {
                    '#' => Ok(Some(Position::new(x as i32, y as i32))),
                    '.' => Ok(None),
                    _ => Err(AocError::ParseError),
                })
            })
            .collect::<Result<Vec<Option<Position>>, AocError>>()
            .map(|positions| {
                positions
                    .iter()
                    .filter_map(|p| *p)
                    .collect::<HashSet<Position>>()
            })
            .map(|elves| InputModel { elves })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn next_direction(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn next_position(&self, position: &Position) -> Position {
        match self {
            Direction::North => Position::new(position.x, position.y - 1),
            Direction::South => Position::new(position.x, position.y + 1),
            Direction::West => Position::new(position.x - 1, position.y),
            Direction::East => Position::new(position.x + 1, position.y),
        }
    }
    // retrunt the position of the next cell in the given direction
    // and the 2 neighbours of the next cell
    fn scan_position(&self, position: &Position) -> Vec<Position> {
        let next_position = self.next_position(position);
        
        match self {
            Direction::North => vec![
                next_position,
                Direction::East.next_position(&next_position),
                Direction::West.next_position(&next_position),
            ],
            Direction::South => vec![
                next_position,
                Direction::East.next_position(&next_position),
                Direction::West.next_position(&next_position),
            ],
            Direction::West => vec![
                next_position,
                Direction::North.next_position(&next_position),
                Direction::South.next_position(&next_position),
            ],
            Direction::East => vec![
                next_position,
                Direction::North.next_position(&next_position),
                Direction::South.next_position(&next_position),
            ],
        }
    }
}

pub type Elves = HashSet<Position>;

fn is_direction_clear(pos: &Position, direction: &Direction, elves: &Elves) -> bool {
    direction
        .scan_position(pos)
        .iter()
        .all(|pos| !elves.contains(pos))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Proposal {
    Ok(Position),
    Conflict,
}

fn has_neighbours(pos: &Position, elves: &HashSet<Position>) -> bool {
    (-1..=1).any(|dx| 
        (-1..=1).any(|dy|  {
            let n = *pos + Position::new(dx, dy);
            !(dx == 0 && dy == 0) && elves.contains(&n)
        }))
}

pub fn do_round(elves: &Elves, first_dir: &Direction) -> Elves {
    let mut to_be: HashMap<Position, Proposal> = HashMap::with_capacity(elves.len());

    // create a proposal for each elf to move in the direction
    // that is clear starting with the first direction
    elves
        .iter()
        .filter(|pos| has_neighbours(pos, elves))
        .filter_map(|pos| {
            let mut dir = *first_dir;
            (0..=3).find_map(|_| {
                if is_direction_clear(pos, &dir, elves) {
                    Some((dir.next_position(pos), *pos))
                } else {
                    dir = dir.next_direction();
                    None
                }
            })
        })
        .for_each(|(proposal, pos)| {
            to_be
                .entry(proposal)
                .and_modify(|p| {
                    *p = Proposal::Conflict;
                })
                .or_insert(Proposal::Ok(pos));
        });

    let mut elves = elves.clone();
    to_be.into_iter().for_each(|(new_pos, proposal)| {
        if let Proposal::Ok(pos) = proposal {
            elves.remove(&pos);
            elves.insert(new_pos);
        }
    });
    elves
}

fn total_area(elves: &Elves) -> i32 {
    let min_x = i32::MAX;
    let max_x = i32::MIN;
    let min_y = i32::MAX;
    let max_y = i32::MIN;

    let (min_x, max_x, min_y, max_y) = elves.iter().fold(
        (min_x, max_x, min_y, max_y),
        |(min_x, max_x, min_y, max_y), pos| {
            (
                min_x.min(pos.x),
                max_x.max(pos.x),
                min_y.min(pos.y),
                max_y.max(pos.y),
            )
        },
    );

    (max_x - min_x + 1) * (max_y - min_y + 1)
}

pub fn count_free_space(elves: &Elves) -> usize {
    total_area(elves) as usize - elves.len()
}

pub fn count_rounds_till_stable(elves: &Elves) -> usize {
    std::iter::successors(Some((elves.clone(), Direction::North)), |(elves, dir)| {
        let next_elves = do_round(elves, dir);
        if next_elves == *elves {
            None
        } else {
            Some((next_elves, dir.next_direction()))
        }
    }).count()
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = ".....
..##.
..#..
.....
..##.
.....";

    fn input_data() -> InputModel {
        InputModel {
            elves: vec![
                (Position::new(2, 1)),
                (Position::new(3, 1)),
                (Position::new(2, 2)),
                (Position::new(2, 4)),
                (Position::new(3, 4)),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
    const TEST_SEQUENCE: &[&str] = &[
        ".....
..##.
..#..
.....
..##.
.....",
        "..##.
.....
..#..
...#.
..#..
.....",
        ".....
..##.
.#...
....#
.....
..#..",
        "..#..
....#
#....
....#
.....
..#..",
    ];

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rounds() {
        let sequence = TEST_SEQUENCE
            .iter()
            .map(|s| s.parse::<InputModel>().unwrap())
            .map(|m| m.elves);
        let pairs = sequence.clone().zip(sequence.skip(1));

        let mut direction = Direction::North;
        for (actual, expected) in pairs {
            let actual = do_round(&actual, &direction);
            assert_eq!(actual, expected);

            direction = direction.next_direction();
        }
    }

    #[test]
    fn test_large_rounds() {
        let sequence = LARGE_TEST_SEQUENCE
            .iter()
            .map(|s| s.parse::<InputModel>().unwrap())
            .map(|m| m.elves);
        let pairs = sequence.clone().zip(sequence.skip(1));

        let mut direction = Direction::North;
        for (actual, expected) in pairs {
            let actual = do_round(&actual, &direction);
            let mut actual = actual.iter().cloned().collect::<Vec<_>>();
            actual.sort();
            let mut expected = expected.iter().cloned().collect::<Vec<_>>();
            expected.sort();
            assert_eq!(actual, expected);

            direction = direction.next_direction();
        }
    }

    #[test]
    fn test_count_rounds_till_stable() {
        let actual = count_rounds_till_stable(&input_data().elves);
        assert_eq!(actual, 4);
    }

    const LARGE_TEST_SEQUENCE: &[&str] = &[
"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............",
"..............
.......#......
.....#...#....
...#..#.#.....
.......#..#...
....#.#.##....
..#..#.#......
..#.#.#.##....
..............
....#..#......
..............
..............",
"..............
.......#......
....#.....#...
...#..#.#.....
.......#...#..
...#..#.#.....
.#...#.#.#....
..............
..#.#.#.##....
....#..#......
..............
..............",
"..............
.......#......
.....#....#...
..#..#...#....
.......#...#..
...#..#.#.....
.#..#.....#...
.......##.....
..##.#....#...
...#..........
.......#......
..............",
"..............
.......#......
......#....#..
..#...##......
...#.....#.#..
.........#....
.#...###..#...
..#......#....
....##....#...
....#.........
.......#......
..............",
".......#......
..............
..#..#.....#..
.........#....
......##...#..
.#.#.####.....
...........#..
....##..#.....
..#...........
..........#...
....#..#......
..............",
];

}
