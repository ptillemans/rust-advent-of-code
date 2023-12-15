#![feature(test)]
use aoc_2023_13::{find_horizontal_mirror, find_vertical_mirror, AocError, InputModel, find_horizontal_smudge, find_vertical_smudge};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let score = input
        .mirrors
        .iter()
        .map(|m| score(m.clone()))
        .sum::<usize>();
    Ok(score.to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let score = input
        .mirrors
        .iter()
        .map(|m| smudge_score(m.clone()))
        .sum::<usize>();
    Ok(score.to_string())
}

fn main() -> Result<(), AocError> {
    let input: InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

fn score(mirror: Vec<String>) -> usize {
    let mut score = 0;
    if let Some(i) = find_horizontal_mirror(&mirror) {
        score += i;
    }
    if let Some(i) = find_vertical_mirror(&mirror) {
        score += i * 100;
    }
    score
}

fn smudge_score(mirror: Vec<String>) -> usize {
    let mut score = 0;
    if let Some(i) = find_horizontal_smudge(&mirror) {
        score += i;
    }
    if let Some(i) = find_vertical_smudge(&mirror) {
        score += i * 100;
    }
    score
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

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
        let expected = "405";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "400";

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
}
