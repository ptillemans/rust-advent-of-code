use core::fmt;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use aoc_common::position::Position;
use ndarray::{array, Array2};

use crate::{
    grid::{Direction, Grid, Move, Tile},
    AocError, InputModel,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubePosition {
    face_id: usize,
    position: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeSide {
    id: usize,
    grid: Grid,
    face_position: Position,
}

impl CubeSide {
    pub fn new(id: usize, grid: Grid, face_position: Position) -> Self {
        Self {
            id,
            grid,
            face_position,
        }
    }

    fn get_tile(&self, position: Position) -> Tile {
        let (x, y) = (position.x, position.y);
        if y < 0 || y >= self.grid.len() as i32 {
            return Tile::Void;
        }
        let row = &self.grid[y as usize];
        if x < 0 || x >= row.len() as i32 {
            return Tile::Void;
        }
        self.grid[position.y as usize][position.x as usize]
    }

}

impl Display for CubeSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Rotation {
    Straight,
    Flip,
    Clockwise,
    CounterClockwise,
}

impl From<Rotation> for Array2<i32> {
    fn from(rotation: Rotation) -> Self {
        match rotation {
            Rotation::Straight => array![[1, 0], [0, 1]],
            Rotation::Flip => array![[-1, 0], [0, -1]],
            Rotation::Clockwise => array![[0, 1], [-1, 0]],
            Rotation::CounterClockwise => array![[0, -1], [1, 0]],
        }
    }
}

impl Rotation {
    fn apply_dir(&self, dir: Direction) -> Direction {
        match self {
            Rotation::Straight => dir,
            Rotation::Flip => dir.turn(Move::TurnLeft).turn(Move::TurnLeft),
            Rotation::Clockwise => dir.turn(Move::TurnRight),
            Rotation::CounterClockwise => dir.turn(Move::TurnLeft),
        }
    }

    fn apply_rot(&self, rot: Rotation) -> Rotation {
        match (self, rot) {
            (Rotation::Straight, Rotation::Straight) => Rotation::Straight,
            (Rotation::Straight, Rotation::Flip) => Rotation::Flip,
            (Rotation::Straight, Rotation::Clockwise) => Rotation::Clockwise,
            (Rotation::Straight, Rotation::CounterClockwise) => Rotation::CounterClockwise,
            (Rotation::Flip, Rotation::Straight) => Rotation::Flip,
            (Rotation::Flip, Rotation::Flip) => Rotation::Straight,
            (Rotation::Flip, Rotation::Clockwise) => Rotation::CounterClockwise,
            (Rotation::Flip, Rotation::CounterClockwise) => Rotation::Clockwise,
            (Rotation::Clockwise, Rotation::Straight) => Rotation::Clockwise,
            (Rotation::Clockwise, Rotation::Flip) => Rotation::CounterClockwise,
            (Rotation::Clockwise, Rotation::Clockwise) => Rotation::Flip,
            (Rotation::Clockwise, Rotation::CounterClockwise) => Rotation::Straight,
            (Rotation::CounterClockwise, Rotation::Straight) => Rotation::CounterClockwise,
            (Rotation::CounterClockwise, Rotation::Flip) => Rotation::Clockwise,
            (Rotation::CounterClockwise, Rotation::Clockwise) => Rotation::Straight,
            (Rotation::CounterClockwise, Rotation::CounterClockwise) => Rotation::Flip,
        }
    }

    fn inverse(&self) -> Rotation {
        match self {
            Rotation::Straight => Rotation::Straight,
            Rotation::Flip => Rotation::Flip,
            Rotation::Clockwise => Rotation::CounterClockwise,
            Rotation::CounterClockwise => Rotation::Clockwise,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeLink {
    to: usize,
    rotation: Rotation,
}

impl CubeLink {
    fn new(to: usize, rotation: Rotation) -> Self {
        Self { to, rotation }
    }
}

type CubeLinks = HashMap<usize, HashMap<Direction, CubeLink>>;

pub fn find_cube_sides(grid: &Grid, size: usize) -> Vec<CubeSide> {
    let faces_h = grid.len() / size;
    (0..grid.len())
        .step_by(size)
        .flat_map(|y| (0..grid[y].len()).step_by(size).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            let face_position = Position::new(x as i32 / size as i32, y as i32 / size as i32);
            let face_grid: Grid = (y..y + size)
                .map(|y| grid[y][x..x + size].to_vec())
                .collect::<Vec<Vec<Tile>>>()
                .into();
            if face_grid
                .iter()
                .all(|row| row.iter().any(|t| *t == Tile::Void))
            {
                None
            } else {
                let face_id = face_position.x * faces_h as i32 + face_position.y + 1;
                Some(CubeSide::new(face_id as usize, face_grid, face_position))
            }
        })
        .collect::<Vec<CubeSide>>()
}

pub fn find_straight_links(sides: &[CubeSide]) -> CubeLinks {
    let side_map: HashMap<Position, &CubeSide> =
        sides.iter().map(|s| (s.face_position, s)).collect();
    let straight_links = sides
        .iter()
        .map(|side| {
            let links = Direction::iter()
                .filter_map(|dir| {
                    let face_pos = dir.step(side.face_position);
                    if let Some(other) = side_map.get(&face_pos) {
                        let link = CubeLink::new(other.id, Rotation::Straight);

                        Some((dir, link))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<Direction, CubeLink>>();
            (side.id, links)
        })
        .collect::<CubeLinks>();

    straight_links
}

fn find_rotation(dir1: Direction, dir2: Direction) -> Rotation {
    match (dir1, dir2) {
        (Direction::Up, Direction::Right) => Rotation::Clockwise,
        (Direction::Right, Direction::Down) => Rotation::Clockwise,
        (Direction::Down, Direction::Left) => Rotation::Clockwise,
        (Direction::Left, Direction::Up) => Rotation::Clockwise,
        (Direction::Up, Direction::Left) => Rotation::CounterClockwise,
        (Direction::Left, Direction::Down) => Rotation::CounterClockwise,
        (Direction::Down, Direction::Right) => Rotation::CounterClockwise,
        (Direction::Right, Direction::Up) => Rotation::CounterClockwise,
        (Direction::Up, Direction::Down) => Rotation::Straight,
        (Direction::Down, Direction::Up) => Rotation::Straight,
        (Direction::Left, Direction::Right) => Rotation::Straight,
        (Direction::Right, Direction::Left) => Rotation::Straight,
        (Direction::Down, Direction::Down) => Rotation::Flip,
        (Direction::Up, Direction::Up) => Rotation::Flip,
        (Direction::Left, Direction::Left) => Rotation::Flip,
        (Direction::Right, Direction::Right) => Rotation::Flip,
    }
}

fn neighbor_directions(dir1: Direction, dir2: Direction) -> bool {
    matches!(
        (dir1, dir2),
        (Direction::Up, Direction::Right)
            | (Direction::Right, Direction::Down)
            | (Direction::Down, Direction::Left)
            | (Direction::Left, Direction::Up)
            | (Direction::Up, Direction::Left)
            | (Direction::Left, Direction::Down)
            | (Direction::Down, Direction::Right)
            | (Direction::Right, Direction::Up)
    )
}

fn add_links(sides: &[CubeSide], links: &CubeLinks, cube_size: usize) -> CubeLinks {
    let mut links = links.clone();
    let additional: Vec<(usize, Direction, Rotation, usize)> = sides
        .iter()
        .flat_map(|side| {
            let start_links = links.get(&side.id).unwrap();
            start_links
                .iter()
                .permutations(2)
                .filter(|edges| neighbor_directions(*edges[0].0, *edges[1].0))
                .map(|links| {
                    let (dir1, link1) = links[0];
                    let (dir2, link2) = links[1];
                    let rotation = find_rotation(*dir2, *dir1);
                    // reverse rotation on link 1 and add rotation of link2
                    let link_rot = link2.rotation.apply_rot(link1.rotation.inverse());
                    // apply on the corner rotation
                    let back_rotation = link_rot.apply_rot(rotation);
                    let dir = link1.rotation.inverse().apply_dir(*dir2);
                    
                    (link1.to, dir, back_rotation, link2.to)
                })
                .collect::<Vec<(usize, Direction, Rotation, usize)>>()
        })
        .collect();

    additional
        .iter()
        .for_each(|(from, direction, rotation, to)| {
            let link = CubeLink::new(*to, *rotation);
            let side = links.entry(*from).or_insert_with(HashMap::new);

            side.entry(*direction)
                .and_modify(|e| {
                    if *e != link {
                        println!("inconsisten links {e:?} <> {link:?}");
                        panic!("invalid link");
                    }
                })
                .or_insert(link);
        });

    links
}

pub fn find_cube_links(sides: &[CubeSide], cube_size: usize) -> CubeLinks {
    let links = find_straight_links(sides);

    let links = add_links(sides, &links, cube_size);
    let links = add_links(sides, &links, cube_size);
    let links = add_links(sides, &links, cube_size);
    add_links(sides, &links, cube_size)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
    size: usize,
    sides: Vec<CubeSide>,
    links: CubeLinks,
}

impl Cube {
    pub fn new(grid: &Grid, size: usize) -> Self {
        let sides = find_cube_sides(grid, size);
        let links = find_cube_links(&sides, size);
        Cube { size, sides, links }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let faces: HashMap<Position, Vec<String>> = self
            .sides
            .iter()
            .map(|side| -> (Position, Vec<String>) {
                (
                    side.face_position,
                    format!("{side}").lines().map(|s| s.to_string()).collect(),
                )
            })
            .collect();

        for y in 0..4 {
            for row in 0..self.size {
                let mut line = String::new();
                for x in 0..=3 {
                    let pos = Position::new(x, y);
                    if let Some(side) = faces.get(&pos) {
                        line.push_str(&side[row]);
                    } else {
                        line.push_str(&" ".repeat(self.size));
                    }
                }
                if line.trim().is_empty() {
                    continue;
                }
                writeln!(f, "{}", line.trim_end())?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CubeWalker {
    cube: Cube,
    face: usize,
    position: Position,
    direction: Direction,
}

impl CubeWalker {
    pub fn new(cube: Cube, face: usize, position: Position, direction: Direction) -> Self {
        CubeWalker {
            cube,
            face,
            position,
            direction,
        }
    }

    pub fn walk(&mut self, no_walls: bool) {
        let mut pos = self.direction.step(self.position);
        let mut face = self.face;
        let mut dir =  self.direction;
        if self.out_of_bounds(pos) {
            pos = self
                .direction
                .inverse()
                .steps(pos, self.cube.size);
            let link = self
                .cube
                .links
                .get(&self.face)
                .unwrap()
                .get(&self.direction)
                .unwrap();
            face = link.to;
            dir = link.rotation.inverse().apply_dir(self.direction);
            pos = self.rotate_position(pos, link.rotation);
        }
        let tile = self.cube.sides.iter().find(|f| f.id == face).unwrap().get_tile(pos);
        if no_walls || tile == Tile::Empty {
            self.face = face;
            self.position = pos;
            self.direction = dir;
        }
    }

    fn get_tile(&self) -> Tile {
        self.cube
            .sides
            .get(self.face)
            .unwrap()
            .get_tile(self.position)
    }

    fn out_of_bounds(&self, pos: Position) -> bool {
        pos.x < 0 || pos.x >= self.cube.size as i32 || pos.y < 0 || pos.y >= self.cube.size as i32
    }

    fn rotate_position(&self, pos: Position, rotation: Rotation) -> Position {
        let n = self.cube.size as i32 - 1;
        match rotation {
            Rotation::Straight => pos,
            Rotation::Clockwise => Position::new(pos.y, n - pos.x),
            Rotation::CounterClockwise => Position::new(n - pos.y, pos.x),
            Rotation::Flip => Position::new(n - pos.x, n - pos.y),
        }
    }

    fn do_walk(&mut self, moves: &[Move], no_walls: bool, display: bool) {
        for m in moves {
            self.direction = self.direction.turn(*m);
            if let Move::Forward(n) = m {
                self.walk(no_walls);
                for _ in 1..*n {
                    if display {
                    println!("{self}");
                    }
                    self.walk(no_walls);
                }
            }
            if display {
                println!("{self}");
            }
        }
    }

    fn current_face(&self) -> CubeSide {
        self.cube.sides.iter().find(|f| f.id == self.face).unwrap().clone()
    }
}

impl Display for CubeWalker {
    // place a marker for the walker position in the current face
    // of the formatted cube
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display = format!("{}", self.cube);
        if let Some(current_face) = self.cube.sides.iter().find(|f| f.id == self.face) {
            let (c, r) = current_face.face_position.into();

            let walker_line = (r * self.cube.size as i32 + self.position.y) as usize;
            for (i, line) in display.lines().enumerate() {
                let mut line = line.to_string();
                if i == walker_line {
                    let p: usize = (c * self.cube.size as i32 + self.position.x) as usize;
                    line.replace_range(
                        p..p + 1,
                        &format!(
                            "{}",
                            match self.direction {
                                Direction::Up => '^',
                                Direction::Down => 'v',
                                Direction::Left => '<',
                                Direction::Right => '>',
                            }
                        ),
                    );
                };
                writeln!(f, "{line}")?;
            }
        } else {
            writeln!(f, "No such face : {}", self.face)?;
        }
        Ok(())
    }
}

pub fn cube_password(input: &InputModel, size: usize) -> Result<i32, AocError> {
    let cube = Cube::new(&input.grid, size);
    let face_id = cube.sides[0].id;
    let mut walker = CubeWalker::new(cube, face_id, Position::new(0, 0), Direction::Right);
    walker.do_walk(&input.moves, false, false);
    let (x, y) = walker.position.into();
    let (c, r) = walker.current_face().face_position.into();

    let x = x + c * walker.cube.size as i32 + 1;
    let y = y + r * walker.cube.size as i32 + 1;
    let d: i32 = walker.direction.try_into().unwrap();
    walker.direction.try_into()
        .map(|d: i32| y * 1000 + x * 4 + d)

}

#[cfg(test)]
mod tests {

    use crate::TEST_INPUT;

    use super::*;

    #[test]
    fn read_cube_sides() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let cubesides = find_cube_sides(&input.grid, 4);

        assert_eq!(cubesides.len(), 6);
    }

    #[test]
    fn test_find_straight_links() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let cubesides = find_cube_sides(&input.grid, 4);

        let links = find_straight_links(&cubesides);
        assert_eq!(links.len(), 6);
        assert_eq!(links.iter().flat_map(|(_, dir)| dir.keys()).count(), 10);
    }

    #[test]
    fn read_cube_links() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let cubesides = find_cube_sides(&input.grid, 4);

        let links = find_cube_links(&cubesides, 4);
        assert_eq!(links.len(), 6);
        assert_eq!(links.iter().flat_map(|(_, dir)| dir.keys()).count(), 24);
    }

    #[test]
    fn test_cube_display() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let cube = Cube::new(&input.grid, 4);
        let cube_display = format!("{}", cube);

        // trim last 2 lines of TEST_INPUT
        let expected = TEST_INPUT
            .lines()
            .take(TEST_INPUT.lines().count() - 2)
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(cube_display.trim_end(), expected);
    }

    #[test]
    fn test_cube_walk_straight() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        for direction in Direction::iter() {
            let cube = Cube::new(&input.grid, 4);
            let face_id = cube.sides[0].id;
            let mut walker = CubeWalker::new(cube, face_id, Position::new(1, 1), direction);
            let start_walker = walker.clone();

            let moves = vec![Move::Forward(16)];
            //walker.display_walk(&moves);
            for _ in 0..16 {
                walker.walk(true);
            }

            assert_eq!(walker, start_walker);
        }
    }

    #[test]
    fn test_cube_password() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let password = cube_password(&input, 4).unwrap();
        assert_eq!(password, 5031);
    }
}
