use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub pipes: Vec<Pipe>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pipe {
    pub x: i64,
    pub y: i64,
    pub section: Section,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Section {
    Start,
    Horizontal,
    Vertical,
    CornerNE,
    CornerNW,
    CornerSE,
    CornerSW,
    Ground,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl FromStr for Section {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Section::Start),
            "-" => Ok(Section::Horizontal),
            "|" => Ok(Section::Vertical),
            "L" => Ok(Section::CornerNE),
            "J" => Ok(Section::CornerNW),
            "F" => Ok(Section::CornerSE),
            "7" => Ok(Section::CornerSW),
            "." => Ok(Section::Ground),
            _ => Err(AocError::ParseError("Invalid section".to_string())),
        }
    }
}


#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

}
