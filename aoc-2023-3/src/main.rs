#![feature(test)]
use aoc_2023_3::{AocError, InputModel, TokenValue, Token};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let symbols = input.tokens.iter()
        .filter(|t| if let TokenValue::Symbol(_) = t.value { true } else { false })
        .collect::<Vec<_>>();
    let numbers = input.tokens.iter()
        .filter(|t| if let TokenValue::Number(_) = t.value { true } else { false })
        .collect::<Vec<_>>();

    let part_numbers = numbers.iter()
        .filter(|n| symbols.iter().any(|s| symbols_touching(n, s)))
        .collect::<Vec<_>>();

    let sum = part_numbers.iter()
        .map(|n| if let TokenValue::Number(n) = n.value { n } else { panic!("Not a number") })
        .sum::<u32>();

    return Ok(sum.to_string());
    
}

fn symbols_touching(a: &Token, b: &Token) -> bool {
    let (a_line, a_col, a_len) = a.location;
    let (b_line, b_col, _b_len) = b.location;
    let (min_line, max_line, min_col, max_col) = (a_line - 1, a_line+1, a_col - 1, a_col + a_len);

    return b_line >= min_line && b_line <= max_line && b_col >= min_col && b_col <= max_col;
}



fn part2(input: &InputModel) -> Result<String, AocError> {
    let symbols = input.tokens.iter()
        .filter(|t| if let TokenValue::Symbol(_) = t.value { true } else { false })
        .collect::<Vec<_>>(); 
    let numbers = input.tokens.iter()
        .filter(|t| if let TokenValue::Number(_) = t.value { true } else { false })
        .collect::<Vec<_>>();

    let gears: u32 = symbols.iter()
        .filter(|t| t.value == TokenValue::Symbol('*'))
        .map(|gear| numbers.iter()
             .filter(|n| symbols_touching(n, gear))
             .filter_map(|t| if let TokenValue::Number(n) = t.value {
                 Some(n)
             } else {
                 None
             })
             .collect::<Vec<u32>>())
        .filter(|ns| ns.len() == 2)
        .map(|numbers| numbers[0]*numbers[1])
        .sum();

    return Ok(gears.to_string());
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
    use aoc_2023_3::{
        Token,
        TokenValue::{Number, Symbol},
    };
    use test::Bencher;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    pub fn input_data() -> InputModel {
        InputModel {
            tokens: vec![
                Token {
                    location: (1, 1, 3),
                    value: Number(467),
                },
                Token {
                    location: (1, 6, 3),
                    value: Number(114),
                },
                Token {
                    location: (2, 4, 1),
                    value: Symbol('*'),
                },
                Token {
                    location: (3, 3, 2),
                    value: Number(35),
                },
                Token {
                    location: (3, 7, 3),
                    value: Number(633),
                },
                Token {
                    location: (4, 7, 1),
                    value: Symbol('#'),
                },
                Token {
                    location: (5, 1, 3),
                    value: Number(617),
                },
                Token {
                    location: (5, 4, 1),
                    value: Symbol('*'),
                },
                Token {
                    location: (6, 6, 1),
                    value: Symbol('+'),
                },
                Token {
                    location: (6, 8, 2),
                    value: Number(58),
                },
                Token {
                    location: (7, 3, 3),
                    value: Number(592),
                },
                Token {
                    location: (8, 7, 3),
                    value: Number(755),
                },
                Token {
                    location: (9, 4, 1),
                    value: Symbol('$'),
                },
                Token {
                    location: (9, 6, 1),
                    value: Symbol('*'),
                },
                Token {
                    location: (10, 2, 3),
                    value: Number(664),
                },
                Token {
                    location: (10, 6, 3),
                    value: Number(598),
                },
            ],
        }
    }


    const TEST_INPUT2: &str = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";
    
    const TEST_INPUT3: &str = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "4361";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_2() {
        let input_data = TEST_INPUT2.parse::<InputModel>().unwrap();
        let actual = part1(&input_data).unwrap();
        let expected = "413";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_3() {
        let input_data = TEST_INPUT3.parse::<InputModel>().unwrap();
        let actual = part1(&input_data).unwrap();
        let expected = "925";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "467835";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_symbols_touching() {
        let a = Token {
            location: (1, 1, 3),
            value: Number(467),
        };
        let b = Token {
            location: (1, 6, 3),
            value: Number(114),
        };
        let c = Token {
            location: (2, 4, 1),
            value: Symbol('*'),
        };

        let d = Token {
            location: (110, 138, 3),
            value: Number(260),
        };
        let e  = Token {
            location: (111, 137, 1),
            value: Symbol('-'),
        };
        assert!(!symbols_touching(&a, &b));
        assert!(symbols_touching(&a, &c));
        assert!(symbols_touching(&d, &e));
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
