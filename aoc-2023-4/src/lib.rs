use nom::{
    bytes::complete::tag, character::complete as cc, combinator::map_res, multi::{separated_list1, many1},
    sequence::tuple
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub cards: Vec<ScratchCard>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_card = map_res(
            tuple((
                tuple((tag("Card"), cc::multispace1)),
                cc::u32::<_,()>,
                tuple((tag(":"), cc::multispace1)),
                separated_list1(cc::multispace1, cc::u32),
                tuple((cc::multispace1, tag("|"), cc::multispace1)),
                separated_list1(cc::multispace1, cc::u32),
                cc::multispace0,
            )),
            |(_, id, _, winning, _, numbers, _)| {
                Ok::<_,()>(ScratchCard {
                    id,
                    winning,
                    numbers,
                })
            },
        );
        let (s, cards) =
            many1(parse_card)(s).map_err(|_| AocError::ParseError)?;
        println!("remaining {:?}", s);
        Ok(InputModel { cards })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScratchCard {
    pub id: u32,
    pub winning: Vec<u32>,
    pub numbers: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_parse() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        println!("{:?}", input.cards);
        assert_eq!(input.cards.len(), 6);
        assert_eq!(input.cards[0].id, 1);
        assert_eq!(input.cards[0].winning, vec![41, 48, 83, 86, 17]);
        assert_eq!(input.cards[0].numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }
}
