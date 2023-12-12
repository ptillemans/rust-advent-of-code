use itertools::iterate;
use std::{collections::{HashMap, HashSet}, str::FromStr};
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub pipes: HashMap<Location, Section>,
}

pub type Location = (i64, i64);

pub fn valid_next_section(
    loc: &Location,
    next: &Location,
    pipes: &HashMap<Location, Section>,
) -> bool {
    if let Some(section) = pipes.get(next) {
        let section = *section;
        if section == Section::Start {
            return true;
        }

        let diff = (loc.0 - next.0, loc.1 - next.1);
        match diff {
            (0, -1) => {
                section == Section::Vertical
                    || section == Section::CornerNE
                    || section == Section::CornerNW
            }
            (0, 1) => {
                section == Section::Vertical
                    || section == Section::CornerSE
                    || section == Section::CornerSW
            }
            (1, 0) => {
                section == Section::Horizontal
                    || section == Section::CornerNE
                    || section == Section::CornerSE
            }
            (-1, 0) => {
                section == Section::Horizontal
                    || section == Section::CornerNW
                    || section == Section::CornerSW
            }
            _ => false,
        }
    } else {
        false
    }
}

pub fn connecting_pipes((x, y): &Location, pipes: &HashMap<Location, Section>) -> Vec<Location> {
    let (x, y) = (*x, *y);
    let section = pipes.get(&(x, y)).unwrap();
    let mut locations = Vec::with_capacity(4);
    match section {
        Section::Start => {
            locations.push((x, y + 1));
            locations.push((x, y - 1));
            locations.push((x + 1, y));
            locations.push((x - 1, y));
        }
        Section::Horizontal => {
            locations.push((x + 1, y));
            locations.push((x - 1, y));
        }
        Section::Vertical => {
            locations.push((x, y + 1));
            locations.push((x, y - 1));
        }
        Section::CornerNE => {
            locations.push((x, y - 1));
            locations.push((x + 1, y));
        }
        Section::CornerNW => {
            locations.push((x, y - 1));
            locations.push((x - 1, y));
        }
        Section::CornerSE => {
            locations.push((x, y + 1));
            locations.push((x + 1, y));
        }
        Section::CornerSW => {
            locations.push((x, y + 1));
            locations.push((x - 1, y));
        }
    }
    let ls = locations
        .iter()
        .filter(|loc| valid_next_section(&(x, y), &loc, &pipes))
        .cloned()
        .collect::<Vec<_>>();
    ls
}

pub fn find_path(
    loc: &Location,
    prev_pipe: &Location,
    pipes: &HashMap<Location, Section>,
) -> Option<Vec<Location>> {
    let section = pipes.get(loc).unwrap();
    let mut last = *prev_pipe;
    let mut loc = *loc;
    let mut section = *section;
    let mut path = Vec::<Location>::with_capacity(10000);
    let mut path_set = HashSet::<Location>::with_capacity(10000);
    path.push(loc);
    path_set.insert(loc);
    let mut finished = false;
    while (section != Section::Start && !finished) {
        let next_pipes = connecting_pipes(&loc, &pipes);
        let next_pipe = next_pipes
            .iter()
            .filter(|&l| *l != last && !path_set.contains(l))
            .next();
        if let Some(next_pipe) = next_pipe {
            if !path_set.contains(&next_pipe) {
                path.push(*next_pipe);
                path_set.insert(*next_pipe);
                section = *pipes.get(&next_pipe).unwrap();
                last = loc;
                loc = *next_pipe;
            } else {
                finished = true;
            }
        } else {
            finished = true;
        }
    }
    if finished {
        None
    } else {
        Some(path)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Section {
    Start,
    Horizontal,
    Vertical,
    CornerNE,
    CornerNW,
    CornerSE,
    CornerSW,
}

impl FromStr for Section {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Section::Start),
            "-" => Ok(Section::Horizontal),
            "|" => Ok(Section::Vertical),
            "L" => Ok(Section::CornerNE),
            "J" => Ok(Section::CornerNW),
            "F" => Ok(Section::CornerSE),
            "7" => Ok(Section::CornerSW),
            _ => Err(AocError::ParseError("Invalid section".to_string())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError(String),
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(move |(x, c)| {
                        let loc = (x as i64, y as i64);
                        let section = Section::from_str(&c.to_string())?;
                        Ok((loc, section))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Result<HashMap<Location, Section>, AocError>>()
            .map(|pipes| InputModel { pipes })
    }
}

pub fn find_start(pipes: &HashMap<Location, Section>) -> Location {
    pipes
        .iter()
        .find(|(_, section)| **section == Section::Start)
        .map(|(loc, _)| *loc)
        .unwrap()
}

pub fn find_loop(pipes: &HashMap<Location, Section>) -> Option<Vec<Location>> {
    let start = find_start(pipes);
    connecting_pipes(&start, pipes)
        .iter()
        .find_map(|p| find_path(p, &start, pipes))
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    const TEXT_INPUT: &str = "
.S-7.
.|.|.
.L-J.
.....";

    const TEXT_INPUT_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const TEST_INPUT_2_2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn test_parse() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;

        let expected = vec![
            ((1, 1), Section::Start),
            ((2, 1), Section::Horizontal),
            ((3, 1), Section::CornerSW),
            ((1, 2), Section::Vertical),
            ((3, 2), Section::Vertical),
            ((1, 3), Section::CornerNE),
            ((2, 3), Section::Horizontal),
            ((3, 3), Section::CornerNW),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();
        assert_eq!(pipes, expected);
    }

    #[test]
    fn test_connecting_sections() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;
        let mut locations = pipes.keys().cloned().collect::<Vec<_>>();
        locations.sort();

        let mut cs = connecting_pipes(&locations[0], &pipes);
        cs.sort();
        assert_eq!(cs, vec![locations[1], locations[3]]);

        let mut cs = connecting_pipes(&locations[1], &pipes);
        cs.sort();
        assert_eq!(cs, vec![locations[0], locations[2]]);

        let mut cs = connecting_pipes(&locations[2], &pipes);
        cs.sort();
        assert_eq!(cs, vec![locations[1], locations[4]]);

        let mut cs = connecting_pipes(&locations[3], &pipes);
        cs.sort();
        assert_eq!(cs, vec![locations[0], locations[5]]);

        let mut cs = connecting_pipes(&locations[4], &pipes);
        cs.sort();
        assert_eq!(cs, vec![locations[2], locations[7]]);

        let mut cs = connecting_pipes(&locations[5], &pipes);
        cs.sort();
        assert_eq!(cs, vec![locations[3], locations[6]]);

        let mut cs = connecting_pipes(&locations[6], &pipes);
        cs.sort();
        assert_eq!(cs, vec![locations[5], locations[7]]);

        let mut cs = connecting_pipes(&locations[7], &pipes);
        cs.sort();
        assert_eq!(cs, vec![locations[4], locations[6]]);
    }

    #[test]
    fn test_find_start() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;
        let start = find_start(&pipes);
        assert_eq!(start, (1, 1));
    }

    #[test]
    fn test_valid_next_section() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;
        let mut locations = pipes.keys().cloned().collect::<Vec<_>>();
        locations.sort();

        assert!(valid_next_section(&locations[0], &locations[1], &pipes));
        assert!(valid_next_section(&locations[0], &locations[3], &pipes));

        assert!(valid_next_section(&locations[1], &locations[0], &pipes));
        assert!(valid_next_section(&locations[1], &locations[2], &pipes));

        assert!(valid_next_section(&locations[2], &locations[1], &pipes));
        assert!(valid_next_section(&locations[2], &locations[4], &pipes));

        assert!(valid_next_section(&locations[3], &locations[0], &pipes));
        assert!(valid_next_section(&locations[3], &locations[5], &pipes));

        assert!(valid_next_section(&locations[4], &locations[2], &pipes));
        assert!(valid_next_section(&locations[4], &locations[7], &pipes));

        assert!(valid_next_section(&locations[5], &locations[3], &pipes));

        assert!(valid_next_section(&locations[6], &locations[5], &pipes));
        assert!(valid_next_section(&locations[6], &locations[7], &pipes));

        assert!(valid_next_section(&locations[7], &locations[6], &pipes));
        assert!(valid_next_section(&locations[7], &locations[4], &pipes));
    }

    #[test]
    fn test_find_path() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;
        let mut locations = pipes.keys().cloned().collect::<Vec<_>>();
        locations.sort();

        let start = find_start(&pipes);
        let next_locs = connecting_pipes(&start, &pipes);
        let next = next_locs.first().unwrap();
        let path = find_path(next, &start, &pipes);
        assert_eq!(
            path,
            Some(vec![
                locations[1],
                locations[2],
                locations[4],
                locations[7],
                locations[6],
                locations[5],
                locations[3],
                locations[0]
            ])
        );

        let input = InputModel::from_str(TEXT_INPUT_2).unwrap();
        let pipes = input.pipes;
        let start = find_start(&pipes);
        let next_pipes = connecting_pipes(&start, &pipes);
        assert_eq!(next_pipes.len(), 2);
        let path = find_path(&next_pipes[0], &start, &pipes).unwrap();
        assert_eq!(path.len(), 16);
        let path = find_path(&next_pipes[1], &start, &pipes).unwrap();
        assert_eq!(path.len(), 16);

        let input = InputModel::from_str(TEXT_INPUT_2).unwrap();
        let pipes = input.pipes;
        let start = find_start(&pipes);
        let next_pipes = connecting_pipes(&start, &pipes);
        assert_eq!(next_pipes.len(), 2);
        let path = find_path(&next_pipes[0], &start, &pipes).unwrap();
        assert_eq!(path.len(), 16);
        let path = find_path(&next_pipes[1], &start, &pipes).unwrap();
        assert_eq!(path.len(), 16);
    }

    #[test]
    fn test_find_loop() {
        let input = InputModel::from_str(TEXT_INPUT_2).unwrap();
        let pipes = input.pipes;
        let path = find_loop(&pipes).unwrap();
        assert_eq!(path.len(), 16);
    }
}
