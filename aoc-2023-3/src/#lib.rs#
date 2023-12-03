use std::str::FromStr;
use nom::{
    IResult,
    character::complete::{digit1, one_of},
    combinator::map_res, sequence, Err,
};
use nom_locate::{LocatedSpan, position};


type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Eq)]
pub enum  TokenValue {
    Number(u32),   // there are no numbers starting with 0 in the input
    Symbol(char),
}
    
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    location: (u32, u32),
    value: TokenValue
}


#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    tokens: Vec<Token>
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

fn location(s: Span) -> (u32, u32) {
    (s.location_line(), s.get_column() as u32)
}

fn parse_number(s: Span) -> IResult<Span,Token> {
    map_res(
        digit1,
        |s: Span| s.fragment().parse::<u32>().map(|n| Token {
            location: location(s),
            value: TokenValue::Number(n)
        })
    )(s)
}

fn parse_symbol(s: Span) -> IResult<Span, Token> { 
    map_res(
        one_of("+-*/@$"),
    |c: char| Ok(Token {
            location: location(s),
            value: TokenValue::Symbol(c),
        })
    )(s)
}

fn parse(s: Span) -> IResult<Span, Vec<Token>> {
    let (s, tokens) = nom::multi::separated_list1(
       one_of(". \n"),
        nom::branch::alt((parse_number, parse_symbol))
    )(s)?;
    Ok((s, tokens))
}



#[cfg(test)]
mod tests {

    

}
