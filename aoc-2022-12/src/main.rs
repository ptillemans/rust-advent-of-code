#![feature(test)]
use aoc_2022_12::*;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    let start = find_start(input);
    let end = find_end(input);
    let steps = shortest_path_bfs(input, &start, &end)?;
    return Ok(steps.to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let end = find_end(input);
    let steps = scenic_route(input, &end)?;
    return Ok(steps.to_string())
}

fn part1_astar(input: &InputModel) -> Result<String,AocError> {
    let start = find_start(input);
    let end = find_end(input);
    let steps = shortest_path(input, &start, &end).map(|path| path.len())?;
    return Ok(steps.to_string())
}

fn part2_astar(input: &InputModel) -> Result<String, AocError> {
    let end = find_end(input);
    let steps = scenic_route_astar(input, &end)?;
    return Ok(steps.to_string())
}

fn main() -> Result<(), AocError> {
    let input:InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("Part1: {}", part1_astar(&input)?);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    println!("Part2: {}", part2_astar(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    pub fn input_data() -> InputModel {
        test_data()
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
        let expected = "31";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "29";

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

    #[bench]
    fn bench_part1_astar(b: &mut Bencher) {
        b.iter(|| part1_astar(&input_data()))
    }

    #[bench]
    fn bench_part2_astar(b: &mut Bencher) {
        b.iter(|| part2_astar(&input_data()))
    }
}
