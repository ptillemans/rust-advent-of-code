use std::str::FromStr;
use nom::{
    IResult, Parser,
    branch::*,
    character::complete::*,
    combinator::*,
    multi::*,
    sequence::*,
};


#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(i64),
}

impl Packet {

    // create a nom parser for a Packet
    fn parser(input: &str) -> nom::IResult<&str, Packet> {
        alt((
            delimited(
                char('['),
                separated_list0(char(','), Packet::parser),
                char(']')
            ).map(|v| Packet::List(v)),
            map(
                digit1,
                |s: &str| Packet::Int(s.parse::<i64>().unwrap())
            )
        ))(input)
    }
}


#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pairs: Vec<(Packet, Packet)>,
}


impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, pairs) = separated_list1(
            pair(newline, newline),
            separated_pair(
                Packet::parser,
                newline,
                Packet::parser),
        )(s).map_err(|_| AocError::ParseError)?;
        Ok(InputModel { pairs })
    }
}


pub const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

pub fn test_input() -> InputModel { 
    let pairs = vec![
        (
            Packet::List(vec![Packet::Int(1),Packet::Int(1),Packet::Int(3),Packet::Int(1),Packet::Int(1)]),
            Packet::List(vec![Packet::Int(1),Packet::Int(1),Packet::Int(5),Packet::Int(1),Packet::Int(1)]),
        ),
        (
            Packet::List(vec![Packet::List(vec![Packet::Int(1)]),Packet::List(vec![Packet::Int(2),Packet::Int(3),Packet::Int(4)])]),
            Packet::List(vec![Packet::List(vec![Packet::Int(1)]),Packet::Int(4)])
        ),
        (
            Packet::List(vec![Packet::Int(9)]),
            Packet::List(vec![Packet::List(vec![Packet::Int(8),Packet::Int(7),Packet::Int(6)])]),
        ),
        (
            Packet::List(vec![Packet::List(vec![Packet::Int(4),Packet::Int(4)]),Packet::Int(4),Packet::Int(4)]),
            Packet::List(vec![Packet::List(vec![Packet::Int(4),Packet::Int(4)]),Packet::Int(4),Packet::Int(4),Packet::Int(4)])
        ),
        (
            Packet::List(vec![Packet::Int(7),Packet::Int(7),Packet::Int(7),Packet::Int(7)]),
            Packet::List(vec![Packet::Int(7),Packet::Int(7),Packet::Int(7)])
        ),
        (
            Packet::List(vec![]),
            Packet::List(vec![Packet::Int(3)])
        ),
        (
            Packet::List(vec![Packet::List(vec![Packet::List(vec![])])]),
            Packet::List(vec![Packet::List(vec![])])
        ),
        (
            Packet::List(vec![Packet::Int(1),Packet::List(vec![Packet::Int(2),Packet::List(vec![Packet::Int(3),Packet::List(vec![Packet::Int(4),Packet::List(vec![Packet::Int(5),Packet::Int(6),Packet::Int(7)])])])]),Packet::Int(8),Packet::Int(9)]),
            Packet::List(vec![Packet::Int(1),Packet::List(vec![Packet::Int(2),Packet::List(vec![Packet::Int(3),Packet::List(vec![Packet::Int(4),Packet::List(vec![Packet::Int(5),Packet::Int(6),Packet::Int(0)])])])]),Packet::Int(8),Packet::Int(9)])
        ),
    ];
    InputModel {
        pairs,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_packet() {
        let input = "[1,1,3,1,1]";
        let result = Packet::parser(input);
        assert_eq!(result, Ok(("", Packet::List(vec![Packet::Int(1),Packet::Int(1),Packet::Int(3),Packet::Int(1),Packet::Int(1)]))));
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = test_input();

        assert_eq!(actual, expected);
    }



}
