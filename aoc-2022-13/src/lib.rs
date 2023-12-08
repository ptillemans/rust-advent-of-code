use std::str::FromStr;
use nom::{
    IResult, Parser,
    branch::*,
    character::complete::*,
    combinator::*,
    multi::*,
    sequence::*,
};


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Int(i64),
}

impl PartialOrd for Packet {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::Int(a), Packet::List(b)) => vec![Packet::Int(*a)].cmp(b),
            (Packet::List(a), Packet::Int(b)) => a.cmp(&vec![Packet::Int(*b)]),
        }
    }
}

impl Packet {

    // create a nom parser for a Packet
    fn parser(input: &str) -> IResult<&str, Packet> {
        alt((
            delimited(
                char('['),
                separated_list0(char(','), Packet::parser),
                char(']')
            ).map(Packet::List),
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
    pub pairs: Vec<(Packet, Packet)>,
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

fn make_divider(x: i64) -> Packet {
    // return packet [[x]]
    Packet::List(vec![Packet::List(vec![Packet::Int(x)])])
}

fn dividers() -> Vec<Packet> {
    vec![make_divider(2), make_divider(6)]
}

fn sorted_with_dividers(input: &InputModel) -> Vec<Packet> {
    let mut packets: Vec<Packet> = input.pairs.iter()
        .flat_map(|(a, b)| vec![a, b].into_iter())
        .chain(dividers().iter())
        .cloned()
        .collect();
    packets.sort();
    packets
}

pub fn decoder_key(input: &InputModel) -> i64 {
    let packets = sorted_with_dividers(input);
    dividers().iter()
        .filter_map(|divider| {
            packets.iter().position(|packet| packet == divider)
        })
        .map(|x| (x + 1) as i64)
        .product()
}

#[cfg(test)]
mod tests {

    use super::*;

    const PART2_DATA: &str = "[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[[2]]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[[6]]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]";

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

    #[test]
    fn test_order() {
        let input = test_input();
        let expected = vec![true, true, false, true, false, true, false, false];
        let actual: Vec<bool> = input.pairs.iter().map(|(a, b)| a < b).collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_with_dividers() {
        let input = test_input();
        let expected = separated_list1(
            newline,
            Packet::parser)(PART2_DATA).unwrap().1;
        let actual = sorted_with_dividers(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decoder_key() {
        let input = test_input();
        let expected = 140;
        let actual = decoder_key(&input);

        assert_eq!(actual, expected);
    }

}
