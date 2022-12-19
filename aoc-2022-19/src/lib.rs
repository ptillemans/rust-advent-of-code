use std::str::FromStr;
use std::collections::HashMap;
use nom::{
    IResult, Parser, 
    sequence::{separated_pair, tuple},
    multi::separated_list1,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
};


#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    blueprints: Vec<BluePrint>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        separated_list1(line_ending, BluePrint::parser)
            .parse(s)
            .map(|(_, blueprints)| InputModel { blueprints })
            .map_err(|_| AocError::ParseError)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Resource {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, AocError> {
        match s {
            "ore" => Ok(Resource::Ore),
            "clay" => Ok(Resource::Clay),
            "obsidian" => Ok(Resource::Obsidian),
            "geode" => Ok(Resource::Geode),
            _ => Err(AocError::ParseError),
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Ingredient {
    resource: Resource,
    quantity: u32,
}

impl Ingredient {
    fn new(resource: Resource, quantity: u32) -> Self {
        Self { resource, quantity }
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        separated_pair(
            digit1.map(|s: &str| s.parse::<u32>().unwrap()), 
            tag(" "), 
            alpha1.map(|s: &str| s.parse::<Resource>().unwrap())
        ).map(|(quantity, resource)| {
            Self::new(resource, quantity)
        }).parse(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BluePrint {
    pub id: u32,
    pub recipes: HashMap<Resource, Vec<Ingredient>>,
}

impl BluePrint {
    pub fn new(id: u32 , recipes: HashMap<Resource, Vec<Ingredient>>) -> Self {
        Self { id, recipes }
    }

    pub fn parser(input: &str) -> IResult<&str, Self> {
        tuple((
            tag("Blueprint "),
            digit1.map(|s: &str| s.parse::<u32>().unwrap()),
            tag(": "),
            separated_list1(
                tag(" "), 
                recipe_parser
            ).map(|v| v.into_iter().collect::<HashMap<_, _>>()),
        )).map(|(_, id, _, recipes )| { BluePrint::new(id, recipes) })
        .parse(input)
    } 
}


fn recipe_parser(input: &str) -> IResult<&str, (Resource, Vec<Ingredient>)> {
    tuple((
        tag("Each "),
        alpha1.map(|s: &str| s.parse::<Resource>().unwrap()),
        tag(" robot costs "),
        separated_list1(
            tag(" and "),
            Ingredient::parser
        ).map(|v| v.into_iter().collect()),
        tag("."),
    )).map(|(_, target, _, ingredients, _)| (target, ingredients))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_blueprint() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let (rest, actual) = super::BluePrint::parser(input).unwrap();
        let expected = BluePrint::new(1, vec![
            (Resource::Ore, vec![Ingredient::new(Resource::Ore, 4)]),
            (Resource::Clay, vec![Ingredient::new(Resource::Ore, 2)]),
            (Resource::Obsidian, vec![Ingredient::new(Resource::Ore, 3), Ingredient::new(Resource::Clay, 14)]),
            (Resource::Geode, vec![Ingredient::new(Resource::Ore, 2), Ingredient::new(Resource::Obsidian, 7)]),
        ].into_iter().collect());

        assert_eq!(rest, "");
        assert_eq!(actual, expected);
    }
}
