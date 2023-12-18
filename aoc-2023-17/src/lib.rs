use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    i8,
    str::FromStr,
};

use aoc_common::{
    direction::Direction,
    grid::Grid,
    position::{self, Position},
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub city: City,
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
        s.parse::<City>()
            .map_err(|_| AocError::ParseError)
            .map(|city| InputModel { city })
    }
}

pub type Heat = i32;
pub type City = Grid<Heat>;

const MAX_CONSECUTIVE_MOVES: usize = 3;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Node {
    position: Position,
    previous: Direction,
}

pub fn next_moves(
    grid: &City,
    current: &Node,
    current_heat: Heat,
    min_steps: usize,
    max_steps: usize,
) -> BinaryHeap<(Heat, Node)> {
    Direction::iter()
        .filter(|&dir| dir != current.previous && dir != current.previous.inverse())
        .flat_map(move |dir| {
            (1..=max_steps)
                .map(move |i| (i, dir.steps(current.position, i).clone()))
                .filter_map(|(i, p)| grid.get(p).map(|h| (i, p, h)))
                .scan(current_heat, move |heat, (i, position, h)| {
                    *heat -= h; // use negative heat to allow use of BinaryHeap as prioQ
                    Some((
                        i,
                        *heat,
                        Node {
                            position,
                            previous: dir.clone(),
                        },
                    ))
                })
                .filter(|(i, _, _)| *i >= min_steps)
                .map(|(_, heat, state)| (heat, state))
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn find_path(grid: &City, min_steps: usize, max_steps: usize) -> Result<Heat, AocError> {
    let start = (0, 0).into();
    let end: Position = (grid.width() as i32 - 1, grid.height() as i32 - 1).into();

    let mut best = HashMap::<Node, Heat>::with_capacity(10000);
    let mut queue = BinaryHeap::<(Heat, Node)>::with_capacity(10000);
    let next_states =
        |current: &Node, current_heat| next_moves(grid, current, current_heat, min_steps, max_steps);

    queue.append(&mut next_states(
        &Node {
            position: start,
            previous: Direction::East,
        },
        0,
    ));
    queue.append(&mut next_states(
        &Node {
            position: start,
            previous: Direction::South,
        },
        0,
    ));

    while let Some((heat, node))= queue.pop() {
        if node.position == end {
            return Ok(-heat);
        }

        if let Some(&best_heat) = best.get(&node) {
            if best_heat > heat {
                continue;
            }
        }

        next_states(&node, heat).iter().for_each(|(heat, next)| {
            let est_heat = *heat - next.position.manhattan(&end) as Heat;
            if let Some(&best_est_heat) = best.get(next) {
                if best_est_heat < est_heat {
                    best.insert(*next, est_heat);
                    queue.push((*heat, *next));
                }
            } else {
                best.insert(*next, est_heat);
                queue.push((*heat, *next));
            }
        });
    }
    Err(AocError::NoSolution)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    pub fn input_data() -> InputModel {
        let city = TEST_INPUT.parse::<City>().unwrap();
        InputModel { city }
    }

    #[test]
    fn test_next_moves() {
        let city = input_data().city;
        let current = Node {
            position: Position::new(0, 0),
            previous: Direction::East,
        };
        let next = next_moves(&city, &current, 0, 1, 3)
            .iter()
            .map(|(heat, node)| (node.position, -*heat))
            .collect::<Vec<_>>();
        println!("{:?}", next);
        assert!(next.contains(&(Position::new(0, 1), 3)));
        assert!(next.contains(&(Position::new(0, 2), 6)));
        assert!(next.contains(&(Position::new(0, 3), 9)));
        assert_eq!(next.len(), 3);

        let current = Node {
            position: Position::new(0, 0),
            previous: Direction::South,
        };
        let next = next_moves(&city, &current, 0, 1, 3)
            .iter()
            .map(|(heat, node)| (node.position, -*heat))
            .collect::<Vec<_>>();
        assert!(next.contains(&(Position::new(1, 0), 4)));
        assert!(next.contains(&(Position::new(2, 0), 5)));
        assert!(next.contains(&(Position::new(3, 0), 8)));
        assert_eq!(next.len(), 3);

        let current = Node {
            position: Position::new(4, 7),
            previous: Direction::East,
        };
        let next = next_moves(&city, &current, 0, 1, 3)
            .iter()
            .map(|(heat, node)| (node.position, -*heat))
            .collect::<Vec<_>>();

        print!("{:?}", next);
        assert!(next.contains(&(Position::new(4, 6), 8)));
        assert!(next.contains(&(Position::new(4, 5), 13)));
        assert!(next.contains(&(Position::new(4, 4), 19)));
        assert!(next.contains(&(Position::new(4, 8), 9)));
        assert!(next.contains(&(Position::new(4, 9), 15)));
        assert!(next.contains(&(Position::new(4, 10), 21)));
        assert_eq!(next.len(), 6);
    }
}
