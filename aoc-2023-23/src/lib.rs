use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_common::{direction::Direction, grid::Grid, position::Position};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub grid: Grid<Cell>,
    pub start: Position,
    pub end: Position,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.parse::<Grid<Cell>>()?;
        let start = grid[0]
            .iter()
            .position(|c| *c == Cell::Path)
            .ok_or(AocError::ParseError)? as i32;
        let l = grid.height() - 1;
        let end = grid[l]
            .iter()
            .position(|c| *c == Cell::Path)
            .ok_or(AocError::ParseError)? as i32;

        Ok(InputModel {
            grid,
            start: Position::new(start, 0),
            end: Position::new(end, l as i32),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Forrest,
    Path,
    Slope(Direction),
}

impl FromStr for Cell {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Path),
            "#" => Ok(Cell::Forrest),
            "^" => Ok(Cell::Slope(Direction::North)),
            ">" => Ok(Cell::Slope(Direction::East)),
            "v" => Ok(Cell::Slope(Direction::South)),
            "<" => Ok(Cell::Slope(Direction::West)),
            _ => Err(AocError::ParseError),
        }
    }
}

fn to_direction(a: &Position, b: &Position) -> Direction {
    if a.x < b.x {
        Direction::East
    } else if a.x > b.x {
        Direction::West
    } else if a.y < b.y {
        Direction::South
    } else {
        Direction::North
    }
}

pub fn find_longest_path(grid: &Grid<Cell>, start: &Position) -> usize {
    let mut todo = vec![(0, *start)];
    let mut best = HashMap::<Position, usize>::new();
    let mut came_from = HashMap::<Position, Position>::new();
    came_from.insert(*start, *start);

    while let Some((steps, pos)) = todo.pop() {
        let &prev = came_from.get(&pos).unwrap();

        let next = pos
            .neighbors()
            .into_iter()
            .filter(|&p| grid.in_bounds(p))
            .filter(|&p| p != prev)
            .filter(|&p| {
                let c = grid.get(p).unwrap();
                let dir = to_direction(&pos, &p);
                c == &Cell::Path || c == &Cell::Slope(dir)
            })
            .filter(|p| best.get(p).map(|s| *s < steps + 1).unwrap_or(true))
            .collect::<Vec<_>>();

        next.iter().for_each(|&p| {
            came_from.insert(p, pos);
            best.insert(p, steps + 1);
            todo.push((steps + 1, p));
        });
    }

    let (_, steps) = best.iter().max_by_key(|(_, steps)| *steps).unwrap();

    *steps
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pos: Position,
    next: HashMap<Direction, (Position, u32)>,
}

pub fn grid_to_graph(grid: &Grid<Cell>, start: &Position) -> Vec<Node> {
    let mut todo = vec![(*start, Direction::South)];
    let mut seen: HashSet<(Position, Direction)> =
        [(*start, Direction::South)].iter().cloned().collect();
    let mut graph = HashMap::<Position, HashMap<Direction, (Position, u32)>>::new();

    while let Some((last_pos, direction)) = todo.pop() {
        let mut last = last_pos;
        let mut pos = direction.step(last_pos);
        let mut next: Vec<Position>;
        let mut steps = 1;
        // walk the trail till next fork
        loop {
            next = pos
                .neighbors()
                .into_iter()
                .filter(|&p| grid.in_bounds(p))
                .filter(|&p| p != last)
                .filter(|&p| {
                    let c = grid.get(p).unwrap();
                    c == &Cell::Path || matches!(c, Cell::Slope(_))
                })
                .collect::<Vec<_>>();

            if next.len() != 1 {
                // dead end or fork
                break;
            }
            last = pos;
            pos = next[0];
            steps += 1;
        }

        graph
            .entry(last_pos)
            .or_default()
            .insert(direction, (pos, steps));

        next.iter()
            .map(|p| (pos, to_direction(&pos, p)))
            .for_each(|state| {
                if seen.insert(state) {
                    todo.push(state);
                }
            })
    }

    graph
        .iter()
        .map(|(pos, next)| Node {
            pos: *pos,
            next: next.clone(),
        })
        .collect()
}

pub fn optimize_graph(nodes: &[Node]) -> HashMap<Position, Vec<(Position, u32)>> {
    let mut graph = nodes_to_graph(nodes);

    println!("unoptimized: {}", graph.len());
    let nodes_2_edges = {
        graph.clone()
            .into_iter()
            .filter(|(_, edges)| edges.len() == 2)
            .collect::<Vec<_>>()
    };

    for (pos, edges) in nodes_2_edges {
        graph.remove(&pos);
        let edge0 = edges[0];
        let edge1 = edges[1];
        graph.entry(edge0.0).and_modify(|ps| {
            let e = ps.iter().find(|(p, _)| *p == pos).unwrap().clone();
            ps.retain(|(p, _)| *p != pos);
            ps.push((edge1.0, e.1 + edge1.1));
        });
        graph.entry(edge1.0).and_modify(|ps| {
            let e = ps.iter().find(|(p, _)| *p == pos).unwrap().clone();
            ps.retain(|(p, _)| *p != pos);
            ps.push((edge0.0, e.1 + edge0.1));
        });
    };

    println!("optimized: {}", graph.len());

    graph
}

fn nodes_to_graph(nodes: &[Node]) -> HashMap<Position, Vec<(Position, u32)>> {
    nodes
        .iter()
        .map(|n| (n.pos, n.next.values().cloned().collect::<Vec<_>>()))
        .collect()
}

pub fn find_longest_path2(nodes: &[Node], start: &Position, end: &Position) -> u32 {
    let graph = optimize_graph(nodes);
    let start = *start;
    let end = *end;
    let mut todo = Vec::with_capacity(10000);
    todo.push((0, vec![start]));

    let mut solutions = Vec::with_capacity(100000);

    while let Some((steps, path)) = todo.pop() {
        let pos = path.iter().last().unwrap().clone();

        if pos == end {
            solutions.push(steps)
        }

        if let Some(next) = graph.get(&pos) {
            let next = next
                .iter()
                .filter(|(p, _)| !path.contains(p))
                .cloned()
                .collect::<Vec<(Position, u32)>>();

            if let Some((_, l)) = next.iter().find(|(p, _)| *p == end) {
                solutions.push(steps + l)
            } else {
                next.into_iter().for_each(|(p, l)| {
                    let mut path = path.clone();
                    path.push(p);
                    todo.push((steps + l, path));
                });
            }
        }
    }

    println!("len solutions: {}", solutions.len());
    solutions.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = InputModel {
            grid: TEST_INPUT.parse().unwrap(),
            start: Position::new(1, 0),
            end: Position::new(21, 22),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_longest_path() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let steps = find_longest_path(&input.grid, &input.start);

        assert_eq!(steps, 94);
    }

    #[test]
    fn test_longest_path2() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let nodes = grid_to_graph(&input.grid, &input.start);
        let steps = find_longest_path2(&nodes, &input.start, &input.end);

        assert_eq!(steps, 154);
    }

    #[test]
    fn test_to_graph() {
        let InputModel {
            grid,
            start,
            end: _,
        } = TEST_INPUT.parse::<InputModel>().unwrap();
        let graph = grid_to_graph(&grid, &start);

        assert_eq!(graph.len(), 8)
    }
}
