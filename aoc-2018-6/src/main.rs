use std::collections::{HashSet, HashMap};

use aoc_2018_6::{AocError, InputModel};
use aoc_2018_6::{fill_grid, count_safe_locations, Cell};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String,AocError> {
    let grid = fill_grid(&input.points);
    let edge_points = grid[0].iter()
        .chain(grid[grid.len() - 1].iter())
        .chain(grid.iter().map(|row| &row[0]))
        .chain(grid.iter().map(|row| &row[row.len() - 1]))
        .filter_map(|cell| match cell {
            Cell::RiskLevel(_, point) => Some(*point),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let mut areas: HashMap<(i32, i32), i32> = HashMap::new();
    for row in grid {
        for cell in row {
            if let Cell::RiskLevel(_, point) = cell {
                if !edge_points.contains(&point) {
                    *areas.entry(point).or_insert(0) += 1;
                }
            }
        }
    }
    let max_area = areas.values().max().unwrap() + 1;
    Ok(max_area.to_string())
}

fn part2(input: &InputModel, max_len: i32) -> Result<String, AocError> {
    let count = count_safe_locations(&input.points, max_len);
    Ok(count.to_string())
}

fn main() -> Result<(), AocError> {
    let input:InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input, 10000)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

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
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "17";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data(), 32).unwrap();
        let expected = "16";

        assert_eq!(actual, expected);
    }
}
