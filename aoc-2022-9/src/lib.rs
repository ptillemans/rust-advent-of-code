use std::{str::FromStr, cmp::max, collections::HashSet};
use nom::{
    IResult,Parser,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    error::ErrorKind,
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub motions: Vec<Motion>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Motion {
    pub direction: Direction,
    pub distance: usize,
}


#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser(s)
            .map(|(_, motions)| InputModel { motions })
            .map_err(|_| AocError::ParseError)
    }
}

fn parser(s: &str) -> IResult<&str, Vec<Motion>> {
    separated_list1(
        newline,
        map_res(
            separated_pair(
                map_res(
                    nom::character::complete::one_of("UDLR"),
                    |c| match c {
                        'U' => Ok(Direction::Up),
                        'D' => Ok(Direction::Down),
                        'L' => Ok(Direction::Left),
                        'R' => Ok(Direction::Right),
                        _ => Err(()),
                    },
                ),
                space1,
                map_res(digit1, |s: &str| s.parse::<usize>()),
            ),
            |(direction, distance)| Ok::<Motion, ErrorKind>(Motion{direction, distance})
        )
    ).parse(s)
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn step_to(&self, direction: &Direction) -> Position {
        let Position{x, y} = self;
        match direction {
            Direction::Up => Position { x: *x, y: *y + 1 },
            Direction::Down => Position { x: *x, y: *y - 1 },
            Direction::Left => Position { x: *x - 1, y: *y },
            Direction::Right => Position { x: *x + 1, y: *y },
        }
    }

    pub fn distance(&self, other: &Self) -> usize {
        max((other.x - self.x).abs(), (other.y - self.y).abs()) as usize
    }

    pub(crate) fn catch_up(&self, head: &Position) -> Position {
        let dx = head.x - self.x;
        let dy = head.y - self.y;

        if max(dx.abs(), dy.abs()) <= 1 { 
            self.clone()
        } else {
            Position {
                x: self.x + sign(dx),
                y: self.y + sign(dy),
            }
        }
            
    }
}

fn sign(x: isize) -> isize {
    match x.cmp(&0) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rope {
    pub knots: Vec<Position>,
}


impl Rope {

    pub fn new(n: usize) -> Rope {
        Rope { knots: vec![Position { x: 0, y: 0 }; n] }
    }

    pub fn step(&self, direction: &Direction) -> Self {
        let mut new_knots = self.knots.clone();
        new_knots[0] = self.knots[0].step_to(direction);
        for i in 1..new_knots.len() {
            new_knots[i] = new_knots[i].catch_up(&new_knots[i-1]);
        }
        Rope { knots: new_knots }
    }

    pub fn tail(&self) -> Position {
        self.knots.last().unwrap().clone()
    }

}


pub fn tail_coverage(rope: &Rope, motions: &Vec<Motion>) -> HashSet<Position> {  

    let mut tail_set = HashSet::new();

    let mut rope = rope.clone();

    for motion in motions {
        for _ in 0..motion.distance {
            rope = rope.step(&motion.direction);
            tail_set.insert(rope.tail().clone());
        }
    }

    tail_set
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    pub fn input_data() -> InputModel {
        let motions = vec![
            Motion{direction: Direction::Right, distance: 4},
            Motion{direction: Direction::Up, distance: 4},
            Motion{direction: Direction::Left, distance: 3},
            Motion{direction: Direction::Down, distance: 1},
            Motion{direction: Direction::Right, distance: 4},
            Motion{direction: Direction::Down, distance: 1},
            Motion{direction: Direction::Left, distance: 5},
            Motion{direction: Direction::Right, distance: 2},
        ];
        InputModel { motions }
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_step() {
        let rope = Rope {
            knots: vec![ Position { x: 0, y: 0 }, Position { x: 0, y: 0 },]
        };
        let expected = Rope {
            knots: vec![ Position { x: 1, y: 0 }, Position { x: 0, y: 0 },]
        };
        let actual = rope.step(&Direction::Right);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2_r_step() {
        let rope = Rope {
            knots: vec![ Position { x: 0, y: 0 }, Position { x: 0, y: 0 },]
        };
        let expected = Rope {
            knots: vec![ Position { x: 2, y: 0 }, Position { x: 1, y: 0 },]
        };
        let rope = rope.step(&Direction::Right);
        let actual = rope.step(&Direction::Right);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_r_u_step() {
        let rope = Rope {
            knots: vec![ Position { x: 0, y: 0 }, Position { x: 0, y: 0 },]
        };
        let expected = Rope {
            knots: vec![ Position { x: 1, y: 1 }, Position { x: 0, y: 0 },]
        };
        let rope = rope.step(&Direction::Right);
        let actual = rope.step(&Direction::Up);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_tail_coverage() {
        let rope = Rope::new(2);
        let motions = input_data().motions;
        let actual = tail_coverage(&rope, &motions).len();
        let expected = 13;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_catch_up_1() {
        let samples = vec![
            (Position { x: 2, y: 0 }, Position { x: 1, y: 0 }),
            (Position { x: 0, y: 2 }, Position { x: 0, y: 1 }),
            (Position { x: 2, y: 1 }, Position { x: 1, y: 1 }),
            (Position { x: 1, y: 2 }, Position { x: 1, y: 1 }),
        ];

        for (head, expected) in samples {
            let actual = Position{ x: 0, y: 0 }.catch_up(&head);
            assert_eq!(actual, expected);
        }
    }
}

