#![feature(test)]
use aoc_2023_4::{AocError, InputModel, ScratchCard};
const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let sum = input.cards.iter()
        .map(count_winning_numbers)
        .map(|n| match n {
            0 => 0,
            n => (2 as u32).pow(n as u32 -1)
        })
        .sum::<u32>();
    return Ok(sum.to_string());
}

fn count_winning_numbers(card: &ScratchCard) -> usize {
    card.winning.iter()
        .filter(|&n| card.numbers.contains(n))
        .count()
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let mut counts = input.cards.iter()
        .map(|_| 1)
        .collect::<Vec<u32>>();
    let winnings = input.cards.iter()
        .map(|card| count_winning_numbers(card))
        .collect::<Vec<usize>>();

    for i in 0..input.cards.len() {
        let win = winnings[i];
        let count = counts[i];
        for j in 1..=win {
            counts[i + j] += count;
        }
    }

    Ok(counts.iter().sum::<u32>().to_string())
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


    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    pub fn input_data() -> InputModel {
        InputModel {
            cards: vec![
                ScratchCard {
                    id: 1,
                    winning: vec![41, 48, 83, 86, 17],
                    numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                },
                ScratchCard {
                    id: 2,
                    winning: vec![13, 32, 20, 16, 61],
                    numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
                },
                ScratchCard {
                    id: 3,
                    winning: vec![1, 21, 53, 59, 44],
                    numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
                },
                ScratchCard {
                    id: 4,
                    winning: vec![41, 92, 73, 84, 69],
                    numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
                },
                ScratchCard {
                    id: 5,
                    winning: vec![87, 83, 26, 28, 32],
                    numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
                },
                ScratchCard {
                    id: 6,
                    winning: vec![31, 18, 13, 56, 72],
                    numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
                },
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
        let expected = "13";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "30";

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
