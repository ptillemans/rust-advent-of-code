use std::str::FromStr;
use aoc_common::position::*;


pub type InputModel = Grid; 

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution found")]
    NoSolution,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines()
            .map(|line| line.bytes().collect())
            .collect::<Vec<Vec<u8>>>();
        Ok(InputModel { grid })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid{ 
    pub grid: Vec<Vec<u8>> 
}

impl Grid {

    fn get(&self, pos: Position) -> Option<u8> {
        self.grid
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize).copied())
    }

}

fn find_char(input: &Grid, target: u8) -> Position {
    input.grid.iter()
        .enumerate()
        .find(|(_, row)| row.contains(&target))
        .map(|(y, row)| ( row.iter().position(|&c| c == target).unwrap() as i32, y as i32 ).into())
        .unwrap()
}

pub fn find_start(input: &Grid) -> Position {
    find_char(input, b'S' )
}

pub fn find_end(input: &Grid) -> Position {
    find_char(input, b'E')
}

pub fn next_moves(grid: &Grid, current: &Position) -> Vec<Position> {
    let possible_moves: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let current_height = grid.grid[current.y as usize][current.x as usize];
    // ensure we can leave start by forcing start to be a higher value than the surrounding tiles
    let current_height = if current_height == b'S' { b'z' + 1} else { current_height };  
    possible_moves.into_iter()
        .map(|m| *current + m.into())
        .filter(|&pos| grid.get(pos).is_some())
        .filter(|&pos| {
            let pos_height = grid.get(pos).unwrap();
            let pos_height = if pos_height == b'E' { b'z' + 1 } else { pos_height };
            pos_height <= current_height + 1 
        })
        .collect()
}


// use the A* algorithm to find the shortest path between two points
pub fn shortest_path(grid: &Grid, start: &Position, end: &Position) -> Result<Vec<Position>, AocError> {
    let mut open_set = vec![start.clone()];
    let mut came_from = std::collections::HashMap::new();
    let mut g_score = std::collections::HashMap::new();
    let mut f_score = std::collections::HashMap::new();
    g_score.insert(start.clone(), 0);
    f_score.insert(start.clone(), start.manhattan(end));

    while !open_set.is_empty() {
        let current = open_set.iter()
            .min_by_key(|pos| f_score.get(pos).unwrap())
            .unwrap()
            .clone();
        if current == *end {
            return Ok(reconstruct_path(&came_from, current));
        }
        open_set.retain(|pos| pos != &current);
        for neighbor in next_moves(grid, &current) {
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            if g_score.get(&neighbor).is_none() || tentative_g_score < *g_score.get(&neighbor).unwrap() {
                came_from.insert(neighbor.clone(), current.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                f_score.insert(neighbor.clone(), tentative_g_score + neighbor.manhattan(end));
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }
    Err(AocError::NoSolution)
}

fn reconstruct_path(came_from: &std::collections::HashMap<Position, Position>, current: Position) -> Vec<Position> {
    let mut total_path = vec![current];
    let mut current = current;
    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap().clone();
        total_path.push(current);
    }
    total_path.reverse();
    total_path
}

pub fn test_data() -> InputModel {
    InputModel {
        grid: vec![
            "Sabqponm".bytes().collect(),
            "abcryxxl".bytes().collect(),
            "accszExk".bytes().collect(),
            "acctuvwj".bytes().collect(),
            "abdefghi".bytes().collect(),
        ],
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = test_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_start() {
        let input = test_data();
        let actual = find_start(&input);
        let expected = (0, 0).into();

        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_find_end() {
        let input = test_data();
        let actual = find_end(&input);
        let expected = (5, 2).into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_moves() {
        let input = test_data();
        let actual = next_moves(&input, &(0, 0).into());
        let expected = vec![(1, 0).into(), (0, 1).into()];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_shortest_path() {
        let input = test_data();
        let path = shortest_path(&input, &(0, 0).into(), &(5, 2).into());
        let expected = 31;

        println!("Path: {:?}", path);

        let actual = path.unwrap().len() - 1;

        assert_eq!(actual, expected);
    }
}
