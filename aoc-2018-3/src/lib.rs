use std::str::FromStr;


#[derive(Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub id: u32,
    pub position: (usize, usize),
    pub size: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub rectangles: Vec<Rectangle>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution found")]
    NoSolution,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.lines()
            .map(|s| parsers::parse_rectangle(s)
                .map(|p| p.1)
                .map_err(|_| AocError::ParseError))
            .collect::<Result<Vec<Rectangle>, AocError>>()
            .map(|rectangles| InputModel{rectangles})
    }
}

mod parsers {
    use super::Rectangle;
    use nom::IResult;
    use nom::bytes::complete::{is_a, tag};

    fn parse_id(s: &str) -> IResult<&str, u32> {
        let (rest, _): (&str, char) = nom::character::complete::char('#')(s)?;
        let (rest, id) = is_a("0123456789")(rest)?;
        let (rest, _) = is_a(" \t")(rest)?;
        let (rest, _) = tag("@")(rest)?;
        let (rest, _) = is_a(" \t")(rest)?;
        Ok((rest, id.parse::<u32>().unwrap()))
    }

    pub fn parse_position(s: &str) -> IResult<&str, (usize, usize)> {
        let (rest, x) = is_a("0123456789")(s)?;
        let (rest, _) = tag(",")(rest)?;
        let (rest, y) = is_a("0123456789")(rest)?;
        let (rest, _) = tag(":")(rest)?;
        let (rest, _) = is_a(" \t")(rest)?;
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        Ok((rest, (x, y)))
    }
    
    pub fn parse_size(s: &str) -> IResult<&str, (usize, usize)> {
        let (rest, w) = is_a("0123456789")(s)?;
        let (rest, _) = tag("x")(rest)?;
        let (rest, h) = is_a("0123456789")(rest)?;
        let w = w.parse::<usize>().unwrap();
        let h = h.parse::<usize>().unwrap();
        Ok((rest, (w, h)))
    }
   
    pub fn parse_rectangle(s: &str) -> IResult<&str, Rectangle> {
        let (s, id) = parse_id(s)?;
        let (s, position) = parse_position(s)?;
        let (s, size) = parse_size(s)?;
        Ok((s, Rectangle{id, position, size}))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const TEST_LINE: &str = "#1 @ 1,3: 4x4";
    
        #[test]
        fn test_parse_id() {
            let (rest, s_id) = parse_id(TEST_LINE).unwrap();
            assert_eq!(s_id, 1);
            assert_eq!(rest, "1,3: 4x4");
        }

        #[test]
        fn test_parse_position() {
            let (rest, (x, y)) = parse_position("1,3: 4x4").unwrap();
            assert_eq!(x, 1);
            assert_eq!(y, 3);
            assert_eq!(rest, "4x4");
        }

        #[test]
        fn test_parse_size() {
            let (rest, (x, y)) = parse_size("4x4").unwrap();
            assert_eq!(x, 4);
            assert_eq!(y, 4);
            assert_eq!(rest, "");
        }


        #[test]
        fn test_parse_rectangle() {
            let (rest, actual) = parse_rectangle(TEST_LINE).unwrap();
            let expected = Rectangle {
                id: 1,
                position: (1, 3),
                size: (4, 4)
            };
            assert_eq!(actual, expected);
            assert_eq!(rest, "");
        }
    }
}

