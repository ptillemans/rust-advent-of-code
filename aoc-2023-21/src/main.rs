#![feature(test)]
use std::io::Write;
use std::path::Path;
use std::{collections::HashSet, io::Read};
use std::fs::File;

use aoc_2023_21::{reachable_in_steps, AocError, InputModel, cyclic_reachable, Tile};
use aoc_common::grid::Grid;
use aoc_common::position::Position;

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let garden = input.garden.clone();
    let start = input.start;
    let reachable = reachable_in_steps(&garden, start, 64);
    return Ok(reachable.len().to_string());
}

const ELF_STEPS: i64 = 26501365;

fn simulate(garden: &Grid<Tile>, start: Position, steps: i64) ->Vec<i64> {
    let path = Path::new("counts.txt");
    if path.exists() {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let counts: Vec<i64> = buf.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
        return counts;
    }
    let mut current: HashSet<Position> = [start].iter().cloned().collect();
    let counts: Vec<i64> = (0..steps).map(|i| {
        current = cyclic_reachable(&garden, &current);
        if i % garden.width() as i64 == 0 {
            println!("i: {}  l: {}", i, current.len());
        }
        current.len() as i64
    }).collect::<Vec<_>>();
    let mut file = File::create("counts.txt").unwrap();
    file.write_all(counts.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",").as_bytes()).unwrap();
    counts
}


fn part2(input: &InputModel) -> Result<String, AocError> {
    let garden = input.garden.clone();
    let width = garden.width() as i64;

    let counts = simulate(&garden, input.start, 5*width);
    let cycles:  i64 = ELF_STEPS / width;
    let remaining: i64 = ELF_STEPS % width - 1;
    println!("cycles: {} remaining: {}", cycles, remaining);


    let mut len: i64 = 0;
    let mut last: i64 = 0;
    let mut last_d1: i64 = 0;
    let mut last_d2: i64 = 0;
    let mut i: i64 = 0;
    while i < 5 {
        len = counts[(i*width + remaining) as usize];
        let d1 = len - last;
        let d2 = d1 - last_d1;

        if d2 == last_d2 {
            println!("i: {}  l: {} d1: {} d2: {}", i, len, d1, d2);
            break;
        }
        last = len;
        last_d1 = d1;
        last_d2 = d2;
        i += 1;
        println!("i: {}  l: {} d1: {} d2: {}", i, len, d1, d2)

    };

    let start_cycle = i - 1;
    let d2 = last_d2;
    let d1 = last_d1;
    let len = last;
    println!("start_cycle: {}  d1: {} d2: {}", start_cycle, d1, d2);

    println!("len: {} -> {}", len, counts[(start_cycle*width+remaining) as usize]);

    let f = |n: i64| {
        len + d1*n + d2*n*(n+1)/2
    };
    for i in 0..3 as i64 {
        let ref_len = counts[((start_cycle + i)*width+remaining) as usize];
        let est = f(i);
        println!("{}: {} -> {}", i, ref_len, est);
    }


    let final_len = f(cycles - start_cycle);
    let final_steps = cycles*width + remaining;
    Ok(final_len.to_string())

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

    const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
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
        let expected = "42";

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
