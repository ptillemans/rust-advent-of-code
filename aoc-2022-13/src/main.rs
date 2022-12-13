#![feature(test)]
use aoc_2022_13::*;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    // sum of all 1-based indices or pairw where the left < right
    let sum: usize = input.pairs.iter()
        .enumerate()
        .filter_map(|(i, (left,right))| {
            if left < right { Some(i+1)} else { None }
        })
        .sum();
    Ok(sum.to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    Ok(decoder_key(&input).to_string())
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

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = test_input();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&test_input()).unwrap();
        let expected = "13";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&test_input()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(&test_input()))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(&test_input()))
    }

}
