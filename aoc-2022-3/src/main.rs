use aoc_2022_3::{AocError, InputModel, priority};

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    let rucksacks = &input.rucksacks;
    Ok(rucksacks.iter()
        .filter_map(|rucksack| rucksack.priority())
        .sum::<usize>()
        .to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let rucksacks = &input.rucksacks;
    Ok(rucksacks
        .chunks(3)
        .filter_map(|team| {
            team.iter()
                .map(|rucksack| rucksack.unique_items())
                .reduce(|acc, items| {
                    acc.intersection(&items).cloned().collect()
                })
            })
        .filter_map(|common_items| priority(common_items))
        .sum::<usize>()
        .to_string())
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
    use aoc_2022_3::TEST_INPUT;

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
        let expected = "157";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "70";

        assert_eq!(actual, expected);
    }
}
