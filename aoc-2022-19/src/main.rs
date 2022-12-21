#![feature(test)]

use aoc_2022_19::*;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(_input: &InputModel) -> Result<String,AocError> {
    let qualities: Vec<u32> = _input.blueprints.iter()
        .map(BluePrint::quality)
        .collect();
    Ok(qualities.iter().sum::<u32>().to_string())
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    let qualities: Vec<u32> = _input.blueprints.iter()
        .take(3)
        .map(|blue_print| blue_print.max_geodes(32))
        .collect();
    Ok(qualities.iter().product::<u32>().to_string())
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
        let expected = "33";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let blue_prints = input_data().blueprints;
        let actual = blue_prints[0].max_geodes(32);
        let expected = 56;
        assert_eq!(actual, expected);
        
        let actual = blue_prints[1].max_geodes(32);
        let expected = 62;
        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&input))
    }

}
