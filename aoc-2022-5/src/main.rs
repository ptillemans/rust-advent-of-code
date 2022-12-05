use aoc_2022_5::{AocError, InputModel, Crate};

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    let mut stacks = input.stacks.clone();
    let moves = input.moves.clone();
    moves.into_iter()
        .map(|m| stacks.apply_move_9000(m))
        .collect::<Result<Vec<_>,_>>()?;
    
    stacks.heads().iter()
       .map(|c| c.map(Crate::to_char).ok_or(AocError::NoSolution))
       .collect()
}
    

fn part2(input: &InputModel) -> Result<String, AocError> {
    let mut stacks = input.stacks.clone();
    let moves = input.moves.clone();
    moves.into_iter()
        .map(|m| stacks.apply_move_9001(m))
        .collect::<Result<Vec<_>,_>>()?;
    stacks.heads().iter()
       .map(|c| c.map(Crate::to_char).ok_or(AocError::NoSolution))
       .collect()
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
    use aoc_2022_5::{Moves, Stacks, Move, Stack};

    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    pub fn input_data() -> InputModel {
        InputModel {
            stacks: Stacks::new(vec![
                Stack::from_chars(vec!['Z', 'N']),
                Stack::from_chars(vec!['M', 'C', 'D']),
                Stack::from_chars(vec!['P']),
            ]),
            moves: Moves::new(vec![
                Move{ from: 2, to: 1, amount: 1 },
                Move{ from: 1, to: 3, amount: 3 },
                Move{ from: 2, to: 1, amount: 2 },
                Move{ from: 1, to: 2, amount: 1 },
            ]),
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
        let expected = "CMZ".to_string();
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "MCD";

        assert_eq!(actual, expected);
    }
}
