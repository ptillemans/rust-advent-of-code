#![feature(test)]
use aoc_2023_10::{find_loop, AocError, InputModel, Pipe, Section};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let path = find_loop(&input.pipes).unwrap();
    let farthest = path.len() / 2;
    Ok(farthest.to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let pipes = input.pipes.clone();
    let mut path = find_loop(&pipes).unwrap();

    let start_symbol = start_symbol(&path);

    let pl = path.len();
    let start = Pipe{loc: path[pl - 1].loc, section: start_symbol};
    path[pl - 1] = start;

    let max_x = path.iter().map(|p| p.loc.0).max().unwrap();
    let max_y = path.iter().map(|p| p.loc.1).max().unwrap();
    
    let inner = (0..=max_x)
        .flat_map(|x| (0..=max_y).map(move |y| (x, y)))
        .filter(|(x, y)| path.iter().find(|p| (*p).loc == (*x, *y)).is_none())
        .filter(|(x, y)| {
            let to_left = (0..*x)
                .filter_map(|tx| path.iter().find(|p| p.loc == (tx, *y)))
                .map(|p| p.section)
                .collect::<Vec<_>>();
            let crossings = to_left.iter()
                .fold((None, 0),|(last_corner, count), section| 
                    match section {
                        Section::Horizontal => (last_corner, count),
                        Section::Vertical => (last_corner, count + 1),
                        Section::CornerNE => (Some(Section::CornerNE), count),
                        Section::CornerSE => (Some(Section::CornerSE), count),
                        Section::CornerSW => match last_corner {
                            Some(Section::CornerNE) => (None, count + 1),
                            Some(Section::CornerSE) => (None, count),
                            _ => panic!("Unexpected corner: {:?}", section),
                        },
                        Section::CornerNW => match last_corner {
                            Some(Section::CornerNE) => (None, count),
                            Some(Section::CornerSE) => (None, count + 1),
                            _ => panic!("Unexpected corner: {:?}", section),
                        },
                        _ => panic!("Unexpected section: {:?}", section),
                    }
                ).1;
            crossings % 2 == 1
                
        })
        .collect::<Vec<_>>();

    let n = inner.len();

    Ok(n.to_string())
}


fn start_symbol(path: &[Pipe]) -> Section {
    let pl = path.len();
    let start = path[pl - 1];
    let first = path[0];
    let last = path[pl - 2];

    let delta = (start.loc.0 - first.loc.0, start.loc.1 - first.loc.1,
                 last.loc.0 - start.loc.0, last.loc.1 - start.loc.1);

    match delta {
        (0, -1, 0, -1) | (0, 1, 0, 1) => Section::Vertical,
        (1, 0, 1, 0) | (-1, 0, -1, 0) => Section::Horizontal,
        (0, 1, 1, 0) | (-1, 0, 0, -1) => Section::CornerNE,
        (0, -1, 1, 0) | (-1, 0, 0, 1) => Section::CornerSE,
        (1, 0, 0, 1) | (0, -1, -1, 0) => Section::CornerSW,
        (1, 0, 0, -1) | (0, 1, -1, 0) => Section::CornerNW,
        _ => panic!("Unexpected delta: {:?}", delta),
    }

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

    #[test]
    fn test_part2() {
        let test_input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let actual = part2(&test_input.parse().unwrap()).unwrap();
        let expected = "4";

        assert_eq!(actual, expected);

        let test_input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let actual = part2(&test_input.parse().unwrap()).unwrap();
        let expected = "8";
        assert_eq!(actual, expected);

        let test_input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let actual = part2(&test_input.parse().unwrap()).unwrap();
        let expected = "10";

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
