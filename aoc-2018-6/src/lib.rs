use std::str::FromStr;

// solve day 6 of AdventOfCode
//

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub points: Vec<(i32, i32)>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

// parse a point from a line
fn parse_line(s: &str) -> Result<(i32, i32), AocError> {
    let mut parts = s.split(", ");
    let x = parts.next().ok_or(AocError::ParseError)?;
    let y = parts.next().ok_or(AocError::ParseError)?;
    let x = x.parse::<i32>().map_err(|_| AocError::ParseError)?;
    let y = y.parse::<i32>().map_err(|_| AocError::ParseError)?;
    Ok((x, y))
}



impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(parse_line)
            .collect::<Result<Vec<(i32, i32)>, AocError>>()
            .map(|points| InputModel { points })
    }
}

// represent points on the grid
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Hazard(i32, i32),
    RiskLevel(i32, (i32, i32)),
    Safe,
}

// find the bounding box around the points
fn bounding_box(points: &Vec<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    let mut min_x = points[0].0;
    let mut max_x = min_x;
    let mut min_y = points[0].1;
    let mut max_y = min_y;
    for (x, y) in points {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    ((min_x, min_y), (max_x,  max_y))
}

fn fill_grid(points: &Vec<(i32, i32)>) -> Vec<Vec<Cell>> {
    let ((min_x, min_y), (max_x, max_y)) = bounding_box(points);
    let mut grid = vec![vec![Cell::Empty; (max_x - min_x + 3) as usize]; (max_y - min_y + 3) as usize];
    for (x, y) in points {
        grid[*y as usize][*x as usize] = Cell::Hazard(*x, *y);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut min_distance = std::i32::MAX;
            let mut min_point = (0, 0);
            let mut min_count = 0;
            for (px, py) in points {
                let distance = (x - px).abs() + (y - py).abs();
                if distance < min_distance {
                    min_distance = distance;
                    min_point = (*px, *py);
                    min_count = 1;
                } else if distance == min_distance {
                    min_count += 1;
                }
            }
            if min_count == 1 {
                grid[y as usize][x as usize] = Cell::RiskLevel(min_distance, min_point);
            }
        }
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{:?}", grid[y as usize][x as usize]);
        }
        println!();
    };
    grid
}
#[cfg(test)]
mod tests {
    use super::*;

    pub fn input_data() -> InputModel {
        InputModel {
          points: vec![
            (1, 1),
            (1, 6),
            (8, 3),
            (3, 4),
            (5, 5),
            (8, 9),
          ]
        }
    }

    #[test]
    fn test_bounding_box() {
        let ((min_x, min_y), (max_x, max_y)) = bounding_box(&input_data().points);
        assert_eq!(min_x, 1);
        assert_eq!(max_x, 8);
        assert_eq!(min_y, 1);
        assert_eq!(max_y, 9);
    }

    #[test]
    fn test_fill_grid_trivial() {
        let grid = fill_grid(&vec![(1, 1)]);
        for x in 0..3 {
            for y in 0..3 {
                if x == 1 && y == 1 {
                    assert_eq!(grid[x][y], Cell::Hazard(0, 0));
                } else {
                    assert_eq!(grid[x][y], Cell::RiskLevel((x as i32 - 1).abs()+(y as i32 - 1).abs(), (1, 1)));
                }
            }
        }
    }

}
