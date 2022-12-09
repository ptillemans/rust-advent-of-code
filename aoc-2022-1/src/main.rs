#![feature(test)]
use aoc_2022_1::{AocError, InputModel};
use itertools::sorted;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    // return max of the sum of each vec 
    input.calories.iter()
        .map(|chunk| chunk.iter().sum())
        .max()
        .map(|max: u32| max.to_string())
        .ok_or(AocError::NoMaxFound)
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let total_calories = input.calories.iter()
        .map(|chunk| chunk.iter().sum::<u32>());
    Ok(sorted(total_calories)
        .rev()
        .take(3)
        .sum::<u32>()
        .to_string())
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
    
    pub fn input_data() -> InputModel {
        InputModel {
            calories: vec![
                vec![1000, 2000, 3000], 
                vec![4000], 
                vec![5000, 6000], 
                vec![7000, 8000, 9000], 
                vec![10000]
            ],
        }
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "24000";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "45000";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>());
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input:InputModel = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input:InputModel = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&input));
    }
}
