use itertools::*;
use std::{str::FromStr, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub lines: Vec<(String, Vec<u64>)>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|l| {
                let (line, blocks) = l.split_once(" ").ok_or(AocError::ParseError)?;
                let blocks = blocks
                    .split(",")
                    .map(|b| b.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                Ok((line.to_string(), blocks))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(InputModel { lines })
    }
}


// assume we are at the start of a possible block
fn step(s: &str, blocks: &[u64], memo: &mut HashMap<(String, Vec<u64>), u64>) -> u64 {
    if blocks.is_empty() {
        return if s.contains('#') { 0 } else { 1 };
    }
    if s.len() < blocks.iter().sum::<u64>() as usize {
        return 0;
    }

    if let Some(&arrangements) = memo.get(&(s.to_string(), blocks.to_vec())) {
        return arrangements;
    }
    
    let mut arrangements = 0;

    if s.starts_with("?") || s.starts_with(".") {
        let space_alt: String = skip_empty(&s[1..]).to_string();
        arrangements += step(&space_alt, blocks, memo);
    }

    if s.starts_with("#") || s.starts_with("?") {
        let (block, rest) = s.split_at(blocks[0] as usize);
        if block.chars().all(|c| c == '#' || c == '?') && !rest.starts_with('#') {
            let next_blocks = &blocks[1..];
            if rest.is_empty() && next_blocks.is_empty() {
                arrangements += 1;
            } else {
                let rest = if rest.is_empty() {
                    ""
                } else {
                    skip_empty(&rest[1..])
                };
                arrangements += step(rest, next_blocks, memo);
            }
        }
    }

    memo.insert((s.to_string(), blocks.to_vec()), arrangements);
    
    arrangements
}

fn skip_empty(s: &str) -> &str {
    s.trim_start_matches('.')
}

pub fn count_arrangements(s: &str, blocks: &[u64]) -> u64 {
    let mut memo = HashMap::new();
    step(skip_empty(s), blocks, &mut memo)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_no_blocks() {
        let blocks = vec![];
        let mut memo = HashMap::new();

        let s = "";
        assert_eq!(step(s, &blocks, &mut memo), 1);

        let s = "###";
        assert_eq!(step(s, &blocks, &mut memo), 0);

        let s = "?.?";
        assert_eq!(step(s, &blocks, &mut memo), 1);
    }

    #[test]
    fn test_one_block() {
        let blocks = vec![1];
        let blocks_3 = vec![3];
        let mut memo = HashMap::new();

        let s = "#";
        assert_eq!(step(s, &blocks, &mut memo), 1);

        let s = "###";
        assert_eq!(step(s, &blocks, &mut memo), 0);

        let s = "###";
        assert_eq!(step(s, &blocks_3, &mut memo), 1);

        let s = "#?#";
        assert_eq!(step(s, &blocks_3, &mut memo), 1);

        let s = "?.?";
        assert_eq!(step(s, &blocks, &mut memo), 2);

        let s = "?.#";
        assert_eq!(step(s, &blocks, &mut memo), 1);
    }

    #[test]
    fn test_more_blocks() {
        let s = "???";
        let blocks = vec![1, 1];
        let mut memo = HashMap::new();
        assert_eq!(step(s, &blocks, &mut memo), 1);
    }

    #[test]
    fn test_test_count_arrangements() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let lines = input.lines;
        let expected = vec![1, 4, 1, 1, 4, 10];

        for i in 0..lines.len() {
            assert_eq!(
                count_arrangements(&lines[i].0, &lines[i].1),
                expected[i],
                "i: {}",
                i
            );
        }
    }
}
