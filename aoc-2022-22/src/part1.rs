use std::collections::HashMap;

use aoc_common::position::Position;

use crate::{grid::*, AocError, InputModel};


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Navigator {
    grid: Grid,
    position: Position,
    x_offset: i32,
    direction: Direction,
    horiz_bounds_cache: HashMap<i32, (i32, i32)>,
    vert_bounds_cache: HashMap<i32, (i32, i32)>,
}

impl Navigator {

    pub fn new(grid: &Grid) -> Navigator {
        let x_offset = grid.clone().into_iter()
            .next().unwrap().iter()
            .enumerate()
            .find(|(_, &tile)| tile == Tile::Empty)
            .map(|(x, _)| x)
            .unwrap() as i32;
        Navigator {
            grid: grid.clone(),
            position: Position::new(0, 0),
            x_offset,
            direction: Direction::Right,
            horiz_bounds_cache: HashMap::with_capacity(grid.len()),
            vert_bounds_cache: HashMap::with_capacity(grid.len()),
        }
    }

    fn get_tile(&self, pos: Position) -> Tile {
        let (x, y) = (pos.x, pos.y);
        if y < 0 || y >= self.grid.len() as i32 {
            return Tile::Void
        }
        let row = &self.grid[y as usize];
        if x < -self.x_offset ||  x + self.x_offset >= row.len() as i32 {
            return Tile::Void
        }
        self.grid[pos.y as usize][(pos.x + self.x_offset) as usize]
    }

    fn horizontal_bounds(&self, pos: Position) -> (i32, i32) {
        let y = pos.y;
        let left: i32 = (-self.x_offset..=pos.x)
            .rev()
            .take_while(|x| self.get_tile(Position::new(*x,y)) != Tile::Void)
            .last()
            .unwrap();
        let right: i32 = (pos.x..self.grid[y as usize].len() as i32)
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
        let bottom = (pos.y as usize..self.grid.len())
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
                    direction: n.direction.turn(*m),
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



pub fn final_password(input: &InputModel) -> Result<i32, AocError> {
    let navigator = Navigator::new(&input.grid);
    let final_navigator = navigator.navigate(&input.moves);
    final_navigator.password()
}


#[cfg(test)]
mod tests {

    use crate::TEST_INPUT;

    use super::*;

    
    #[test]
    fn test_password() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let actual = final_password(&input).unwrap();
        let expected = 6032;
        assert_eq!(actual, expected);
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

    #[test]
    fn test_do_move() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let navigator = Navigator::new(&input.grid);
        let actual = navigator.take_step();
        let expected = Position::new(1, 0);
        assert_eq!(actual.position, expected);
    }


}
