#![feature(test)]
use aoc_2023_8::{AocError, InputModel, Direction};
use num::integer::lcm;

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let steps = input.directions.iter()
        .cycle()
        .scan("AAA".to_string(), |state, direction| {
            if state == "ZZZ" {
                return None;
            }
            
            let (a, b) = input.nodes.get(state).unwrap();
            *state = match direction {
                Direction::Left => a.to_string(),
                Direction::Right => b.to_string(),
            };
            Some(state.clone())
        })
        .count();
    Ok(steps.to_string())
    
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let start_nodes = input.nodes.keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<String>>();
    let directions = &input.directions;
    let result = start_nodes.iter()
        .map(|start_node| {
        let steps = directions.iter()
            .cycle()
            .scan(start_node.clone(), |node, direction| {
                if node.ends_with('Z') {
                    return None;
                }
             
                let (a, b) = input.nodes.get(node).unwrap();
                *node = match direction {
                    Direction::Left => a.to_string(),
                    Direction::Right => b.to_string(),
                };

                Some(())
            })
            .count();
        steps
        })
        .fold(1_usize, lcm);

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
    use std::collections::HashMap;

    use super::*;
    use aoc_2023_8::Direction;
    use test::Bencher;

    const TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    
    const TEST_INPUT_1_B: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";


    pub fn input_data() -> InputModel {
        let directions = vec![Direction::Right, Direction::Left];
        let mut nodes = HashMap::new();
        nodes.insert("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()));
        nodes.insert("BBB".to_string(), ("DDD".to_string(), "EEE".to_string()));
        nodes.insert("CCC".to_string(), ("ZZZ".to_string(), "GGG".to_string()));
        nodes.insert("DDD".to_string(), ("DDD".to_string(), "DDD".to_string()));
        nodes.insert("EEE".to_string(), ("EEE".to_string(), "EEE".to_string()));
        nodes.insert("GGG".to_string(), ("GGG".to_string(), "GGG".to_string()));
        nodes.insert("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string()));
        InputModel { directions, nodes }
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
        let expected = "2";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_2() {
        let input = TEST_INPUT_1_B.parse::<InputModel>().unwrap();
        let actual = part1(&input).unwrap();
        let expected = "6";

        assert_eq!(actual, expected);
    }

    const TEST_INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    
    #[test]
    fn test_part2() {
        let input = TEST_INPUT_2.parse::<InputModel>().unwrap();
        let actual = part2(&input).unwrap();
        let expected = "6";

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
