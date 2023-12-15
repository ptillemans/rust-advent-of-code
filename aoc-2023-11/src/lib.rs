use aoc_common::position::Position;
use std::{str::FromStr, collections::HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub galaxies: Vec<Position>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| Position::new(x as i32, y as i32))
            })
            .collect::<Vec<_>>();
        Ok(InputModel { galaxies })
    }
}

fn empty_rows(input: &InputModel) -> HashSet<i32> {
    let rows: Vec<i32> = input
        .galaxies
        .iter()
        .map(|Position { x: _, y }| *y)
        .collect::<Vec<_>>();

    let row_max: i32 = rows.iter().cloned().max().unwrap();

    (0..=row_max).filter(|r| !rows.contains(r)).collect()
}

fn empty_cols(input: &InputModel) -> HashSet<i32> {
    let columns: Vec<i32> = input
        .galaxies
        .iter()
        .map(|Position { x, y: _ }| *x)
        .collect::<Vec<_>>();

    let column_max: i32 = columns.iter().cloned().max().unwrap();

    (0..=column_max).filter(|r| !columns.contains(r)).collect()
}

pub type EmptySpace = (HashSet<i32>, HashSet<i32>);

pub fn empty_space(input: &InputModel) -> EmptySpace {
    (empty_cols(input), empty_rows(input))
}

pub fn position_after_expansion(pos: &Position, empty_space: &EmptySpace, age: i32) -> Position {
    let (x, y) = (*pos).into();
    let (empty_xs, empty_ys) = empty_space;

    let dx = (0..x).filter(|x| empty_xs.contains(x)).count() as i32;
    let dy = (0..y).filter(|y| empty_ys.contains(y)).count() as i32;

    *pos + (dx * (age - 1), dy * (age - 1)).into()
}


#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    fn input_data() -> InputModel {
        InputModel {
            galaxies: vec![
                Position::new(3, 0),
                Position::new(7, 1),
                Position::new(0, 2),
                Position::new(6, 4),
                Position::new(1, 5),
                Position::new(9, 6),
                Position::new(7, 8),
                Position::new(0, 9),
                Position::new(4, 9),
            ],
        }
    }

    #[test]
    fn test_from_str() {
        let actual: InputModel = TEST_INPUT.parse().unwrap();

        let expected = input_data();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_empty_rows() {
        let input = input_data();
        let actual = empty_rows(&input);
        let expected = vec![3, 7].into_iter().collect::<HashSet<_>>();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_empty_cols() {
        let input = input_data();
        let actual = empty_cols(&input);
        let expected = vec![2, 5, 8].into_iter().collect::<HashSet<_>>();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_empty_space() {
        let input = input_data();
        let actual = empty_space(&input);
        let expected = (
            vec![2, 5, 8].into_iter().collect::<HashSet<_>>(),
            vec![3, 7].into_iter().collect::<HashSet<_>>()
        );

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_position_after_expansion() {
        let input = input_data();
        let empty_space = empty_space(&input);

        let test_cases: Vec<((i32, i32), (i32,i32))>= vec![
            ((0,0), (0,0)),
            ((6,0), (8,0)),
            ((0,4), (0,5)),
            ((9,9), (12,11)),
        ];
        for test_case in test_cases {
            let pos = test_case.0.into(); 
            let actual = position_after_expansion(&pos, &empty_space, 2);
            let expected = test_case.1.into();
            assert_eq!(actual, expected)
        }

        let test_cases: Vec<((i32, i32), (i32,i32))>= vec![
            ((0,0), (0,0)),
            ((6,0), (2000004,0)),
            ((0,4), (0,1000003)),
            ((9,9), (3000006,2000007)),
        ];
        for test_case in test_cases {
            let pos = test_case.0.into(); 
            let actual = position_after_expansion(&pos, &empty_space, 1_000_000);
            let expected = test_case.1.into();
            assert_eq!(actual, expected)
        }
    }

}
