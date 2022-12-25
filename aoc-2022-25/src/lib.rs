use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub lines: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InputModel {
            lines: s.lines().map(|l| l.to_string()).collect(),
        })
    }
}


pub fn snafu_decimal(snafu: &str) -> i64  {
   snafu.chars()
       .map(|c| match c {
           '=' => -2,
           '-' => -1,
           '0' => 0,
           '1' => 1,
           '2' => 2,
           _ => panic!("Invalid character"),
       })
       .fold(0, |acc, x| acc * 5 + x)
}

pub fn decimal_snafu(decimal: i64) -> String {
    match decimal {
        -2 => "=".to_string(),
        -1 => "-".to_string(),
        0 => "0".to_string(),
        1 => "1".to_string(),
        2 => "2".to_string(),
        _ => {
            let div = decimal.div_euclid(5);
            let rem = decimal.rem_euclid(5);
            if rem > 2  {
                decimal_snafu(div + 1) + &decimal_snafu(rem - 5)
            } else {
                decimal_snafu(div) + &decimal_snafu(rem)
            }
        }
    }
}

pub fn sum_snafu(snafus: &[String]) -> i64 {
    snafus.iter()
        .map(|s| snafu_decimal(s))
        .sum::<i64>()
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_CASES: &[(&str, &str)] = &[
        ("1", "1"),
        ("2", "2"),
        ("3", "1="),
        ("4", "1-"),
        ("5", "10"),
        ("6", "11"),
        ("7", "12"),
        ("8", "2="),
        ("9", "2-"),
        ("10", "20"),
        ("15", "1=0"),
        ("20", "1-0"),
        ("2022", "1=11-2"),
        ("12345", "1-0---0"),
        ("314159265", "1121-1110-1=0"),
    ];

    #[test]
    fn test_snafu_decimal() {
        for (decimal, snafu) in TEST_CASES {
            assert_eq!(snafu_decimal(snafu), decimal.parse::<i64>().unwrap());
        }
    }

    #[test]
    fn test_decimal_snafu() {
        for (decimal, snafu) in TEST_CASES {
            let decimal = decimal.parse::<i64>().unwrap();
            println!("{} {}", decimal, snafu);
            assert_eq!(decimal_snafu(decimal), *snafu);
        }
    }
}
