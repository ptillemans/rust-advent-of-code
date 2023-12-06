use nom::{
    bytes::complete::tag,
    character::complete::{self as cc, alpha1, newline},
    combinator::map_res,
    multi::{many0, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub seeds: Vec<u64>,
    pub mappings: HashMap<String, (String, Vec<MapRange>)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MapRange {
    pub from: u64,
    pub to: u64,
    pub length: u64,
}

impl MapRange {
    pub fn contains(&self, value: u64) -> bool {
        self.from <= value && value < self.from + self.length
    }

    pub fn transform(&self, value: u64) -> Option<u64> {
        if self.contains(value) {
            Some(self.to + value - self.from)
        } else {
            None
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub source: String,
    pub destination: String,
    pub triplets: Vec<MapRange>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        map_res(
            tuple((
                parse_seeds,
                newline,
                separated_list1(newline, parse_map_entry),
            )),
            |(seeds, _, mappings)| {
                Ok::<_, ()>(InputModel {
                    seeds,
                    mappings: convert_input(mappings),
                })
            },
        )(s)
        .map(|(rest, input)| {
            println!("remaining {:?}", rest);
            input
        })
        .map_err(|_| AocError::ParseError)
    }
}

fn convert_input(mappings: Vec<Map>) -> HashMap<String, (String, Vec<MapRange>)> {
    mappings
        .iter()
        .map(|mapping| {
            (
                mapping.source.clone(),
                (mapping.destination.clone(), mapping.triplets.clone()),
            )
        })
        .collect()
}

fn parse_seeds(s: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tuple((tag("seeds:"), cc::multispace1)),
        separated_list1(cc::multispace1, cc::u64),
        newline,
    )(s)
}

fn parse_mapping(s: &str) -> IResult<&str, (String, String)> {
    map_res(
        tuple((
            alpha1::<&str, _>,
            tag("-to-"),
            alpha1::<&str, _>,
            tag(" map:\n"),
        )),
        |(source, _, destination, _)| Ok::<_, ()>((source.to_string(), destination.to_string())),
    )(s)
}

fn parse_triplets(s: &str) -> IResult<&str, Vec<MapRange>> {
    many0(terminated(
        map_res(
            tuple((cc::u64, cc::multispace1, cc::u64, cc::multispace1, cc::u64)),
            |(to, _, from, _, length)| Ok::<_, ()>(MapRange { from, to, length }),
        ),
        newline,
    ))(s)
}

fn parse_map_entry(s: &str) -> IResult<&str, Map> {
    map_res(
        tuple((parse_mapping, parse_triplets)),
        |((source, destination), triplets)| {
            Ok::<_, ()>(Map {
                source,
                destination,
                triplets,
            })
        },
    )(s)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 24 55 13
foobar";
        let expected = ("foobar", vec![79, 24, 55, 13]);

        let actual = parse_seeds(input);
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn test_parse_mapping() {
        let input = "foo-to-bar map:\n";
        let expected = ("", ("foo".to_string(), "bar".to_string()));

        let actual = parse_mapping(input);
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn test_parse_triplets() {
        let input = "49 53 8
0 11 42
42 0 7
57 7 4
";
        let expected = (
            "",
            vec![
                MapRange {
                    to: 49,
                    from: 53,
                    length: 8,
                },
                MapRange {
                    to: 0,
                    from: 11,
                    length: 42,
                },
                MapRange {
                    to: 42,
                    from: 0,
                    length: 7,
                },
                MapRange {
                    to: 57,
                    from: 7,
                    length: 4,
                },
            ],
        );

        let actual = parse_triplets(input);
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn test_parse_map_entry() {
        let input = "seed-to-soil map:
50 98 2
52 50 48
";
        let expected = (
            "",
            Map {
                source: "seed".to_string(),
                destination: "soil".to_string(),
                triplets: vec![
                    MapRange {
                        to: 50,
                        from: 98,
                        length: 2,
                    },
                    MapRange {
                        to: 52,
                        from: 50,
                        length: 48,
                    },
                ],
            },
        );

        let actual = parse_map_entry(input);
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn test_maprange_contains() {
        let range = MapRange {
            from: 50,
            to: 98,
            length: 2,
        };

        assert!(range.contains(50));
        assert!(range.contains(51));
        assert!(!range.contains(52));
        assert!(!range.contains(49));
    }
}
