use std::{str::FromStr, collections::HashMap};

const INPUT: &str = include_str!("../data/input.txt");


#[derive(Debug, PartialEq, Eq)]
struct InputModel  {
    values: Vec<String>,
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
        let values: Vec<String> = s.lines()
            .map(|s| s.to_string())
            .collect();
        Ok(InputModel{ values })
    }
}

fn char_frequency(s: &str) -> HashMap<char, usize> {
    s.chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}

fn part1(input: &InputModel) -> Result<String,AocError> {
    let (n2, n3) = input.values.iter()
        .map(|s| char_frequency(s))
        .map(|freqs| (
            if freqs.values().any(|&x| x == 2) { 1 } else { 0 },
            if freqs.values().any(|&x| x == 3) { 1 } else { 0 }
            )
        )
        .fold((0,0), |(n2, n3), (a, b)| (n2 + a, n3 + b));
    Ok((n2 * n3).to_string())
}

fn hamming_distance(s1: &str, s2: &str) -> usize {
    s1.chars().zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn matching_characters(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect()
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    input.values.iter()
        .flat_map(move |s1|input.values.iter().map(move |s2| (s1, s2)))
        .filter(|(s1, s2)| hamming_distance(s1, s2) == 1)
        .map(|(s1, s2)| matching_characters(s1, s2))
        .next()
        .ok_or(AocError::NoSolution)
}

fn main() -> Result<(), AocError> {
    let input:InputModel = INPUT.parse::<InputModel>()?;
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

    const TEST_INPUT: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    fn input_data() -> InputModel {
        InputModel {
            values: vec!(
                "abcdef".to_string(),
                "bababc".to_string(),
                "abbcde".to_string(),
                "abcccd".to_string(),
                "aabcdd".to_string(),
                "abcdee".to_string(),
                "ababab".to_string(),
            )
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
        let expected = "12";

        assert_eq!(actual, expected);
    }

    const DATA_TEST2: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    #[test]
    fn test_part2() {
        let actual = part2(&DATA_TEST2.parse::<InputModel>().unwrap()).unwrap();
        let expected = "fgij";
        
        assert_eq!(actual, expected);
    }
}
