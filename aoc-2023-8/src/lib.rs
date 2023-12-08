use std::{collections::HashMap, str::FromStr};
use winnow::{
    ascii::{alphanumeric1, line_ending},
    combinator::{alt, delimited, repeat, terminated},
    Parser,
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub directions: Vec<Direction>,
    pub nodes: HashMap<String, (String, String)>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_directions = repeat(
            0..,
            alt((
                "L".try_map(|_| Ok::<_, ()>(Direction::Left)),
                "R".try_map(|_| Ok::<_, ()>(Direction::Right)),
            )),
        );
        let parse_nodes = repeat(
            0..,
            (
                terminated(alphanumeric1::<_, ()>, " = "),
                delimited("(", alphanumeric1, ", "),
                terminated(alphanumeric1, ")"),
                line_ending,
            )
                .try_map(|(key, a, b, _): (&str, &str, &str, &str)| {
                    Ok::<_, ()>((key.to_string(), (a.to_string(), b.to_string())))
                }),
        )
        .try_map(|nodes: Vec<(String, (String, String))>| {
            Ok::<_, ()>(
                nodes
                    .into_iter()
                    .collect::<HashMap<String, (String, String)>>(),
            )
        });

        let mut parse_input = (parse_directions, line_ending, line_ending, parse_nodes)
            .map(|(directions, _, _, nodes)| InputModel { directions, nodes });

        parse_input.parse(s).map_err(|_| AocError::ParseError)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_parse() {
        let model: InputModel = TEST_INPUT.parse().unwrap();

        let directions = vec![Direction::Right, Direction::Left];
        let mut nodes = HashMap::new();
        nodes.insert("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()));
        nodes.insert("BBB".to_string(), ("DDD".to_string(), "EEE".to_string()));
        nodes.insert("CCC".to_string(), ("ZZZ".to_string(), "GGG".to_string()));
        nodes.insert("DDD".to_string(), ("DDD".to_string(), "DDD".to_string()));
        nodes.insert("EEE".to_string(), ("EEE".to_string(), "EEE".to_string()));
        nodes.insert("GGG".to_string(), ("GGG".to_string(), "GGG".to_string()));
        nodes.insert("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string()));
        assert_eq!(model, InputModel { directions, nodes });
    }
}
