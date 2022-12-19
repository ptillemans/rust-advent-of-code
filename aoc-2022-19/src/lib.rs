use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::{
    collections::HashMap,
    ops::{Add, Index, IndexMut, Sub},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
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
    const ALL: [Resource; 4] = [
        Resource::Ore,
        Resource::Clay,
        Resource::Obsidian,
        Resource::Geode,
    ];
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
pub struct Inventory {
    geode: u32,
    obsidian: u32,
    clay: u32,
    ore: u32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
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

    fn parser(input: &str) -> IResult<&str, Self> {
        separated_list1(
            tag(" and "),
            separated_pair(
                digit1.map(|s: &str| s.parse::<u32>().unwrap()),
                tag(" "),
                alpha1.map(|s: &str| s.parse::<Resource>().unwrap()),
            ),
        )
        .map(|resources| {
            let mut result = Inventory::new();
            resources.iter().for_each(|(quantity, resource)| {
                result[*resource] = *quantity;
            });
            result
        })
        .parse(input)
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

    fn has_all_resources(&self, other: &Inventory) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
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
        let mut result = self;
        for resource in Resource::ALL.iter() {
            result[*resource] += rhs[*resource];
        }
        result
    }
}

impl Sub for Inventory {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BluePrint {
    pub id: u32,
    pub recipes: HashMap<Resource, Inventory>,
}

impl BluePrint {
    pub fn new(id: u32, recipes: HashMap<Resource, Inventory>) -> Self {
        Self { id, recipes }
    }

    pub fn parser(input: &str) -> IResult<&str, Self> {
        tuple((
            tag("Blueprint "),
            digit1.map(|s: &str| s.parse::<u32>().unwrap()),
            tag(": "),
            separated_list1(tag(" "), recipe_parser)
                .map(|v| v.into_iter().collect::<HashMap<_, _>>()),
        ))
        .map(|(_, id, _, recipes)| BluePrint::new(id, recipes))
        .parse(input)
    }

    pub fn best_production(&self, period: usize) -> Factory {
        let mut start = Factory::new();
        start.robots.add_resource(Resource::Ore, 1);

        let mut open = Vec::new();
        open.push((0, start));
        let mut best = start;

        let mut counter = 0;
        while !open.is_empty() {
            counter += 1;
            let (time, state) = open.pop().unwrap();

            if time >= period {
                let result = state;
                if result.resources > best.resources {
                    best = result;
                }
                continue;
            }

            let time_remaining = (period - time) as u32;
            let max_geodes = state.resources.geode + min(1,state.resources.geode)*time_remaining*(time_remaining+1)/2;
            if max_geodes < best.resources.geode {
                continue;
            }

            // check if we can build some robots
            let mut skip = None;
            Resource::ALL
                .iter()
                .filter_map(|resource| {
                    state
                        .earliest_production(self.recipes.get(resource).unwrap())
                        .map(|t| (resource, t))
                })
                .for_each(|(resource, t)| {
                    let mut new_state = state;
                    for _ in 0..t {
                        new_state.resources = new_state.resources + state.robots;
                    }
                    new_state.resources =
                        new_state.resources - *self.recipes.get(resource).unwrap();
                    new_state.robots.add_resource(*resource, 1);
                    let new_time = time + t;
                    if new_time <= period {
                        open.push((new_time, new_state));
                    }
                    skip = skip.or(Some(t)).map(|x| min(x, t));
                });

            let mut new_state = state;
            let mut new_time = time;
            if let Some(t) = skip {
                let t = min(t, period - time);

                for _ in 0..t {
                    new_state = new_state.produce_nothing();
                    new_time += 1;
                }
            } else {
                new_state = new_state.produce_nothing();
                new_time += 1;
            }
            if time <= period {
                open.push((new_time, new_state));
            }
        }

        println!("Best result: {:?}", best);
        println!("Counter: {}", counter);
        best
    }

    pub fn quality(&self) -> u32 {
        self.max_geodes(24) * self.id
    }

    fn max_geodes(&self, rounds: usize) -> u32 {
        self.best_production(rounds).resources.geode
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
pub struct Factory {
    resources: Inventory,
    robots: Inventory,
}

impl Factory {
    fn new() -> Self {
        Self {
            robots: Inventory::new(),
            resources: Inventory::new(),
        }
    }

    fn produce_robot(&self, resource: &Resource, ingredients: &Inventory) -> Self {
        let mut result = *self;
        result.resources = result.resources + result.robots - *ingredients;
        result.robots[*resource] += 1;
        result
    }

    fn produce_nothing(&self) -> Self {
        let mut result = *self;
        result.resources = result.resources + result.robots;
        result
    }

    fn earliest_production(&self, ingredients: &Inventory) -> Option<usize> {
        let mut result = 1;
        for resource in Resource::ALL.iter() {
            if ingredients[*resource] > 0 && self.robots[*resource] == 0 {
                return None;
            }
            let available = self.resources[*resource];
            let needed = ingredients[*resource];
            let production_rate = self.robots[*resource];
            if needed > available {
                let time = 1 + (needed - available + production_rate - 1) / production_rate;
                result = std::cmp::max(result, time);
            }
        }
        Some(result as usize)
    }
}

fn recipe_parser(input: &str) -> IResult<&str, (Resource, Inventory)> {
    tuple((
        tag("Each "),
        alpha1.map(|s: &str| s.parse::<Resource>().unwrap()),
        tag(" robot costs "),
        Inventory::parser,
        tag("."),
    ))
    .map(|(_, target, _, ingredients, _)| (target, ingredients))
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
        let expected = BluePrint::new(
            1,
            vec![
                (
                    Resource::Ore,
                    Inventory {
                        ore: 4,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                ),
                (
                    Resource::Clay,
                    Inventory {
                        ore: 2,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                ),
                (
                    Resource::Obsidian,
                    Inventory {
                        ore: 3,
                        clay: 14,
                        obsidian: 0,
                        geode: 0,
                    },
                ),
                (
                    Resource::Geode,
                    Inventory {
                        ore: 2,
                        clay: 0,
                        obsidian: 7,
                        geode: 0,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(rest, "");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_inputmodel() {
        let actual = test_input().blueprints.len();
        assert_eq!(actual, 2);
    }

    //#[test]
    fn test_quality() {
        let actual = test_input().blueprints[0].quality();
        assert_eq!(actual, 9);
        let actual = test_input().blueprints[1].quality();
        assert_eq!(actual, 24);
    }
    #[test]
    fn test_factory_production() {
        let blueprint = test_input().blueprints[0].clone();
        let mut factory = Factory::new();
        factory.robots.add_resource(Resource::Ore, 1);
        let factory = factory.produce_nothing();
        assert_eq!(
            factory.resources,
            Inventory {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0
            }
        );
        assert_eq!(
            factory.robots,
            Inventory {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0
            }
        );

        assert!(!factory
            .resources
            .has_all_resources(&blueprint.recipes[&Resource::Clay]));

        let factory = factory.produce_nothing();
        assert_eq!(
            factory.resources,
            Inventory {
                ore: 2,
                clay: 0,
                obsidian: 0,
                geode: 0
            }
        );
        assert_eq!(
            factory.robots,
            Inventory {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0
            }
        );

        assert!(factory
            .resources
            .has_all_resources(&blueprint.recipes[&Resource::Clay]));

        let factory = factory.produce_robot(&Resource::Clay, &blueprint.recipes[&Resource::Clay]);
        assert_eq!(
            factory.resources,
            Inventory {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0
            }
        );
        assert_eq!(
            factory.robots,
            Inventory {
                ore: 1,
                clay: 1,
                obsidian: 0,
                geode: 0
            }
        );

        let factory = factory.produce_nothing();
        let factory = factory.produce_robot(&Resource::Clay, &blueprint.recipes[&Resource::Clay]);
        assert_eq!(
            factory.resources,
            Inventory {
                ore: 1,
                clay: 2,
                obsidian: 0,
                geode: 0
            }
        );
        assert_eq!(
            factory.robots,
            Inventory {
                ore: 1,
                clay: 2,
                obsidian: 0,
                geode: 0
            }
        );

        let factory = factory.produce_nothing();
        let factory = factory.produce_robot(&Resource::Clay, &blueprint.recipes[&Resource::Clay]);
        // minute 7
        assert_eq!(
            factory.resources,
            Inventory {
                ore: 1,
                clay: 6,
                obsidian: 0,
                geode: 0
            }
        );
        assert_eq!(
            factory.robots,
            Inventory {
                ore: 1,
                clay: 3,
                obsidian: 0,
                geode: 0
            }
        );

        let factory = factory.produce_nothing();
        let factory = factory.produce_nothing();
        assert!(!factory
            .resources
            .has_all_resources(&blueprint.recipes[&Resource::Obsidian]));
        let factory = factory.produce_nothing();
        assert!(factory
            .resources
            .has_all_resources(&blueprint.recipes[&Resource::Obsidian]));
        let factory =
            factory.produce_robot(&Resource::Obsidian, &blueprint.recipes[&Resource::Obsidian]);
        // minute 11
        assert_eq!(
            factory.resources,
            Inventory {
                ore: 2,
                clay: 4,
                obsidian: 0,
                geode: 0
            }
        );
        assert_eq!(
            factory.robots,
            Inventory {
                ore: 1,
                clay: 3,
                obsidian: 1,
                geode: 0
            }
        );

        let factory = factory.produce_robot(&Resource::Clay, &blueprint.recipes[&Resource::Clay]);
        let factory = factory.produce_nothing();
        let factory = factory.produce_nothing();
        let factory =
            factory.produce_robot(&Resource::Obsidian, &blueprint.recipes[&Resource::Obsidian]);
        // minute 15

        let factory = factory.produce_nothing();
        assert!(!factory
            .resources
            .has_all_resources(&blueprint.recipes[&Resource::Geode]));
        let factory = factory.produce_nothing();
        assert!(factory
            .resources
            .has_all_resources(&blueprint.recipes[&Resource::Geode]));
        let factory = factory.produce_robot(&Resource::Geode, &blueprint.recipes[&Resource::Geode]);
        // minute 18
        assert_eq!(
            factory.resources,
            Inventory {
                ore: 2,
                clay: 17,
                obsidian: 3,
                geode: 0
            }
        );
        assert_eq!(
            factory.robots,
            Inventory {
                ore: 1,
                clay: 4,
                obsidian: 2,
                geode: 1
            }
        );

        let factory = factory.produce_nothing();
        let factory = factory.produce_nothing();
        let factory = factory.produce_robot(&Resource::Geode, &blueprint.recipes[&Resource::Geode]);
        let factory = factory.produce_nothing();
        let factory = factory.produce_nothing();
        let factory = factory.produce_nothing();
        // minute 24
        assert_eq!(
            factory.resources,
            Inventory {
                ore: 6,
                clay: 41,
                obsidian: 8,
                geode: 9
            }
        );
        assert_eq!(
            factory.robots,
            Inventory {
                ore: 1,
                clay: 4,
                obsidian: 2,
                geode: 2
            }
        );
    }

    #[test]
    fn test_earliest_production() {
        let blueprint = test_input().blueprints[0].clone();
        let mut factory = Factory::new();
        factory.robots.add_resource(Resource::Ore, 1);

        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Ore]),
            Some(5)
        );
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Clay]),
            Some(3)
        );
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Obsidian]),
            None
        );
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Geode]),
            None
        );

        factory.resources.add_resource(Resource::Ore, 1); // ensure sufficient ore
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Clay]),
            Some(2)
        );
        factory.resources.add_resource(Resource::Ore, 1); // ensure sufficient ore
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Clay]),
            Some(1)
        );
        factory.resources.add_resource(Resource::Ore, 1); // ensure sufficient ore
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Clay]),
            Some(1)
        );

        factory.robots.add_resource(Resource::Clay, 1);
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Obsidian]),
            Some(15)
        );
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Geode]),
            None
        );

        factory.robots.add_resource(Resource::Clay, 1);
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Obsidian]),
            Some(8)
        );
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Geode]),
            None
        );

        factory.robots.add_resource(Resource::Obsidian, 1);
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Geode]),
            Some(8)
        );

        factory.resources.add_resource(Resource::Obsidian, 6);
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Geode]),
            Some(2)
        );
        factory.resources.add_resource(Resource::Obsidian, 1);
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Geode]),
            Some(1)
        );
    
        // end minute 19
        factory.resources = Inventory {
            ore: 3,
            clay: 21,
            obsidian: 5,
            geode: 1
        };
        assert_eq!(
            factory.earliest_production(&blueprint.recipes[&Resource::Geode]),
            Some(2)
        );
        

    }

    #[test]
    fn test_best_production() {
        let actual = test_input().blueprints[0].best_production(2);
        println!("actual : {:?}", actual);
        assert_eq!(
            actual.robots,
            Inventory {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0
            }
            );
        assert_eq!(
            actual.resources,
            Inventory {
                ore: 2,
                clay: 0,
                obsidian: 0,
                geode: 0
            }
            );
        let actual = test_input().blueprints[0].best_production(4);
        println!("actual : {:?}", actual);
        assert_eq!(
            actual.robots,
            Inventory {
                ore: 1,
                clay: 1,
                obsidian: 0,
                geode: 0
            }
            );
        assert_eq!(
            actual.resources,
            Inventory {
                ore: 2,
                clay: 1,
                obsidian: 0,
                geode: 0
            }
            );
    }

    #[test]
    fn test_best_production_3() {
        let actual = test_input().blueprints[0].best_production(8);
        println!("actual : {:?}", actual);
        assert_eq!(
            actual.robots,
            Inventory {
                ore: 1,
                clay: 3,
                obsidian: 0,
                geode: 0
            }
            );
        assert_eq!(
            actual.resources,
            Inventory {
                ore: 2,
                clay: 9,
                obsidian: 0,
                geode: 0
            }
            );
        let actual = test_input().blueprints[0].best_production(12);
        println!("actual : {:?}", actual);
        assert_eq!(
            actual.robots,
            Inventory {
                ore: 1,
                clay: 3,
                obsidian: 1,
                geode: 0
            }
            );
        assert_eq!(
            actual.resources,
            Inventory {
                ore: 3,
                clay: 7,
                obsidian: 1,
                geode: 0
            }
            );
    }

    #[test]
    fn test_max_geode() {

        let blueprint = test_input().blueprints;

        for i in 1..=24 {
            let actual = blueprint[0].max_geodes(i);
            println!("actual({}) : {:?}", i, actual);
        }
    }

    //#[test]
    fn test_best_geode_production() {
        println!("test_best_geode_production");

        let actual = test_input().blueprints[0].best_production(24);
        println!("actual : {:?}", actual);

        let actual = test_input().blueprints[1].best_production(24);
        println!("actual : {:?}", actual);

        let actual = test_input().blueprints.iter()
            .map(|b| b.best_production(24))
            .map(|f| f.resources.geode)
            .collect::<Vec<_>>();
        let expected = vec![9, 12];

        assert_eq!(actual, expected);
    }

}
