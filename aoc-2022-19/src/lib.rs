use std::{str::FromStr, ops::{Add, Sub, Index, IndexMut}, collections::HashMap};
use nom::{
    IResult, Parser, 
    sequence::{separated_pair, tuple},
    multi::separated_list1,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
};


#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub blueprints: Vec<BluePrint>,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    const ALL: [Resource; 4] = [Resource::Ore, Resource::Clay, Resource::Obsidian, Resource::Geode];
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Inventory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
   
    fn add_resource(&mut self, index: Resource, value: u32) {
        match index {
            Resource::Ore => self.ore += value,
            Resource::Clay => self.clay += value,
            Resource::Obsidian => self.obsidian += value,
            Resource::Geode => self.geode += value,
        }
    }
    
    fn remove_resource(&mut self, index: Resource, value: u32) {
        match index {
            Resource::Ore => self.ore -= value,
            Resource::Clay => self.clay -= value,
            Resource::Obsidian => self.obsidian -= value,
            Resource::Geode => self.geode -= value,
        }
    }
}

impl Index<Resource> for Inventory {
    type Output = u32;

    fn index(&self, index: Resource) -> &Self::Output {
        match index {
            Resource::Ore => &self.ore,
            Resource::Clay => &self.clay,
            Resource::Obsidian => &self.obsidian,
            Resource::Geode => &self.geode,
        }
    }
}

impl IndexMut<Resource> for Inventory {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        match index {
            Resource::Ore => &mut self.ore,
            Resource::Clay => &mut self.clay,
            Resource::Obsidian => &mut self.obsidian,
            Resource::Geode => &mut self.geode,
        }
    }
}

impl Add for Inventory {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for resource in Resource::ALL.iter() {
            result[*resource] += rhs[*resource];
        }
        result
    }
}

impl Sub for Inventory {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for resource in Resource::ALL.iter() {
            result.remove_resource(*resource, rhs[*resource]);
        }
        result
    }

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
pub struct Ingredient {
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
pub struct BluePrint {
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


    pub fn best_geode_production(&self, period: usize) -> u32 {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct State {
            period: usize,
            robots: Inventory,
            resources: Inventory,
        }
        let mut robots = Inventory::new();
        robots.add_resource(Resource::Ore, 1);
        let start = State {
            period: 0,
            robots,
            resources: Inventory::new(),
        };

   
        let mut open = vec![start];
        let mut best = 0;

        while !open.is_empty() {
            let state = open.pop().unwrap();
            if state.period == period {
                let result = state.resources[Resource::Geode];
                if result > best {
                    best = result;
                }
                continue
            }

            // check if we can build some robots
            self.recipes.iter()
                .filter(|(_, ingredients)| ingredients.iter().all(|ingredient| 
                    state.resources[ingredient.resource] >= ingredient.quantity))
                .for_each(|(robot, ingredients)| {
                    let mut new_state = state.clone();
                    new_state.period += 1;
                    new_state.robots[*robot] += 1;
                    for ingredient in ingredients.iter() {
                        new_state.resources[ingredient.resource] -= ingredient.quantity;
                    }
                    open.push(new_state);
                });
        }

        best
    }

    pub fn quality(&self) -> u32 {
        self.max_geodes() * self.id 
    }

    fn max_geodes(&self) -> u32 {
        self.best_geode_production(24)
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

pub const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

pub fn test_input() -> InputModel {
    TEST_INPUT.parse().unwrap()
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

    #[test]
    fn test_parse_inputmodel() {
        let actual = test_input().blueprints.len();
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_quality() {
        let actual = test_input().blueprints[0].quality();
        assert_eq!(actual, 9);
        let actual = test_input().blueprints[1].quality();
        assert_eq!(actual, 24);
    }

}
