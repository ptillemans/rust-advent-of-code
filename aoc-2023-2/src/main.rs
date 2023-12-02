#![feature(test)]
use aoc_2023_2::{AocError, Bag, InputModel};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let bag = Bag::new(12, 13, 14);
    let sum: u32 = input
        .games
        .iter()
        .filter(|g| g.is_valid(&bag))
        .map(|g| g.game_number)
        .sum();

    return Ok(sum.to_string());
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let result = input
        .games
        .iter()
        .map(|g| g.minimal_bag().power())
        .sum::<u32>()
        .to_string();
    return Ok(result);
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
    use aoc_2023_2::{Game, InputModel, Showing};
    use test::Bencher;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    pub fn input_data() -> InputModel {
        InputModel {
            games: vec![
                Game {
                    game_number: 1,
                    showings: vec![
                        Showing {
                            red: 4,
                            green: 0,
                            blue: 3,
                        },
                        Showing {
                            red: 1,
                            green: 2,
                            blue: 6,
                        },
                        Showing {
                            red: 0,
                            green: 2,
                            blue: 0,
                        },
                    ],
                },
                Game {
                    game_number: 2,
                    showings: vec![
                        Showing {
                            red: 0,
                            green: 2,
                            blue: 1,
                        },
                        Showing {
                            red: 1,
                            green: 3,
                            blue: 4,
                        },
                        Showing {
                            red: 0,
                            green: 1,
                            blue: 1,
                        },
                    ],
                },
                Game {
                    game_number: 3,
                    showings: vec![
                        Showing {
                            red: 20,
                            green: 8,
                            blue: 6,
                        },
                        Showing {
                            red: 4,
                            green: 13,
                            blue: 5,
                        },
                        Showing {
                            red: 1,
                            green: 5,
                            blue: 0,
                        },
                    ],
                },
                Game {
                    game_number: 4,
                    showings: vec![
                        Showing {
                            red: 3,
                            green: 1,
                            blue: 6,
                        },
                        Showing {
                            red: 6,
                            green: 3,
                            blue: 0,
                        },
                        Showing {
                            red: 14,
                            green: 3,
                            blue: 15,
                        },
                    ],
                },
                Game {
                    game_number: 5,
                    showings: vec![
                        Showing {
                            red: 6,
                            green: 3,
                            blue: 1,
                        },
                        Showing {
                            red: 1,
                            green: 2,
                            blue: 2,
                        },
                    ],
                },
            ],
        }
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        actual
            .games
            .iter()
            .zip(expected.games.iter())
            .for_each(|(a, e)| {
                assert_eq!(a.game_number, e.game_number);
                a.showings
                    .iter()
                    .zip(e.showings.iter())
                    .for_each(|(a, e)| assert_eq!(a, e));
            });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "8";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "2286";

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
