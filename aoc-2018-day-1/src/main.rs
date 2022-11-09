use std::collections::HashSet;


const INPUT: &str = include_str!("../data/input.txt");


#[derive(Debug)]
struct InputModel  {
    values: Vec<i32>
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution found")]
    NoSolutionFound,
}
        
impl TryFrom<String> for InputModel {
    type Error = AocError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let values:Vec<i32> = value.lines().into_iter()
            .map(|s| s.parse::<i32>().expect("Parse input error"))
            .collect();
        Ok(InputModel { values })
    }
}

fn part1(input: &InputModel) -> Result<String,AocError> {
    return Ok(input.values.iter().sum::<i32>().to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let mut sums: HashSet<i32> = HashSet::new();
    let mut sum = 0;
    for x in input.values.iter().cycle() {
        sum += x;
        if sums.contains(&sum) {
            return Ok(sum.to_string())
        }
        sums.insert(sum);
    }
    Err(AocError::NoSolutionFound)
}

fn main() -> Result<(), AocError> {
    let input:InputModel = InputModel::try_from(INPUT.to_string())?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}
