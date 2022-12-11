#![feature(test)]
use aoc_2022_11::*;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    return Ok("Not implemented".to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
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
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    pub fn input_data() -> InputModel {
        InputModel {
            monkeys:  vec![
                Monkey {
                    id: MonkeyId(0),
                    starting_items: vec![Item(79), Item(98)],
                    operation: Operation::Multiply(Operand::Old, Operand::Literal(19)),
                    test: Test::DivisibleBy(23),
                    if_true: MonkeyId(2),
                    if_false: MonkeyId(3),
                },
                Monkey {
                    id: MonkeyId(1),
                    starting_items: vec![Item(54), Item(65), Item(75), Item(74)],
                    operation: Operation::Add(Operand::Old, Operand::Literal(6)),
                    test: Test::DivisibleBy(19),
                    if_true: MonkeyId(2),
                    if_false: MonkeyId(0),
                },
                Monkey {
                    id: MonkeyId(2),
                    starting_items: vec![Item(79), Item(60), Item(97)],
                    operation: Operation::Multiply(Operand::Old, Operand::Old),
                    test: Test::DivisibleBy(13),
                    if_true: MonkeyId(1),
                    if_false: MonkeyId(3),
                },
                Monkey {
                    id: MonkeyId(3),
                    starting_items: vec![Item(74)],
                    operation: Operation::Add(Operand::Old, Operand::Literal(3)),
                    test: Test::DivisibleBy(17),
                    if_true: MonkeyId(0),
                    if_false: MonkeyId(1),
                },
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
        let expected = "10605";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

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
