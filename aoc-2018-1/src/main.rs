use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("../data/input.txt");

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    values: Vec<i32>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution found")]
    NoSolutionFound,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .into_iter()
            .map(|s| s.parse::<i32>().map_err(|_| AocError::ParseError))
            .collect::<Result<Vec<i32>, AocError>>()
            .map(|values| InputModel{ values })
    }
}

pub fn part1(input: &InputModel) -> Result<String, AocError> {
    return Ok(input.values.iter().sum::<i32>().to_string());
}

pub fn part2(input: &InputModel) -> Result<String, AocError> {
    let mut sums: HashSet<i32> = HashSet::new();
    let mut sum = 0;
    for x in input.values.iter().cycle() {
        sum += x;
        if sums.contains(&sum) {
            return Ok(sum.to_string());
        }
        sums.insert(sum);
    }
    Err(AocError::NoSolutionFound)
}

fn main() -> Result<(), AocError> {
    let input: InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "+1\n-2\n+3\n+1";

    fn input_data() -> InputModel {
        InputModel {
            values: vec![1, -2, 3, 1],
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
        let expected = "3";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "2";

        assert_eq!(actual, expected);
    }
}
