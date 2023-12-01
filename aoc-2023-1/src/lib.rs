use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub lines: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(|l| l.to_string()).collect();
        Ok(InputModel { lines })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let input = "line1\nline2\nline3";
        let model: InputModel = input.parse().unwrap();
        assert_eq!(model.lines, vec!["line1", "line2", "line3"]);
    }

}
