#![feature(test)]
use aoc_2023_9::{AocError, InputModel, next_number, prev_number};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
   let result = input.measurements.iter()
        .filter_map(|measurement| next_number(measurement)) 
        .sum::<i64>()
        .to_string();
    Ok(result)
}

fn part2(input: &InputModel) -> Result<String, AocError> {
   let result = input.measurements.iter()
        .filter_map(|measurement| prev_number(measurement)) 
        .sum::<i64>()
        .to_string();
    Ok(result)
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

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    pub fn input_data() -> InputModel {
        InputModel {
            measurements: vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
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
        let expected = "114";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "2";

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
