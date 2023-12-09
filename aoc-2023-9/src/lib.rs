use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub measurements: Vec<Vec<i64>>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

fn differences(ns: &[i64]) -> Vec<i64> {
    ns.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn next_number(ns: &[i64]) -> Option<i64> {
    ns.last().and_then(|&last| {
        let ds = differences(ns);
        if ds.iter().all(|&d| d == 0) {
            Some(last)
        } else {
            next_number(&ds).map(|d| last + d)
        }
    })
}

pub fn prev_number(ns: &[i64]) -> Option<i64> {
    ns.first().and_then(|&first| {
        let ds = differences(ns);
        if ds.iter().all(|&d| d == 0) {
            Some(first)
        } else {
            prev_number(&ds).map(|d| first - d)
        }
    })
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let measurements = s
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|s| s.parse::<i64>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("Error parsing input");

        Ok(InputModel {
            measurements,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    fn test_input() -> InputModel {
        let input = TEST_INPUT;
        input.parse::<InputModel>().unwrap()
    }

    #[test]
    fn test_differences() {
        let actual = differences(&[0, 3, 6, 9, 12, 15]);
        let expected = vec![3, 3, 3, 3, 3];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_number() {
        let meas = test_input().measurements;
        let actual = meas
            .iter()
            .filter_map(|ns| next_number(ns))
            .collect::<Vec<_>>();
        let expected: Vec<i64> = vec![18, 28, 68];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_prev_number() {
        let meas = test_input().measurements;
        let actual = meas
            .iter()
            .filter_map(|ns| prev_number(ns))
            .collect::<Vec<_>>();
        let expected: Vec<i64> = vec![-3, 0, 5];

        assert_eq!(actual, expected);
    }
}
