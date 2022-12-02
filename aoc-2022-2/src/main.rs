use aoc_2022_2::{AocError, InputModel, Play, score, play_for_outcome, Outcome};
use phf::phf_map;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(_input: &InputModel) -> Result<String,AocError> {
    let player1_mapping = phf_map![
        'A' => Play::Rock,
        'B' => Play::Paper,
        'C' => Play::Scissors,
    ];
    let player2_mapping = phf_map![
        'X' => Play::Rock,
        'Y' => Play::Paper,
        'Z' => Play::Scissors,
    ];
    let score: i32 = _input.plays.iter()
        .filter_map(|(player1, player2)| {
            player1_mapping.get(player1).and_then(|player1| {
                player2_mapping.get(player2).map(|player2| {
                    score(*player1, *player2)
                })
            })
        })
        .sum();
    Ok(score.to_string())
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    let player1_mapping = phf_map![
        'A' => Play::Rock,
        'B' => Play::Paper,
        'C' => Play::Scissors,
    ];
    let outcome_mapping = phf_map![
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
    ];
    let score: i32 = _input.plays.iter()
        .filter_map(|(player1, player2)| {
            player1_mapping.get(player1).and_then(|&player1| {
                outcome_mapping.get(player2).map(|&outcome| {
                    let player2 = play_for_outcome(player1, outcome);
                    score(player1, player2)
                })
            })
        })
        .sum();
    Ok(score.to_string())
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
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    pub fn input_data() -> InputModel {
        InputModel {
            plays: vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')],
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
        let expected = "15";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "12";

        assert_eq!(actual, expected);
    }
}
