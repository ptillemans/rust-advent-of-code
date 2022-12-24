use std::{str::FromStr, fmt::{Display, Formatter}, collections::{BinaryHeap, HashMap, HashSet}};
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

    fn is_free(&self, pos: Position , time: i32) -> bool {
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


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Walker {
    position: Position,
    time: i32,
}

impl Walker {
    fn new(valley: &Valley) -> Walker {
        Walker {
            position: valley.start,
            time: 0,
        }
    }

    fn next_moves(&self, valley: &Valley) -> Vec<Position> {
        let (x, y) = self.position.into();
        (x - 1..=x + 1)
            .flat_map(|x| (y - 1..=y + 1).map(move |y| (x, y).into()))
            .filter(|p| valley.is_free(*p, self.time))
            .collect()
    }

    // use A* algorithm to find the shortest path
    fn best_path(&self, valley: &Valley) -> Option<Vec<Position>> {
        let mut open = BinaryHeap::new();
        let mut closed = HashSet::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();

        fn reconstruct_path(came_from: &HashMap<Position, Position>, current: Position) -> Vec<Position> {
            let mut path = vec![current];
            let mut current = current;
            while came_from.contains_key(&current) {
                current = came_from[&current];
                path.push(current);
            }
            path.reverse();
            path
        }

        open.push((0, self.position));
        g_score.insert(self.position, 0);
        f_score.insert(self.position, self.position.manhattan(&valley.finish));
        while let Some((time, current)) = open.pop() {
            println!("current: {:?}", current);
            let walker = Walker {
                position: current,
                time,
            };
            if current == valley.finish {
                return Some(reconstruct_path(&came_from, current));
            }
            closed.insert(current);
            println!("next moves: {:?}", self.next_moves(valley));
            for next in walker.next_moves(valley) {
                println!("next: {:?}", next);
                if closed.contains(&next) {
                    continue;
                }
                let tentative_g_score = g_score[&current] + 1;
                println!("tentative_g_score: {}", tentative_g_score);
                if tentative_g_score <= g_score.get(&next).copied().unwrap_or(i32::MAX) {
                    println!("add position to open");
                    came_from.insert(next, current);
                    g_score.insert(next, tentative_g_score);
                    f_score.insert(next, tentative_g_score + next.manhattan(&valley.finish));
                    open.push((f_score[&next], next));
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

    #[test]
    fn test_has_blizzard() {
        let model = SIMPLE_INPUT.parse::<Valley>().unwrap();
        assert!(!model.is_free(Position::new(1, 2), 0));
        assert!(model.is_free(Position::new(1, 2), 1));
        assert!(!model.is_free(Position::new(2, 2), 1));
        assert!(!model.is_free(Position::new(4, 2), 3));
    }

    #[test]
    fn test_walker_moves() {
        let model = SIMPLE_INPUT.parse::<Valley>().unwrap();
        let expected = vec![
            Position::new(1,0), 
            Position::new(1,1), 
            Position::new(2,1)
        ];
        let actual = Walker::new(&model).next_moves(&model);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_shortest_path() {
        let model = SIMPLE_INPUT.parse::<Valley>().unwrap();
        let mut walker = Walker::new(&model);
        let actual = walker.best_path(&model).unwrap().len();
        assert_eq!(actual, 10);
    }

    const COMPLEX_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_complex_shortest_path() {
        let model = COMPLEX_INPUT.parse::<Valley>().unwrap();
        let walker = Walker::new(&model);
        let actual = walker.best_path(&model).unwrap().len();
        assert_eq!(actual, 18);
    }
}
