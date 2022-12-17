#![feature(test)]
use aoc_2022_17::*;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    let jets = input.moves.clone().into_iter().cycle();
    let mut chamber = Chamber::new(Box::new(jets));
    Ok(chamber.play_rounds(2022).to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let jets = input.moves.clone();
    let rounds = 1000000000000;
    Ok(play_many_rounds(jets, rounds).to_string())
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
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "3068";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "1514285714288";

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
