use aoc_common::{grid::Grid, position::Position};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub garden: Garden,
    pub start: Position,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("Found no solution")]
    NoSolution,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut garden = s.parse::<Garden>()?;
        let start = garden
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &tile)| tile == Tile::Start)
                    .map(move |(x, _)| Position::new(x as i32, y as i32))
            })
            .next()
            .ok_or(AocError::ParseError)?;
        garden.set(start, Tile::Garden);
        Ok(InputModel { garden, start })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Garden,
    Rock,
    Start,
}

impl FromStr for Tile {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Garden),
            "#" => Ok(Tile::Rock),
            "S" => Ok(Tile::Start),
            _ => Err(AocError::ParseError),
        }
    }
}

type Garden = Grid<Tile>;

fn reachable(garden: &Garden, current: &HashSet<Position>) -> HashSet<Position> {
    current
        .iter()
        .flat_map(|&pos| {
            pos.neighbors().into_iter().filter(|&new_pos| {
                garden
                    .get(new_pos)
                    .map(|&tile| tile == Tile::Garden)
                    .unwrap_or(false)
            })
        })
        .collect()
}

pub fn reachable_in_steps(garden: &Garden, start: Position, steps: usize) -> HashSet<Position> {
    let mut current = HashSet::new();
    current.insert(start);

    for _ in 0..steps {
        current = reachable(garden, &current);
    }

    current
}

fn get_cyclic(garden: &Garden, pos: Position) -> Option<&Tile> {
    let (x, y) = (
        pos.x.rem_euclid(garden.width() as i32),
        pos.y.rem_euclid(garden.height() as i32),
    );
    let npos = Position::new(x, y);
    garden.get(npos)
}

pub fn cyclic_reachable(garden: &Garden, current: &HashSet<Position>) -> HashSet<Position> {
    current
        .iter()
        .flat_map(|&pos| {
            pos.neighbors().into_iter().filter(|&new_pos| {
                get_cyclic(garden, new_pos)
                    .map(|&tile| tile == Tile::Garden)
                    .unwrap_or(false)
            })
        })
        .collect()
}

pub fn cyclic_in_steps(garden: &Garden, start: Position, steps: usize) -> HashSet<Position> {
    let mut current = HashSet::new();
    current.insert(start);

    for _ in 0..steps {
        current = cyclic_reachable(garden, &current);
    }

    current
}

pub fn count_cis(garden: &Garden, start: Position, steps: usize) -> usize {
    let mut current = HashSet::new();
    current.insert(start);

    for _ in 0..steps {
        current = cyclic_reachable(garden, &current);
    }

    current.len()
}

pub fn simulate_steps(garden: &Garden, start: Position, steps: usize) -> Vec<usize> {
    let mut current = HashSet::new();
    current.insert(start);

    (0..steps)
        .map(|i| {
            if (i % 100) == 0 {
                println!("step {}", i);
            }
            current = cyclic_reachable(garden, &current);
            current.len()
        })
        .collect()
}


pub fn find_cycle(garden: &Garden, start: Position) -> Result<(Vec<(usize, i32, i32, i32)>, usize, usize), AocError> {
    let sim = simulate_steps(garden, start, garden.width() * 20)
        .into_iter()
        .scan((0 as usize, 0 as i32, 0 as i32, 0 as i32), |state, l| {
            let (ll, d1l, d2l, d3l) = *state;
            let d1 = (l - ll) as i32;
            let d2 = d1 - d1l;
            let d3 = d2 - d2l;
            *state = (l, d1, d2, d3);
            Some(*state)
        })
        .collect::<Vec<_>>();
    let l = sim.len();

    println!("sim: {:?}", sim);

    let cycle = (10..l/3)
        .find(|&cycle| (0..cycle).all(|i| sim[i + cycle ].3 == sim[i + 2*cycle].3))
        .ok_or(AocError::NoSolution)?;

    // backtrack
    let toofar = (0..cycle)
        .find(|i| sim[cycle - i - 1].3 != sim[2*cycle - i - 1].3)
        .ok_or(AocError::NoSolution)?;

    let offset = cycle - toofar;
    let sample = sim[0..offset + cycle].to_vec();
    Ok((sample, cycle, offset))
        
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    use super::*;

    #[test]
    fn test_parse() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();

        assert_eq!(input.garden.width(), 11);
        assert_eq!(input.garden.height(), 11);
        assert_eq!(input.start, Position::new(5, 5));

        assert_eq!(input.garden.get(Position::new(4, 5)), Some(&Tile::Garden));
    }

    #[test]
    fn test_reachable() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let reachable = reachable(&input.garden, &vec![input.start].into_iter().collect());

        assert_eq!(reachable.len(), 2);
        assert!(reachable.contains(&Position::new(4, 5)));
        assert!(reachable.contains(&Position::new(5, 4)));
    }

    #[test]
    fn test_reachable_in_steps() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let reachable = reachable_in_steps(&input.garden, input.start, 6);

        assert_eq!(reachable.len(), 16);
    }

    #[test]
    fn test_cyclic_reachable_in_steps() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let reachable = cyclic_in_steps(&input.garden, input.start, 6);
        assert_eq!(reachable.len(), 16);
        let reachable = cyclic_in_steps(&input.garden, input.start, 10);
        println!("reachable: {:?}", reachable);
        assert_eq!(reachable.len(), 50);
        let reachable = cyclic_in_steps(&input.garden, input.start,50);
        assert_eq!(reachable.len(), 1594);
        let reachable = cyclic_in_steps(&input.garden, input.start,100);
        assert_eq!(reachable.len(), 6536);
        // let reachable = cyclic_in_steps(&input.garden, input.start,500);
        // assert_eq!(reachable.len(), 167004);
        // let reachable = cyclic_in_steps(&input.garden, input.start,1000);
        // assert_eq!(reachable.len(), 668697);
        // let reachable = cyclic_in_steps(&input.garden, input.start,5000);
       // assert_eq!(reachable.len(), 16733044);
    }

    //[test]
    fn test_find_cycle() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let (samples, cycle, offset)= find_cycle(&input.garden, input.start).unwrap();

        assert_eq!(samples.len(), cycle + offset);
        assert_eq!(cycle, 11);
        assert_eq!(offset, 10);
        
    }
}
