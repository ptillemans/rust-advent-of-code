#![feature(test)]
use aoc_2023_11::{AocError, InputModel, position_after_expansion, empty_space};
use itertools::Itertools;

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let sum = sum_combinations(input, 2);
    Ok(sum.to_string())
}

fn sum_combinations(input: &InputModel, age: i32) -> u64 {
    let empty_space = empty_space(input);

    let new_positions = input.galaxies.iter()
        .map(|pos| position_after_expansion(pos, &empty_space, age))
        .collect::<Vec<_>>();

    new_positions.iter()
        .combinations(2)
        .map(|ps| ps[0].manhattan(ps[1]) as u64)
        .sum::<u64>()
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let sum = sum_combinations(input, 1_000_000);
    Ok(sum.to_string())
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

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use aoc_common::position::Position;
    use test::Bencher;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    fn input_data() -> InputModel {
        InputModel {
            galaxies: vec![
                Position::new(3, 0),
                Position::new(7, 1),
                Position::new(0, 2),
                Position::new(6, 4),
                Position::new(1, 5),
                Position::new(9, 6),
                Position::new(7, 8),
                Position::new(0, 9),
                Position::new(4, 9),
            ],
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
        let expected = "374";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sum_combinations() {
        let input = input_data();
        let sum_2 = sum_combinations(&input, 2);
        let sum_10 = sum_combinations(&input, 10);
        let sum_100 = sum_combinations(&input, 100);
        assert_eq!(sum_2, 374);
        assert_eq!(sum_10, 1030);   
        assert_eq!(sum_100, 8410);   
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
