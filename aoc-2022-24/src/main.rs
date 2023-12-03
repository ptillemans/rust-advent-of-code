#![feature(test)]
use aoc_2022_24::*;

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &mut Valley) -> Result<String, AocError> {
    let walker = Walker::new(input);
    let finish = input.finish;
    let path = walker.best_path(input, &finish);
    Ok(path.unwrap().time).map(|x| x.to_string())
}

fn part2(input: &mut Valley) -> Result<String, AocError> {
    let walker = Walker::new(input);
    let start = input.start;
    let finish = input.finish;
    let walker = walker
        .best_path(input, &finish)
        .and_then(|w| w.best_path(input, &start))
        .and_then(|w| w.best_path(input, &finish));
    Ok(walker.unwrap().time).map(|x| x.to_string())
}

fn main() -> Result<(), AocError> {
    let mut input: Valley = INPUT.parse::<Valley>()?;
    let part1_result = part1(&mut input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&mut input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    pub fn input_data() -> Valley {
        TEST_INPUT.parse::<Valley>().unwrap()
    }

    #[test]
    fn test_parse() {
        let valley = TEST_INPUT.parse::<Valley>().unwrap();
        let actual = valley.to_string();
        let expected = TEST_INPUT;

        assert_eq!(actual.trim(), expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&mut input_data()).unwrap();
        let expected = "18";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&mut input_data()).unwrap();
        let expected = "54";

        assert_eq!(actual, expected);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<Valley>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(&mut input_data()))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(&mut input_data()))
    }
}
