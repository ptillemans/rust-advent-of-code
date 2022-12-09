#![feature(test)]
use aoc_2022_6::{AocError, InputModel, find_packet_start};

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    find_packet_start(&input.datastream, 4)
        .map(|pos| pos.to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    find_packet_start(&input.datastream, 14)
        .map(|pos| pos.to_string())
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

    const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    pub fn input_data() -> InputModel {
        InputModel {
            datastream: TEST_INPUT.to_owned(),
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
        let expected = "7";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "19";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>());
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input:InputModel = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input:InputModel = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&input))
    }

}
