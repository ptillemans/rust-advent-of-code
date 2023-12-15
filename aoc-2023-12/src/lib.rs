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

    fn operational_spring(s: &str, blocks: &[u64], memo: &mut HashMap<(String, Vec<u64>), u64>) -> u64 {
        step(&skip_operational(&s[1..]).to_string(), blocks, memo)
    }

    fn broken_spring(s: &str, blocks: &[u64], memo: &mut HashMap<(String, Vec<u64>), u64>) -> u64 {
        let (block, rest) = s.split_at(blocks[0] as usize);
        if block.chars().all(|c| c == '#' || c == '?') && !rest.starts_with('#') {
            let next_blocks = &blocks[1..];
            if rest.is_empty() {
                if next_blocks.is_empty() {1} else {0}
            } else {
                step(skip_operational(&rest[1..]), next_blocks, memo)
            }
        } else {
            0
        }
    }

    
    if blocks.is_empty() {
        return if s.contains('#') { 0 } else { 1 };
    }
    if s.len() < blocks.iter().sum::<u64>() as usize + blocks.len() - 1 {
        return 0;
    }

    if let Some(&arrangements) = memo.get(&(s.to_string(), blocks.to_vec())) {
        return arrangements;
    }
    
    let mut arrangements = 0;

    if s.starts_with("?") || s.starts_with(".") {
        arrangements += operational_spring(s, blocks, memo);
    }

    if s.starts_with("#") || s.starts_with("?") {
        arrangements += broken_spring(s, blocks, memo);
    }

    memo.insert((s.to_string(), blocks.to_vec()), arrangements);
    
    arrangements
}

fn skip_operational(s: &str) -> &str {
    s.trim_start_matches('.')
}

pub fn count_arrangements_nfa(s: &str, blocks: &[u64]) -> u64 {

    let s = s.chars().collect::<Vec<_>>();

    #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
    struct State {
        si: usize,
        bi: usize,
        bc: usize,
        expect_dot: bool,
    }
    let mut cstates: HashMap<State,u64> = HashMap::new();
    let mut nstates: HashMap<State,u64> = HashMap::new();
    cstates.insert(State { si: 0, bi: 0, bc: 0, expect_dot: false }, 1);
    let mut pos = 0;
    loop {
        
        for (state, count) in &cstates {
            let si = state.si;
            let bi = state.bi;
            let mut bc = state.bc;
            let expect_dot = state.expect_dot;
         
            if state.si == s.len() {
                if state.bi == blocks.len() {
                    pos += count;
                }
                continue;
            }
            if (s[si] == '#' || s[si] == '?') &&  bi < blocks.len() && !expect_dot {
                if s[si] == '?' && bc == 0  {
                    let v =nstates.entry(State { si: si + 1, bi: bi, bc: bc, expect_dot: false }).or_insert(0);
                    *v += count;
                }
                bc += 1;
                if bc == blocks[bi] as usize {
                    let v = nstates.entry(State { si: si + 1, bi: bi + 1, bc: 0, expect_dot: true }).or_insert(0);
                    *v += count;
                } else {
                    let v = nstates.entry(State { si: si + 1, bi: bi, bc: bc, expect_dot: false }).or_insert(0);
                    *v += count;
                }
            } else if (s[state.si] == '.' || s[state.si] == '?') && bc == 0 {
                let v = nstates.entry(State { si: si + 1, bi: bi, bc: bc, expect_dot: false }).or_insert(0);
                *v += count;
            }
        }
        (cstates, nstates) = (nstates, cstates);
        nstates.clear();
        if cstates.is_empty() {
            break;
        }
    }
    pos
}
pub fn count_arrangements(s: &str, blocks: &[u64]) -> u64 {
    let mut memo = HashMap::new();
    step(skip_operational(s), blocks, &mut memo)
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


    #[test]
    fn test_test_count_arrangements_nfa() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let lines = input.lines;
        let expected = vec![1, 4, 1, 1, 4, 10];

        for i in 0..lines.len() {
            assert_eq!(
                count_arrangements_nfa(&lines[i].0, &lines[i].1),
                expected[i],
                "i: {}",
                i
            );
        }
    }
}
