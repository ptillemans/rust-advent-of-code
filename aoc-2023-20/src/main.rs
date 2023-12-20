#![feature(test)]
use aoc_2023_20::{AocError, InputModel, Pulse};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let mut pulses = vec![];
    let mut circuit = input.circuit.clone();
    (0..1000).for_each(|_| {
        let new_pulses = circuit.push_button();
        pulses.extend(new_pulses);
    });
    let high_count = pulses
        .iter()
        .filter(|&x| matches!(x, Pulse::High(_, _)))
        .count();
    let low_count = pulses
        .iter()
        .filter(|&x| matches!(x, Pulse::Low(_, _)))
        .count();
    let result = high_count * low_count;
    Ok(result.to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let mut circuit = input.circuit.clone();
    let cycles = circuit.find_cycles();

    let inputs_rx = circuit.inputs("rx");

    let inputs_trigger = inputs_rx.iter()
        .flat_map(|s| circuit.inputs(s))
        .flat_map(|s| circuit.inputs(&s))
        .collect::<Vec<String>>();


    let lens:Vec<usize> = inputs_trigger.into_iter()
        .filter_map(|s| cycles.get(&s))
        .map(|(_, l)| *l)
        .collect();

    let big_cycle: u64 = lens.into_iter()
        .map(|s| s as u64)
        .product();
   
    Ok(big_cycle.to_string())
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

    const TEST_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

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
        let expected = "32000000";

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
