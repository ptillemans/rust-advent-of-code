use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use std::{
    collections::HashMap,
    ops::{Add, Index, IndexMut, Sub},
    str::FromStr, cmp::Ordering,
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
        Resource::Geode,
        Resource::Obsidian,
        Resource::Clay,
        Resource::Ore,
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

        let mut open = Vec::with_capacity(10000);
        open.push(start);
        let mut best = start;

        // keep a list of maximums inputs for each recipe per resource
        let max_needed : HashMap<Resource, u32> = Resource::ALL.iter()
            .map(|r| (*r, self.recipes.values()
                      .map(|v| v[*r])
                      .max().unwrap_or(0)))
            .collect();

        for time in 0..period {
            let mut new_open = Vec::with_capacity(10000);
            while !open.is_empty() {
                let factory = open.pop().unwrap();
                // check if we can produce a robot
                let can_make: Vec<Resource> = self.recipes.iter()
                    .filter(|(_, inventory)| factory.resources.has_all_resources(inventory))
                    .map(|(resource, _)| *resource)
                    .collect();
                for resource in can_make.iter() {
                    // no need for more robots than to produce the most needed 
                    if max_needed[resource] <= factory.robots[*resource] && *resource != Resource::Geode {
                        continue;
                    }
                    let inventory = self.recipes.get(resource).unwrap();
                    if factory.resources.has_all_resources(inventory) {
                        let factory = factory.produce_robot(resource, inventory);
                        new_open.push(factory);

                        // build geode robot if we can
                        if resource == &Resource::Geode {
                            break;
                        }
                    } else {
                    }
                }

                // do not pile resources if we can build can_build_anything
                if can_make.len() < 4 {
                    new_open.push(factory.produce_nothing());
                }


            }

            //println!("{}: {} factories", time, new_open.len());

            new_open.sort();
            open = last_n(new_open, 3000);
            
            if let Some(factory) = open.last() {
                if factory.resources.geode > best.resources.geode {
                    best = *factory;
                    //println!("New best {}: {:?}", time, best);
                }
            }
        }

        println!("Best result for blueprint {} : {:?}", self.id,  best.resources.geode);
        best
    }

    pub fn quality(&self) -> u32 {
        self.max_geodes(24) * self.id
    }

    pub fn max_geodes(&self, rounds: usize) -> u32 {
        self.best_production(rounds).resources.geode
    }
}

fn last_n<T: Clone>(v: Vec<T>, n: usize) -> Vec<T> {
    let l = v.len();
    if l > n {
        (v[l - n..]).to_vec()
    } else {
        v
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Factory {
    robots: Inventory,
    resources: Inventory,
}

impl Factory {
    fn new() -> Self {
        Self {
            resources: Inventory::new(),
            robots: Inventory::new(),
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

}

impl PartialOrd for Factory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Factory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.resources.geode.cmp(&other.resources.geode)
        .then(self.robots.geode.cmp(&other.robots.geode))
        .then(self.resources.obsidian.cmp(&other.resources.obsidian))
        .then(self.robots.obsidian.cmp(&other.robots.obsidian))
        .then(self.resources.clay.cmp(&other.resources.clay))
        .then(self.robots.clay.cmp(&other.robots.clay))
        .then(self.resources.ore.cmp(&other.resources.ore))
        .then(self.robots.ore.cmp(&other.robots.ore))
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

    #[test]
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
    fn test_max_geode_1() {

        let blueprint = test_input().blueprints;

        let actual = blueprint[0].max_geodes(24);
        println!("actual max geode : {:?}",  actual);
        assert_eq!(actual, 9);
    }

    #[test]
    fn test_max_geode_2() {

        let blueprint = test_input().blueprints;

        let actual = blueprint[1].max_geodes(24);
        println!("actual max geode : {:?}",  actual);
        assert_eq!(actual, 12);
    }


}
