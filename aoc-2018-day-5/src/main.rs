use std::str::FromStr;

use itertools::iterate;

const INPUT: &str = include_str!("../data/input.txt");

#[derive(Debug, PartialEq, Eq)]
struct InputModel  {
    polymer: String
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution")]
    NoSolution,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InputModel {
            polymer: s.to_string()
        })
    }
}

fn is_reverse_polarity(a: char, b: char) -> bool {
    (a.to_lowercase().next() == b.to_lowercase().next()) &&
     (a.is_uppercase() && b.is_lowercase() || a.is_lowercase() && b.is_uppercase())
}

fn perform_reaction(polymer: &str) -> String {
    let chars = polymer.chars().collect::<Vec<char>>();
    let pair = chars.windows(2)
        .filter(|cs| is_reverse_polarity(*cs.get(0).unwrap(), *cs.get(1).unwrap()))
        .next();

    match pair {
        Some(cs) => {
            let p: String = cs.into_iter().collect();
            polymer.replacen(&p, "", 1)
        }
        _ => polymer.to_string()
    }
}


fn part1(input: &InputModel) -> Result<String,AocError> {
    iterate(input.polymer.to_owned(), |s| perform_reaction(&s))
        .scan("".to_string(), |last, new| 
            if *last == new { 
                None
            } else {
                *last = new.to_owned(); 
                Some(new)
            })
        .last()
        .map(|s| s.len().to_string())
        .ok_or(AocError::NoSolution)
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    return Ok("Not implemented".to_string())
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

    const TEST_INPUT: &str = "dabAcCaCBAcCcaDA";

    fn input_data() -> InputModel {
        InputModel {
            polymer: TEST_INPUT.to_string()
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
        let expected = "10";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }

    const REACTION_DATA: &[(&str, &str)] = &[
        ("aA", ""),
        ("Aa", ""),
        ("aa", "aa"),
        ("AA", "AA"),
        ("aBbA", "aA"),
        ("abAB", "abAB"),
        ("aabAAB", "aabAAB"),
    ];

    #[test]
    fn test_perform_reaction() {
        for (input, expected) in REACTION_DATA {
            let actual = perform_reaction(input);
            assert_eq!(&actual, expected);
        }
    }

    const REACTION_STEPS: &[&str] = &[
        "dabAcCaCBAcCcaDA",
        "dabAaCBAcCcaDA",
        "dabCBAcCcaDA",
        "dabCBAcaDA",
        "dabCBAcaDA",
    ];

    #[test]
    fn test_perform_reaction_steps() {
        let steps = REACTION_STEPS.iter()
            .zip(REACTION_STEPS[1..].iter());
        for (input, expected) in steps {
            println!("{} {}", input, expected);
           assert_eq!(perform_reaction(input), *expected)
        }
    }


}
