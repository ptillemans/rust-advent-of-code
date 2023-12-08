#![feature(test)]
#![feature(portable_simd)]

use aoc_2023_6::{AocError, InputModel};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let result = input.races.iter()
        .map(|(t, d)| solve_equation(*t as i64, *d as i64))
        .map(|(x1, x2)| x2 - x1 + 1)
        .product::<i64>();

    Ok(result.to_string())
   
}


fn solve_equation(t: i64, d: i64) -> (i64, i64) {

    let disc = ((t*t - 4*d) as f64).sqrt();
    let x1 = (t as f64 - disc) / 2.0;
    let x2 = (t as f64 + disc) / 2.0;

    let mut x1: i64 = x1.ceil() as i64;
    let mut x2: i64 = x2.floor() as i64;

    while x1 * (t - x1) <= d {
        x1 += 1;
    }
    while x2 * (t - x2) <= d {
        x2 -= 1
    }
    println!("{} {} {} {}", t, d, x1, x2);
    (x1, x2)
    
}
fn part2(input: &InputModel) -> Result<String, AocError> {
    let (x1, x2) = solve_equation(input.part2_time as i64, input.part2_distance as i64);
    let result = x2 - x1 + 1;
    Ok(result.to_string())
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

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    pub fn input_data() -> InputModel {
        InputModel {
            races: vec![(7, 9), (15, 40), (30, 200)],
            part2_time: 71530,
            part2_distance: 940200,
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
        let expected = "288";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "71503";

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
