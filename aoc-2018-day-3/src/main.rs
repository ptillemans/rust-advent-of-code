mod parsers;

use aoc_2018_3::Rectangle;
use aoc_2018_3::{AocError, InputModel};

const INPUT: &str = include_str!("../data/input.txt");

type Fabric = [[i16; 1000]; 1000];

fn fabric_used(input: &InputModel) -> Fabric {
    let mut fabric: Fabric = [[0; 1000]; 1000];
    for rect in &input.rectangles {
        let (x0, y0) = rect.position;
        let (w, h) = rect.size;
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                fabric[x][y] += 1
            }
        }
    }
    fabric
}

fn part1(input: &InputModel) -> Result<String, AocError> {
    let fabric = fabric_used(input);
    let overlap: usize = (0..1000)
        .map(|x| fabric[x].into_iter().filter(|&c| c > 1).count())
        .sum();
    return Ok(overlap.to_string());
}

fn is_non_overlapping(fabric: &Fabric, rect: &Rectangle) -> bool {
    let (x0, y0) = rect.position;
    let (w, h) = rect.size;
    (y0..y0 + h)
        .flat_map(move |y| (x0..x0 + w).map(move |x| fabric[x][y]))
        .all(|n| n == 1)
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let fabric = fabric_used(input);
    input
        .rectangles
        .iter()
        .filter(|rect| is_non_overlapping(&fabric, rect))
        .next()
        .map(|rect| rect.id.to_string())
        .ok_or(AocError::NoSolution)
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
    use aoc_2018_3::Rectangle;

    use super::*;

    const TEST_INPUT: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    fn input_data() -> InputModel {
        InputModel {
            rectangles: vec![
                Rectangle {
                    id: 1,
                    position: (1, 3),
                    size: (4, 4),
                },
                Rectangle {
                    id: 2,
                    position: (3, 1),
                    size: (4, 4),
                },
                Rectangle {
                    id: 3,
                    position: (5, 5),
                    size: (2, 2),
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
        let expected = "4";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "3";

        assert_eq!(actual, expected);
    }
}
