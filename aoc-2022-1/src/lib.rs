use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub calories : Vec<Vec<u32>>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No max found")]
    NoMaxFound,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories = s.lines()
            .group_by(|line| line.is_empty())
            .into_iter()
            .filter_map(|(is_empty, group)| {
                if is_empty {
                    None
                } else {
                    Some(group.map(|s| s.parse::<u32>().unwrap())
                         .collect::<Vec<_>>())
                }
            })
            .collect::<Vec<Vec<u32>>>();
        Ok(InputModel { calories })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    pub fn input_data() -> InputModel {
        InputModel {
            calories: vec![
                vec![1000, 2000, 3000], 
                vec![4000], 
                vec![5000, 6000], 
                vec![7000, 8000, 9000], 
                vec![10000]
            ],
        }
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

}
