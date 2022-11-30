use std::str::FromStr;

// solve day 6 of AdventOfCode
//

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub points: Vec<(i32, i32)>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

// parse a point from a line
fn parse_line(s: &str) -> Result<(i32, i32), AocError> {
    let mut parts = s.split(", ");
    let x = parts.next().ok_or(AocError::ParseError)?;
    let y = parts.next().ok_or(AocError::ParseError)?;
    let x = x.parse::<i32>().map_err(|_| AocError::ParseError)?;
    let y = y.parse::<i32>().map_err(|_| AocError::ParseError)?;
    Ok((x, y))
}



impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(parse_line)
            .collect::<Result<Vec<(i32, i32)>, AocError>>()
            .map(|points| InputModel { points })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
