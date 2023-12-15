#![feature(test)]
use std::iter::repeat;

use aoc_2023_12::{count_arrangements, AocError, InputModel, count_arrangements_nfa};
use rayon::prelude::*;

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    Ok(input
        .lines
        .par_iter()
        .map(|(line, blocks)| count_arrangements(line, blocks))
        .sum::<u64>()
        .to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    Ok(input.lines
        .par_iter()
        .map(|(line, blocks)| {
            (
                repeat(line)
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("?"),
                repeat(blocks)
                    .take(5)
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(line, blocks)| count_arrangements(&line, &blocks))
        .sum::<u64>()
        .to_string())
}

fn part2_nfa(input: &InputModel) -> Result<String, AocError> {
    Ok(input.lines
        .par_iter()
        .map(|(line, blocks)| {
            (
                repeat(line)
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("?"),
                repeat(blocks)
                    .take(5)
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(line, blocks)| count_arrangements_nfa(&line, &blocks))
        .sum::<u64>()
        .to_string())
}

fn main() -> Result<(), AocError> {
    let input: InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    let part2_result = part2_nfa(&input)?;
    println!("Part2 NFA: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    pub fn input_data() -> InputModel {
        TEST_INPUT.parse::<InputModel>().unwrap()
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
        let expected = "21";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "525152";

        assert_eq!(actual, expected);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&data))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&data))
    }

    #[bench]
    fn bench_part2_nfa(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2_nfa(&data))
    }
}
