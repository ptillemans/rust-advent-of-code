use std::{str::FromStr, collections::HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Rucksack (String);

impl Rucksack {

    fn new(s: &str) -> Self {
        Rucksack(s.to_string())
    }

    pub fn compartments(&self) -> (&str, &str) {
        let len = self.0.len();
        self.0.split_at(len / 2)
    }

    pub(crate) fn common_items(&self) -> HashSet<char> {
        let (left, right) = self.compartments();
        let left: HashSet<char> = HashSet::from_iter(left.chars());
        let right: HashSet<char> = HashSet::from_iter(right.chars());
        left.intersection(&right).cloned().collect()
    }

    pub fn priority(&self) -> Option<usize> {
        priority(self.common_items())
    }

    pub fn unique_items(&self) -> HashSet<char> {
        HashSet::from_iter(self.0.chars())
    }
}

pub fn priority(items: HashSet<char>) -> Option<usize> {
    items.iter()
        .next()
        .map(|&item|
            match item {
                'a'..='z' => item as usize - 'a' as usize + 1,
                'A'..='Z' => item as usize - 'A' as usize + 27,
                _ => 0
            })
}


impl FromStr for Rucksack {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack::new(s))
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub rucksacks: Vec<Rucksack>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| line.parse::<Rucksack>())
            .collect::<Result<Vec<Rucksack>, AocError>>()
            .map(|rucksacks| InputModel { rucksacks })
    }
}


pub const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

pub fn test_input() -> InputModel {
    
    let rucksacks = vec![
        Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
        Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        Rucksack::new("PmmdzqPrVvPwwTWBwg"),
        Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
        Rucksack::new("ttgJtRGJQctTZtZT"),
        Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
    ];
    
    InputModel {
        rucksacks,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = test_input();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_compartments() {
        let rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        let actual = rucksack.compartments();
        let expected = ("vJrwpWtwJgWr", "hcsFMMfFFhFp");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_common_items() {
        let rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        let actual = rucksack.common_items();
        let expected = HashSet::from_iter(vec!['p'].iter().cloned());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_common_items_2() {
        let rucksack = Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let actual = rucksack.common_items();
        let expected = HashSet::from_iter(vec!['L'].iter().cloned());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_common_items_3() {
        let rucksack = Rucksack::new("PmmdzqPrVvPwwTWBwg");
        let actual = rucksack.common_items();
        let expected = HashSet::from_iter(vec!['P'].iter().cloned());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_priorities() {
        let rucksacks = test_input().rucksacks;
        let actual : Vec<usize> = rucksacks.iter().map(|r| r.priority().unwrap()).collect();
        let expected = vec![16, 38, 42, 22, 20, 19];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_unique_items() {
        let rucksack = Rucksack::new("PmmdzqPrVvPwwTWBwg");
        let actual = rucksack.unique_items();
        let expected = HashSet::from_iter(vec!['P', 'm', 'd', 'z', 'q', 'r', 'V', 'v', 'w', 'T', 'W', 'B', 'g'].into_iter());

        assert_eq!(actual, expected);
    }

}
