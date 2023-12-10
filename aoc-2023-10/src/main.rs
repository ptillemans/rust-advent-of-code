#![feature(test)]
use aoc_2023_10::{AocError, InputModel, Pipe, Section, find_start};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let pipes = &input.pipes;
    let start = find_start(&pipes);
    println!("start {:?}", start);

    let next_pipes = start.connecting_pipes(pipes);
    let paths = start
        .connecting_pipes(pipes)
        .iter()
        .filter_map(|pipe| pipe.find_path(pipes, &start))
        .filter(|path| {
            matches!(
                path.last(),
                Some(Pipe {
                    loc: _,
                    section: Section::Start
                })
            )
        })
        .collect::<Vec<Vec<Pipe>>>();

    println!("{}", paths.len());
    for path in paths.iter() {
        println!("{:?}", path);
    }

    assert!(paths.len() == 2);
    assert!(paths[0].len() == paths[1].len());
    let farthest = paths[0].len() / 2;
    Ok(farthest.to_string())
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    return Ok("Not implemented".to_string());
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

    const TEST_INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

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
        let expected = "4";

        assert_eq!(actual, expected);
    }

    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>().unwrap())
    }

    fn bench_part1(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&data))
    }

    fn bench_part2(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&data))
    }
}
