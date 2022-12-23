use std::{str::FromStr, collections::HashMap, fmt::{Display, Formatter}};
use aoc_common::position::*;
use nom::{
    Parser,IResult,
    multi::many1,
    bytes::complete::tag,
    character::complete::digit1,
    branch::alt,
};
use ndarray::{Array1, Array2, array};
use itertools::Itertools;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("Invalid Direction")]
    DirectionError,
}
      

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Void,
    Empty,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    TurnLeft,
    TurnRight,
    Forward(i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<Direction> for i32 {
    type Error = AocError;

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::Right => Ok(0),
            Direction::Down => Ok(1),
            Direction::Left => Ok(2),
            Direction::Up => Ok(3),
        }
    }
}

impl Direction {
    fn turn(&self, move_: &Move) -> Direction {
        match move_ {
            Move::TurnLeft => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Move::TurnRight => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            _ => *self
        }
    }

}

impl Move {
   fn distance(&self) -> i32 {
       match self {
           Move::TurnLeft => 0,
           Move::TurnRight => 0,
           Move::Forward(distance) => *distance,
       }
   }
}

impl FromStr for Move {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            'L' => Ok(Move::TurnLeft),
            'R' => Ok(Move::TurnRight),
            _ => s.parse()
                .map(Move::Forward)
                .map_err(|_| AocError::ParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct Grid(Vec<Vec<Tile>>);

impl Grid {
   fn new(input: &[&str]) -> Grid {
       Grid(input.iter()
           .map(|row| 
                row.chars().map(|c| 
                    match c {
                       '#' => Tile::Wall,
                       '.' => Tile::Empty,
                       _ => Tile::Void,
                    })
                .collect::<Vec<_>>())
           .collect::<Vec<_>>())
   }

   fn get_tile(&self, position: &Position) -> Tile {
       self.0[position.y as usize][position.x as usize]
   }
}

impl FromStr for Grid {
   type Err = ();

   fn from_str(s: &str) -> Result<Self, Self::Err> {
       Ok(Grid::new(&s.lines().collect::<Vec<_>>()))
   }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for tile in row {
                match tile {
                    Tile::Void => write!(f, " ")?,
                    Tile::Empty => write!(f, ".")?,
                    Tile::Wall => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    grid: Grid,
    moves: Vec<Move>,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let parts = lines.as_slice()
            .split(|line| line.is_empty())
            .collect::<Vec<_>>();

        let grid = Grid::new(parts[0]);

        let moves = parse_moves(parts[1][0])?;
        Ok(InputModel { grid, moves })
    }
}

fn parse_moves(input: &str) -> Result<Vec<Move>, AocError> {
    let result: IResult<&str, Vec<Move>> = many1(
        alt((tag("L"), tag("R"), digit1))
            .map(|s: &str| s.parse::<Move>().unwrap())
    ).parse(input);
    result
        .map(|(_, moves)| moves)
        .map_err(|_| AocError::ParseError)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Navigator {
    grid: Grid,
    position: Position,
    x_offset: i32,
    direction: Direction,
    horiz_bounds_cache: HashMap<i32, (i32, i32)>,
    vert_bounds_cache: HashMap<i32, (i32, i32)>,
}

impl Navigator {

    fn new(grid: &Grid) -> Navigator {
        let x_offset = grid.0[0].iter()
            .enumerate()
            .find(|(_, &tile)| tile == Tile::Empty)
            .map(|(x, _)| x)
            .unwrap() as i32;
        Navigator {
            grid: grid.clone(),
            position: Position::new(0, 0),
            x_offset,
            direction: Direction::Right,
            horiz_bounds_cache: HashMap::with_capacity(grid.0.len()),
            vert_bounds_cache: HashMap::with_capacity(grid.0.len()),
        }
    }

    fn get_tile(&self, pos: Position) -> Tile {
        let (x, y) = (pos.x, pos.y);
        if y < 0 || y >= self.grid.0.len() as i32 {
            return Tile::Void
        }
        let row = &self.grid.0[y as usize];
        if x < -self.x_offset ||  x + self.x_offset >= row.len() as i32 {
            return Tile::Void
        }
        self.grid.0[pos.y as usize][(pos.x + self.x_offset) as usize]
    }

    fn horizontal_bounds(&self, pos: Position) -> (i32, i32) {
        let y = pos.y;
        let left: i32 = (-self.x_offset..=pos.x)
            .rev()
            .take_while(|x| self.get_tile(Position::new(*x,y)) != Tile::Void)
            .last()
            .unwrap();
        let right: i32 = (pos.x..self.grid.0[y as usize].len() as i32)
            .take_while(|x| self.get_tile(Position::new(*x,y)) != Tile::Void)
            .last()
            .unwrap();
        (left, right + 1)
    }
   
    fn vertical_bounds(&self, pos: Position) -> (i32, i32) {
        let x = pos.x;
        let top = (0..=pos.y as usize)
            .rev()
            .take_while(|y| self.get_tile(Position::new(x,*y as i32)) != Tile::Void)
            .last()
            .unwrap() as i32;
        let bottom = (pos.y as usize..self.grid.0.len())
            .take_while(|y| self.get_tile(Position::new(x,*y as i32)) != Tile::Void)
            .last()
            .unwrap() as i32;
        (top, bottom + 1)
    }

    fn take_step(&self) -> Self {
        let new_pos : Position = match self.direction {
            Direction::Left => {
                let (left, right) = self.horizontal_bounds(self.position);
                let new_x = (self.position.x - 1 - left).rem_euclid(right - left) + left;
                (new_x, self.position.y).into()
            },
            Direction::Right => {
                let (left, right) = self.horizontal_bounds(self.position);
                let new_x = (self.position.x + 1 - left).rem_euclid(right - left) + left;
                (new_x, self.position.y).into()
            },
            Direction::Up => {
                let (top, bottom) = self.vertical_bounds(self.position);
                let new_y = (self.position.y - 1 - top).rem_euclid(bottom - top) + top;
                (self.position.x, new_y).into()
            },
            Direction::Down => {
                let (top, bottom) = self.vertical_bounds(self.position);
                let new_y = (self.position.y + 1 - top).rem_euclid(bottom - top) + top;
                (self.position.x, new_y).into()
            },
        };

        let tile =self.get_tile(new_pos); 
        match  tile {
            Tile::Wall => {
                Self {
                    ..self.clone()
                }
            },
            Tile::Empty => {
                Self {
                    position: new_pos,
                    ..self.clone()
                }
            },
            _ => panic!("Unexpected tile"),
        }
    }

    fn navigate(&self, moves: &[Move]) -> Self {
        moves.iter()
            .fold(self.clone(), |n, m| {
                let new = Self {
                    direction: n.direction.turn(m),
                    ..n
                };
                (0..m.distance()).fold(new, |n, _| {
                    n.take_step()
                })
            })
    }

    fn password(&self) -> Result<i32, AocError> {
        let (x, y) = (self.position.x + self.x_offset + 1, self.position.y + 1);
        self.direction.try_into()
            .map(|d: i32| y * 1000 + x * 4 + d)
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeSide {
    id: i32,
    // transfomation matrix compared to side at grid origin

    // offset applied after transformation
    offset: Position,
}

impl CubeSide {
    fn new(id: i32, offset: &Position) -> Self {
        Self {
            id,
            offset: *offset,
        }
    }

    fn to_grid(&self, pos: Position) -> Position {
        pos + self.offset
    }


}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeLink {
    from: i32,
    direction: Direction,
    to: i32,
    transform: Array2<i32>,
    offset: Array1<i32>,
}

impl CubeLink{
    fn new(from: i32, direction: Direction, to: i32, transform: &Array2<i32>, offset: &Array1<i32>) -> Self {
        Self {
            from,
            direction,
            to,
            transform: transform.clone(),
            offset: offset.clone(),
        }
    } 

    fn to_cube(&self, pos: Position, dir: Direction) -> (Position, Direction) {
        let pos = Array1::from(vec![pos.x, pos.y]);
        let pos = self.transform.dot(&pos);
        let pos = pos + self.offset.clone();
        let dir = self.to_cube_direction(dir);
        println!("{}-{} {:?} -> {:?} {:?}", self.from, self.to, self.direction, pos, dir);
        (Position::new(pos[0], pos[1]), dir)
    }

    fn to_cube_direction(&self, dir: Direction) -> Direction {
        let vec = match dir {
            Direction::Up => Array1::from(vec![0, -1]),
            Direction::Down => Array1::from(vec![0, 1]),
            Direction::Left => Array1::from(vec![-1, 0]),
            Direction::Right => Array1::from(vec![1, 0]),
        };
        let new_vec = self.transform.dot(&vec);
        println!("cubelink {self:?}");
        println!("to_cube_direction: {vec} -> {new_vec}");
        match new_vec.as_slice().unwrap() {
            [0, -1] => Direction::Up,
            [0, 1] => Direction::Down,
            [-1, 0] => Direction::Left,
            [1, 0] => Direction::Right,
            _ => panic!("Unexpected direction"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeNavigator {
    grid: Grid,
    cube_id: i32,
    position: Position,
    direction: Direction,
    sides: HashMap<i32, CubeSide>,
    links: HashMap<i32, HashMap<Direction, CubeLink>>,
}

impl CubeNavigator {
    fn new(grid: Grid) -> Self {
        let links = create_cube_links().into_iter()
            .map(|l| (l.from, l.direction, l))
            .group_by(|(from, _, _)| *from)
            .into_iter()
            .map(|(from, group)| {
                (from, group.map(|(_, dir, l)| (dir, l)).collect::<HashMap<Direction, CubeLink>>())
            })
            .collect::<HashMap<i32, HashMap<Direction, CubeLink>>>();
        let sides = create_cube_sides().into_iter()
            .map(|s| (s.id, s))
            .collect::<HashMap<i32, CubeSide>>();
        let cube_id = 1;
        let position = Position::new(0, 0);
        let direction = Direction::Right;
        Self {
            grid,
            cube_id,
            position,
            direction,
            sides,
            links,
        }
    }

    fn get_tile(&self, pos: Position) -> Tile {
        let side = self.sides.get(&self.cube_id).unwrap();
        let grid_pos = side.to_grid(pos);
        println!("side: {side:?}, pos {pos:?}, grid {grid_pos:?}");
        self.grid.get_tile(&grid_pos)
    }
    
    fn take_step(&self) -> Self {
        print!("move {:?} from {:?} ", self.direction, self.position);
        let (new_cube, new_pos, new_direction) = match self.direction {
            Direction::Left => {
                let new_x = self.position.x - 1;
                if new_x < 0 {
                    let pos = Position::new(new_x, self.position.y);
                    let link = self.links.get(&self.cube_id).unwrap().get(&Direction::Left).unwrap();
                    let (new_pos, new_dir) = link.to_cube(pos, self.direction);
                    (link.to, new_pos, new_dir)
                } else {
                    (self.cube_id, Position::new(new_x, self.position.y), self.direction)
                }
            },
            Direction::Right => {
                let new_x = self.position.x + 1;
                if new_x >= 50 {
                    let pos = Position::new(new_x, self.position.y);
                    let link = self.links.get(&self.cube_id).unwrap().get(&Direction::Right).unwrap();
                    let (new_pos, new_dir) = link.to_cube(pos, self.direction);
                    (link.to, new_pos, new_dir)
                } else {
                    (self.cube_id, Position::new(new_x, self.position.y), self.direction)
                }
            },
            Direction::Up => {
                let new_y = self.position.y - 1; 
                if new_y < 0 {
                    let pos = Position::new(self.position.x, new_y);
                    let link = self.links.get(&self.cube_id).unwrap().get(&Direction::Up).unwrap();
                    let (new_pos, new_dir) = link.to_cube(pos, self.direction);
                    (link.to, new_pos, new_dir)
                } else {
                    (self.cube_id, Position::new(self.position.x, new_y), self.direction)
                }
            },
            Direction::Down => {
                let new_y = self.position.y + 1;
                if new_y >= 50 {
                    let pos = Position::new(self.position.x, new_y);
                    let link = self.links.get(&self.cube_id).unwrap().get(&Direction::Down).unwrap();
                    let (new_pos, new_dir) = link.to_cube(pos, self.direction);
                    (link.to, new_pos, new_dir)
                } else {
                    (self.cube_id, Position::new(self.position.x, new_y), self.direction)
                }
            },
        };

        let tile =self.get_tile(new_pos); 
        print!("{tile:?} at {new_pos:?} ");
        match  tile {
            Tile::Wall => {
                println!("(wall) -> {:?}", self.position);
                Self {
                    ..self.clone()
                }
            },
            Tile::Empty => {
                println!("-> {new_pos:?}");
                Self {
                    cube_id: new_cube,
                    direction: new_direction,
                    position: new_pos,
                    ..self.clone()
                }
            },
            _ => panic!("Unexpected tile"),
        }
    }

    fn navigate(&self, moves: &[Move]) -> Self {
        println!("navigate {moves:?}");
        println!("grid {}", self.grid);
        moves.iter()
            .fold(self.clone(), |n, m| {
                let new = Self {
                    direction: n.direction.turn(m),
                    ..n
                };
                (0..m.distance()).fold(new, |n, _| {
                    let new = n.take_step();
                    println!("move to {:?} facing {:?}", new.position, new.direction);
                    new
                })
            })
    }

    fn password(&self) -> Result<i32, AocError> {
        let side = self.sides.get(&self.cube_id).unwrap();
        let grid_pos = side.to_grid(self.position);
        let (x, y) = (grid_pos.x + 1, grid_pos.y + 1);
        self.direction.try_into()
            .map(|d: i32| y * 1000 + x * 4 + d)
    }
}

pub fn final_password(input: &InputModel) -> Result<i32, AocError> {
    let navigator = Navigator::new(&input.grid);
    let final_navigator = navigator.navigate(&input.moves);
    final_navigator.password()
}

pub fn cube_password(input: &InputModel) -> Result<i32, AocError> {
    let navigator = CubeNavigator::new(input.grid.clone());
    let final_navigator = navigator.navigate(&input.moves);
    final_navigator.password()
}


pub fn create_cube_sides() -> Vec<CubeSide> {
    vec![
        CubeSide::new(1, &Position::new(50, 0)),
        CubeSide::new(2, &Position::new(100, 0)),
        CubeSide::new(3, &Position::new(50, 50)),
        CubeSide::new(4, &Position::new(0, 100)),
        CubeSide::new(5, &Position::new(50, 100)),
        CubeSide::new(6, &Position::new(0, 150)),
    ]
}

pub fn create_cube_links() -> Vec<CubeLink> {
    vec![
        CubeLink::new(1, Direction::Up,    6, &array![[0, -1], [1, 0]], &array![0, 0]),
        CubeLink::new(1, Direction::Right, 2, &array![[1, 0], [0, 1]], &array![-50, 0]),
        CubeLink::new(1, Direction::Down,  3, &array![[1, 0], [0, 1]], &array![0, -50]),
        CubeLink::new(1, Direction::Left,  4, &array![[-1, 0], [0, -1]], &array![0, 50]),
        CubeLink::new(2, Direction::Up,    6, &array![[1, 0], [0, 1]], &array![100, -200]),
        CubeLink::new(2, Direction::Right, 5, &array![[-1, 0], [0, -1]], &array![100, 150]),
        CubeLink::new(2, Direction::Down,  3, &array![[0, -1], [1, 0]], &array![100, 50]),
        CubeLink::new(2, Direction::Left,  1, &array![[1, 0], [0, 1]], &array![50, 0]),
        CubeLink::new(3, Direction::Up, 1, &array![[1, 0], [0, 1]], &array![0, 50]),
        CubeLink::new(3, Direction::Right, 2, &array![[0, 1], [-1, 0]], &array![-50, 50]),
        CubeLink::new(3, Direction::Down, 3, &array![[1, 0], [0, 1]], &array![0, -50]),
        CubeLink::new(3, Direction::Left, 4, &array![[0, -1], [1, 0]], &array![0, 0]),
        CubeLink::new(4, Direction::Up, 3, &array![[0, 1], [-1, 0]], &array![0, 0]),
        CubeLink::new(4, Direction::Right, 5, &array![[1, 0], [0, 1]], &array![-50, 0]),
        CubeLink::new(4, Direction::Down, 6, &array![[1, 0], [0, 1]], &array![0, -50]),
        CubeLink::new(4, Direction::Left, 1, &array![[-1, 0], [0, -1]], &array![0, 50]),
        CubeLink::new(5, Direction::Up, 3, &array![[1, 0], [0, 1]], &array![0, 50]),
        CubeLink::new(5, Direction::Right, 2, &array![[-1, 0], [0, -1]], &array![100, 50]),
        CubeLink::new(5, Direction::Down, 6, &array![[0, -1], [1, 0]], &array![99, 0]),
        CubeLink::new(5, Direction::Left, 1, &array![[-1, 0], [0, -1]], &array![0, 50]),
        CubeLink::new(6, Direction::Up, 4, &array![[1, 0], [0, 1]], &array![0, 50]),
        CubeLink::new(6, Direction::Right, 5, &array![[0, 1], [-1, 0]], &array![0, 99]),
        CubeLink::new(6, Direction::Down, 2, &array![[0, 1], [-1, 0]], &array![0, -50]),
        CubeLink::new(6, Direction::Left, 1, &array![[0, 1], [-1, 0]], &array![0, 0]),
    ]
}


pub const TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        assert_eq!(input.grid.0.len(), 12);
        assert_eq!(input.moves, vec![
            Move::Forward(10),
            Move::TurnRight,
            Move::Forward(5),
            Move::TurnLeft,
            Move::Forward(5),
            Move::TurnRight,
            Move::Forward(10),
            Move::TurnLeft,
            Move::Forward(4),
            Move::TurnRight,
            Move::Forward(5),
            Move::TurnLeft,
            Move::Forward(5),
        ]);
    }
    
    #[test]
    fn test_password() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let actual = final_password(&input).unwrap();
        let expected = 6032;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_do_move() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let navigator = Navigator::new(&input.grid);
        let actual = navigator.take_step();
        let expected = Position::new(1, 0);
        assert_eq!(actual.position, expected);
    }

    #[test]
    fn test_horizontal_bounds() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let navigator = Navigator::new(&input.grid);
        let actual = navigator.horizontal_bounds(Position::new(0,0));
        let expected = (0, 4);
        assert_eq!(actual, expected);
        let actual = navigator.horizontal_bounds(Position::new(0,5));
        let expected = (-8, 4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_vertical_bounds() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let navigator = Navigator::new(&input.grid);
        let actual = navigator.vertical_bounds(Position::new(0,0));
        let expected = (0, 12);
        assert_eq!(actual, expected);
    }


}
