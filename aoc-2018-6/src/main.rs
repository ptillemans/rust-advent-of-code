use aoc_2018_6::{AocError, InputModel};

const INPUT: &str = include_str!("../data/input.txt");


fn part1(_input: &InputModel) -> Result<String,AocError> {
    return Ok("Not implemented".to_string())
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    return Ok("Not implemented".to_string())
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
    
    const TEST_INPUT: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    pub fn input_data() -> InputModel {
        InputModel {
          points: vec![
            (1, 1),
            (1, 6),
            (8, 3),
            (3, 4),
            (5, 5),
            (8, 9),
          ]
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
        let expected = "";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }
}
