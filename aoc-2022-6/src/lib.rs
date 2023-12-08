use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub datastream: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution found")]
    NoSolution,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InputModel {
            datastream: s.to_owned(),
        })
    }
}

pub fn all_unique(window: &str) -> bool {
    let chars = window.chars();
    let mut seen = std::collections::HashSet::new();
    for c in chars {
        if seen.contains(&c) {
            return false;
        }
        seen.insert(c);
    }
    true
}

pub fn find_packet_start(datastream: &str, window_size: usize) -> Result<usize, AocError> {
    for i in 0..datastream.len()-window_size {
        let window = &datastream[i..i+window_size];
        if all_unique(window) {
            return Ok(i+window_size);
        }
    }
    Err(AocError::NoSolution)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
                
    #[test]
    fn test_parse() {
        let actual = InputModel::from_str(TEST_INPUT).unwrap();
        let expected = super::InputModel {
            datastream: TEST_INPUT.to_owned(),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(all_unique("abcde"), true);
        assert_eq!(all_unique("abcda"), false);
    }
    
    #[test]
    fn test_find_packet_start() {
        let actual = find_packet_start(TEST_INPUT, 4).unwrap();
        let expected = 7;
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_find_packet_start_2() {
        let actual = find_packet_start(TEST_INPUT, 14).unwrap();
        let expected = 19;
        assert_eq!(actual, expected);
    }
}
