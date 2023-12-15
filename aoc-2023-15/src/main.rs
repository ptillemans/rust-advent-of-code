#![feature(test)]
use aoc_2023_15::{AocError, InputModel, hash, parse_instruction, execute, focussing_power};

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    let s = input.instructions.iter()
        .map(|s| hash(s))
        .sum::<u32>()
        .to_string();

    Ok(s)
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let boxes = execute(&input.instructions);
    let s = focussing_power(&boxes).to_string();
    Ok(s.to_string())
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

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7

";

    pub fn input_data() -> InputModel {
        InputModel {
            instructions: vec!["rn=1".to_string(),"cm-".to_string(),"qp=3".to_string(),"cm=2".to_string(),"qp-".to_string(),"pc=4".to_string(),"ot=9".to_string(),"ab=5".to_string(),"pc-".to_string(),"pc=6".to_string(),"ot=7".to_string()],
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
        let expected = "1320";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "145";

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
