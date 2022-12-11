use std::str::FromStr;
use nom::{
    IResult, Parser, 
    character::complete::*, 
    branch::*,
    bytes::complete::*, 
    combinator::*, 
    multi::*, 
    sequence::*,
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub monkeys: Vec<Monkey>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = separated_list1(pair(newline, newline), Monkey::parser)(s)
            .map_err(|_| AocError::ParseError);
        println!("{:?}", result);
        let (remaining, monkeys) = result?;
        println!("Remaining: {}", remaining);

        if remaining.len() > 0 {
            return Err(AocError::ParseError);
        }
        Ok(InputModel { monkeys })  
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Item(pub u32);

impl Item {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(digit1, |s: &str| Item(s.parse().unwrap()))(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Old,
    Literal(u32),
}

impl Operand {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("old"), |_| Operand::Old),
            map(digit1, |s: &str| Operand::Literal(s.parse().unwrap())),
        ))(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum  Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

impl Operation {
    fn parser(input: &str) -> IResult<&str, Self> {
        preceded(
            tag("new = "),
            alt((
                tuple((Operand::parser, tag(" + "), Operand::parser)).map(|(a, _, b)| Operation::Add(a, b)),
                tuple((Operand::parser, tag(" * "), Operand::parser)).map(|(a, _, b)| Operation::Multiply(a, b)),
            ))
        ).parse(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Test {
    DivisibleBy(u32),
}

impl Test {
    fn parser(input: &str) -> IResult<&str, Self> {
        preceded(
            tag("divisible by "),
            map(digit1, |s: &str| Test::DivisibleBy(s.parse().unwrap()))
        ).parse(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MonkeyId(pub usize);

impl MonkeyId {
    pub fn new(id: usize) -> Self {
        MonkeyId(id)
    }

    pub fn parser(input: &str) -> nom::IResult<&str, Self> {
        map(digit1, |s: &str| MonkeyId(s.parse().unwrap()))(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Monkey {
    pub id: MonkeyId,
    pub starting_items: Vec<Item>,
    pub operation: Operation,
    pub test: Test,
    pub if_true: MonkeyId,
    pub if_false: MonkeyId,
}

impl Monkey {
    pub(crate) fn parser(input: &str) -> nom::IResult<&str, Self> {
        tuple((
            delimited(tag("Monkey "), MonkeyId::parser, tag(":\n")),
            delimited(tag("  Starting items: "), separated_list0(tag(", "), Item::parser), tag("\n")),
            delimited(tag("  Operation: "), Operation::parser, tag("\n")),
            delimited(tag("  Test: "), Test::parser, tag("\n")),
            delimited(tag("    If true: throw to monkey "), MonkeyId::parser, tag("\n")),
            preceded(tag("    If false: throw to monkey "), MonkeyId::parser)
        ))
        .map(|(id, starting_items, operation, test, if_true, if_false)| 
             Monkey{id, starting_items, operation, test, if_true, if_false})
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        let expected = Monkey {
            id: MonkeyId(0),
            starting_items: vec![Item(79), Item(98)],
            operation: Operation::Multiply(Operand::Old, Operand::Literal(19)),
            test: Test::DivisibleBy(23),
            if_true: MonkeyId(2),
            if_false: MonkeyId(3),
        };
        let (rest, actual) = Monkey::parser(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(actual, expected);
    }
}
