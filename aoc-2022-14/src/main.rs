#![feature(test)]
use aoc_2022_14::*;
use std::iter::from_fn;
use aoc_common::position::Position;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    let drop_pos = Position::new(500, 0);
    let mut cave = Cave::new(&input.paths);
    let count = from_fn(|| cave.drop_sand(&drop_pos))
        .count();
    Ok(count.to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let drop_pos = Position::new(500, 0);
    let mut cave = Cave::new(&input.paths);
    let count = from_fn(|| cave.drop_sand_with_floor(&drop_pos))
        .count();
    Ok(count.to_string())
}

fn main() -> Result<(), AocError> {
    let input:InputModel = input_data()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

fn input_data() -> Result<InputModel, AocError> {   
    INPUT.parse::<InputModel>()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = test_input();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&test_input()).unwrap();
        let expected = "24";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&test_input()).unwrap();
        let expected = "93";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = input_data().unwrap();
        b.iter(|| part1(&input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = input_data().unwrap();
        b.iter(|| part2(&input))
    }

}
