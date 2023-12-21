use std::{
    fmt::{Display, Formatter},
    ops::Index,
    str::FromStr,
};

use crate::position::Position;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.0.iter()
    }

    pub fn in_bounds(&self, index: Position) -> bool {
        index.x >= 0
            && index.y >= 0
            && index.x < self.width() as i32
            && index.y < self.height() as i32
    }

    pub fn get(&self, index: Position) -> Option<&T> {
        if self.in_bounds(index) {
            Some(&self.0[index.y as usize][index.x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: Position, cell: T) -> Option<()> {
        if self.in_bounds(index) {
            self.0[index.y as usize][index.x as usize] = cell;
            Some(())
        } else {
            None
        }
    }
}

impl<T: FromStr> FromStr for Grid<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_string().parse::<T>()?);
            }
            grid.push(row);
        }
        Ok(Grid(grid))
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Clone> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Grid(value.iter().map(|row| row.to_vec()).collect::<Vec<_>>())
    }
}

impl<T: Clone> From<&[&[T]]> for Grid<T> {
    fn from(value: &[&[T]]) -> Self {
        Grid(value.iter().map(|row| row.to_vec()).collect::<Vec<_>>())
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        if self.in_bounds(index) {
            &self.0[index.y as usize][index.x as usize]
        } else {
            panic!("Index out of bounds: {}", index);
        }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
