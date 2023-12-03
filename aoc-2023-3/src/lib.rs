use nom::{
    branch::alt,
    character::complete::{digit1, one_of},
    combinator::map_res,
    multi::many1,
    IResult,
};
use nom_locate::LocatedSpan;
use std::str::FromStr;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenValue {
    Number(u32), // there are no numbers starting with 0 in the input
    Symbol(char),
    WhiteSpace,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub location: (u32, u32, u32),
    pub value: TokenValue,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub tokens: Vec<Token>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = Span::new(s);
        let (_, tokens) = parse(s).map_err(|_| AocError::ParseError)?;
        Ok(InputModel { tokens })
    }
}

fn parse_number(s: Span) -> IResult<Span, Token> {
    map_res(digit1, |s: Span| {
        s.fragment().parse::<u32>().map(|n| Token {
            location: (s.location_line(), s.get_column() as u32, s.fragment().len() as u32),
            value: TokenValue::Number(n),
        })
    })(s)
}

fn parse_symbol(s: Span) -> IResult<Span, Token> {
    map_res(one_of("+-*/@$#%&="), |c: char| {
        Ok::<Token, ()>(Token {
            location: (s.location_line(), s.get_column() as u32, 1),
            value: TokenValue::Symbol(c),
        })
    })(s)
}

fn parse_whitespace(s: Span) -> IResult<Span, Token> {
    map_res(many1(one_of(". \n")), |_| {
        Ok::<Token, ()>(Token {
            location: (s.location_line(), s.get_column() as u32, s.fragment().len() as u32), 
            value: TokenValue::WhiteSpace,
        })
    })(s)
}

fn parse(s: Span) -> IResult<Span, Vec<Token>> {
    let (s, tokens) = many1(alt((parse_number, parse_symbol, parse_whitespace)))(s)?;
    let tokens = tokens
        .into_iter()
        .filter(|t| t.value != TokenValue::WhiteSpace)
        .collect();
    Ok((s, tokens))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_number() {
        let s = Span::new("123");
        let (s, token) = parse_number(s).unwrap();
        assert_eq!(
            token,
            Token {
                location: (1, 1, 3),
                value: TokenValue::Number(123)
            }
        );
        assert_eq!(*s.fragment(), "");
    }

    #[test]
    fn test_parse_symbol() {
        let s = Span::new("*");
        let (s, token) = parse_symbol(s).unwrap();
        assert_eq!(
            token,
            Token {
                location: (1, 1, 1),
                value: TokenValue::Symbol('*')
            }
        );
        assert_eq!(*s.fragment(), "");
    }

    #[test]
    fn test_parse() {
        let s = Span::new("123*456");
        let (s, tokens) = parse(s).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    location: (1, 1, 3),
                    value: TokenValue::Number(123)
                },
                Token {
                    location: (1, 4, 1),
                    value: TokenValue::Symbol('*')
                },
                Token {
                    location: (1, 5, 3),
                    value: TokenValue::Number(456)
                },
            ]
        );
        assert_eq!(*s.fragment(), "");
    }

    #[test]
    fn test_parse_multiline() {
        let s = Span::new("...123*.\n...456.\n");
        let (s, tokens) = parse(s).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    location: (1, 4, 3),
                    value: TokenValue::Number(123)
                },
                Token {
                    location: (1, 7, 1),
                    value: TokenValue::Symbol('*')
                },
                Token {
                    location: (2, 4, 3),
                    value: TokenValue::Number(456)
                },
            ]
        );
        assert_eq!(*s.fragment(), "");
    }

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

    #[test]
    fn test_input() {
        let input: InputModel = TEST_INPUT.parse().unwrap();
        assert_eq!(input.tokens.len(), 16);
    }
}
