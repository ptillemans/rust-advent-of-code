use std::str::FromStr;

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
            polymer: s.trim().to_string()
        })
    }
}

fn foldr<T,U>(mut xs: impl Iterator<Item=T>, init: U, f: fn(T, U) -> U) -> U {
    match xs.next() {
        None => init,
        Some(x) => f(x, foldr(xs, init, f))
    }
}

fn toggle_char(c: char) -> char {
     match c {
        c if c.is_lowercase() => c.to_uppercase().next().unwrap(),
        c if c.is_uppercase() => c.to_lowercase().next().unwrap(),
        c => c
    }
}

fn react_head(c: char, s: String) -> String {
    if s.is_empty() {
        return c.to_string()
    }
    
    if s.starts_with(toggle_char(c)) {
        s[1..].to_string()
    } else {
        c.to_string() + &s
    }.to_string()
}

fn perform_reaction(input: &str) -> String {
    foldr(input.chars(), "".to_string(), react_head)
}

fn part1(input: &InputModel) -> Result<String, AocError> {
    let result = perform_reaction(&input.polymer);
    Ok(result.len().to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let base_polymer = input.polymer.to_owned();
    let minimal_length = ('a'..='z')
        .map(|c| base_polymer
             .replace(c, "")
             .replace(c.to_uppercase().next().unwrap(), ""))
        .map(|polymer| perform_reaction(&polymer).len())
        .min();
        
    minimal_length
        .map(|n| n.to_string())
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
        let expected = "4";

        assert_eq!(actual, expected);
    }

}

