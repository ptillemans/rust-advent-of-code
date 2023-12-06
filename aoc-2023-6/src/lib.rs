use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub races: Vec<(u64, u64)>,
    pub part2_time: u64,
    pub part2_distance: u64,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_lines: Vec<(String, Vec<u64>, u64)> =
            s.lines().map(|line| parse_line(line)).collect();
        let times = parsed_lines[0].1.clone();
        let distances = parsed_lines[1].1.clone();
        let races = times
            .into_iter()
            .zip(distances.into_iter())
            .collect();
        let part2_time = parsed_lines[0].2;
        let part2_distance = parsed_lines[1].2;
        Ok(InputModel { races , part2_time, part2_distance})
    }
}

fn parse_line(line: &str) -> (String, Vec<u64>, u64) {
    let mut parts = line.split(":");
    let name = parts.next().unwrap().trim().to_string();
    let vals = parts
        .next()
        .unwrap();
    let values = vals.split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect();
    let squashed = vals.replace(" ", "").parse::<u64>().unwrap();

    (name, values, squashed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Time:      7  15   30";
        let (name, values, squashed) = parse_line(line);
        assert_eq!(name, "Time");
        assert_eq!(values, vec![7, 15, 30]);
        assert_eq!(squashed, 71530);
    }
}
