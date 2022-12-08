#![feature(test)]
use aoc_2022_8::{AocError, InputModel, find_visible};

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    Ok(find_visible(&input.trees).into_iter()
        .flatten()
        .filter(|&x| x)
        .count()
        .to_string())
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    return Ok("Not implemented".to_string())
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

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    pub fn input_data() -> InputModel {
        InputModel {
            trees: vec![
                vec!['3', '0', '3', '7', '3', ],
                vec!['2', '5', '5', '1', '2', ],
                vec!['6', '5', '3', '3', '2', ],
                vec!['3', '3', '5', '4', '9', ],
                vec!['3', '5', '3', '9', '0', ],
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
        let expected = "21";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(&input_data()))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(&input_data()))
    }

}
