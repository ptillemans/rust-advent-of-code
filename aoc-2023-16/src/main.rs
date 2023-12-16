#![feature(test)]
use rayon::prelude::*;

use aoc_2023_16::{AocError, Cave, InputModel, Direction};
use aoc_common::position::Position;

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let energized = input.cave.energized_tiles((0, 0).into(), Direction::East);
    Ok(energized.len().to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let mut start_combinations: Vec<(Position, Direction)> = Vec::with_capacity(2*(input.cave.width + input.cave.length));
    for x in 0..input.cave.get_width() as i32 {
        start_combinations.push(((x, 0).into(), Direction::South));
        start_combinations.push(((x, input.cave.get_length() as i32 - 1).into(), Direction::North));
    }
    for y in 0..input.cave.get_length() as i32 {
        start_combinations.push(((0, y).into(), Direction::East));
        start_combinations.push(((input.cave.get_width() as i32 - 1, y).into(), Direction::West));
    }
    start_combinations.par_iter().map(|(pos, dir)|
                                      input.cave.energized_tiles(*pos, *dir).len()
    )
        .max()
        .map(|l| l.to_string())
        .ok_or(AocError::ParseError)
    
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
    use test::Bencher;

    const TEST_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    pub fn input_data() -> InputModel {
        let cave = TEST_INPUT.parse::<Cave>().unwrap();
        InputModel { cave }
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
        let expected = "46";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "51";

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
