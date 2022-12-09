#![feature(test)]
use aoc_2022_9::*;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    Ok(tail_coverage(&Rope::new(2), &input.motions).len().to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    Ok(tail_coverage(&Rope::new(10), &input.motions).len().to_string())
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

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    pub fn input_data() -> InputModel {
        let motions = vec![
            Motion{direction: Direction::Right, distance: 4},
            Motion{direction: Direction::Up, distance: 4},
            Motion{direction: Direction::Left, distance: 3},
            Motion{direction: Direction::Down, distance: 1},
            Motion{direction: Direction::Right, distance: 4},
            Motion{direction: Direction::Down, distance: 1},
            Motion{direction: Direction::Left, distance: 5},
            Motion{direction: Direction::Right, distance: 2},
        ];
        InputModel { motions }
    }

    const LARGER_EXAMPLE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "13";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "1";

        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_part2_larger() {
        let input = LARGER_EXAMPLE.parse::<InputModel>().unwrap();
        let actual = part2(&input).unwrap();
        let expected = "36";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input:InputModel = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input:InputModel = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&input))
    }

}
