#![feature(test)]
use aoc_2022_15::*;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel, y: i32) -> Result<String,AocError> {
    let sensors = input.sensors.clone();
    let cave = Cave::new(sensors);
    let covered = cave.covered_positions(y);
    Ok(covered.to_string())
}

fn part2(input: &InputModel, bound : i32) -> Result<String, AocError> {
    let sensors = input.sensors.clone();
    let cave = Cave::new(sensors);
    let freq = cave.tuning_frequency(bound).unwrap();
    Ok(freq.to_string())
}

fn main() -> Result<(), AocError> {
    let input:InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input, 2_000_000)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input, 4_000_000)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data(), 10).unwrap();
        let expected = "26";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data(), 20).unwrap();
        let expected = "56000011";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&input, 2_000_000))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&input, 4_000_000))
    }

}
