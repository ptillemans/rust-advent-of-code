#![feature(test)]
use aoc_2023_1::{AocError, InputModel};

const INPUT: &str = include_str!("../data/input.txt");


fn find_first_and_last(line: &str) -> Option<(u32, u32)> {
    let spelled_numbers: Vec<(String, u32)> = vec![
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
        ("zero".to_string(), 0),
        ("1".to_string(), 1),
        ("2".to_string(), 2),
        ("3".to_string(), 3),
        ("4".to_string(), 4),
        ("5".to_string(), 5),
        ("6".to_string(), 6),
        ("7".to_string(), 7),
        ("8".to_string(), 8),
        ("9".to_string(), 9),
    ];

    let mut occurrences = spelled_numbers.iter()
        .flat_map(|(spelled, digit)|
                  if let Some(first) = line.find(spelled) {
                      if let Some(last) = line.rfind(spelled) {
                         if first != last {
                             vec![(first, digit) , (last, digit)]
                         } else {
                             vec![(first,digit)]
                         }
                      } else {
                         vec![(first, digit)] 
                      }
                  } else {
                      vec![]
                      })
        .map(|(i, digit)| (i, *digit))
        .collect::<Vec<(usize, u32)>>();

    occurrences.sort();

    let digits = occurrences.iter()
        .map(|(_, digit)| *digit)
        .collect::<Vec<u32>>();

    digits.first().and_then(|first| digits.last().map(|last| (*first, *last)))
}

fn line_to_number(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    let n = 10*first + last;
    
    return n;
}

fn lines_to_number(lines: Vec<String>) -> Vec<u32> {
    lines.iter()
        .map(|line| line_to_number(line))
        .collect()
}
fn part1(_input: &InputModel) -> Result<String,AocError> {
    let sum = lines_to_number(_input.lines.clone()).iter().sum::<u32>();
    return Ok(sum.to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let sum = input.lines.iter()
        .filter_map(|line| find_first_and_last(line))
        .map(|(first, last)| first * 10 + last)
        .sum::<u32>();
    return Ok(sum.to_string())
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
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    pub fn input_data() -> InputModel {
        InputModel {
            lines: TEST_INPUT.lines().map(|l| l.to_string()).collect(),
        }
    }

    pub fn input_data_2() -> InputModel {
        InputModel {
            lines: TEST_INPUT_2.lines().map(|l| l.to_string()).collect(),
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
        let expected = "142";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data_2()).unwrap();
        let expected = "281";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(&input_data()))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(&input_data()))
    }

}
