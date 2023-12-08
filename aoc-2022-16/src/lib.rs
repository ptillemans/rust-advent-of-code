use std::hash::Hash;
use std::str::FromStr;
use std::collections::HashMap;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::tuple,
    combinator::opt,
};

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub valves: Vec<Valve>,
}

impl InputModel {

    fn parser(s: &str) -> IResult<&str, InputModel> {
        separated_list1(line_ending, Valve::parser)
            .map(|valves| InputModel { valves })
            .parse(s)
    }
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parser(s)
            .map(|(_, input_model)| input_model) 
            .map_err(|_| AocError::ParseError)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ValveId(String);

impl ValveId {
    pub fn new(s: &str) -> Self {
        ValveId(s.to_string())
    }

    fn parser(s: &str) -> IResult<&str, Self> {
        take_while1(|c: char| c.is_alphabetic())
            .map(Self::new)
            .parse(s)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Tunnel {
    to: ValveId,
    distance: u32,
}

impl Tunnel {
    
    pub fn new(s: &str) -> Self {
        let to = ValveId::new(s);
        Tunnel { distance: 1, to }
    }

    fn new_with_distance(s: &str, distance: u32) -> Self {
        let to = ValveId::new(s);
        Tunnel { distance, to }
    }

    fn parser(s: &str) -> IResult<&str, Self> {
        ValveId::parser
            .map(|to| Tunnel{ distance: 1, to })
            .parse(s)
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Valve {
    id: ValveId,
    flow: u32,
    tunnels: HashMap<ValveId, u32>,
}

impl Valve {

    fn parser(s: &str) -> IResult<&str, Self> {
        tuple((
            tag("Valve "),
            ValveId::parser,
            tag(" has flow rate="),
            digit1.map(|s: &str| s.parse::<u32>().unwrap()),
            tuple((tag("; tunnel"), opt(tag("s")), 
                   tag(" lead"), opt(tag("s")),
                   tag(" to valve"),opt(tag("s")),
                   space1)),
            separated_list1(tag(", "), Tunnel::parser),
        ))
            .map(|(_, id, _, flow, _, tunnels)| Valve{id, flow, 
                tunnels: tunnels.iter().map(|t| (t.to.clone(), t.distance)).collect()
            })
            .parse(s)
    }

}


fn simplify_once(valves: &[Valve]) -> Option<Vec<Valve>> {
    let mut valve_map: HashMap<ValveId, Valve> = valves.iter()
        .map(|v| (v.id.clone(), v.clone()))
        .collect();

    valves.iter().find(|v| v.flow == 0 && v.tunnels.len() == 2)
        .map(|valve | {
            let tunnels = valve.tunnels.iter().collect::<Vec<_>>();
            let valve_id0 = tunnels[0].0.clone();
            let valve_id1 = tunnels[1].0.clone();
            let distance = tunnels[0].1 + tunnels[1].1;


            if let Some(v) = valve_map.get_mut(&valve_id0) {
                v.tunnels.remove(&valve.id);
                v.tunnels.insert(valve_id1.clone(), distance);
            };

            if let Some(v) = valve_map.get_mut(&valve_id1) {
                v.tunnels.remove(&valve.id);
                v.tunnels.insert(valve_id0, distance);
            };

            valve_map.remove(&valve.id);

            let valves: Vec<Valve> = valve_map.values().cloned().collect();
            valves
        })

}

fn simplify(valves: &[Valve]) -> Vec<Valve> {
    let mut valves = valves.to_vec();
    let _ = std::iter::from_fn(|| {
        simplify_once(&valves).map(|v| {
            valves = v;
        })
    })
        .last();
    valves
}



pub const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

fn tunnels_to_map(tunnels: &[Tunnel]) -> HashMap<ValveId, u32> {
    tunnels.iter().map(|t| (t.to.clone(), t.distance)).collect()
}
pub fn input_data() -> InputModel {
    InputModel {
        valves: vec![
            Valve {
                id: ValveId::new("AA"),
                flow: 0,
                tunnels: tunnels_to_map(&[Tunnel::new("DD"), Tunnel::new("II"), Tunnel::new("BB")]),
            },
            Valve {
                id: ValveId::new("BB"),
                flow: 13,
                tunnels: tunnels_to_map(&[Tunnel::new("CC"), Tunnel::new("AA")]),
            },
            Valve {
                id: ValveId::new("CC"),
                flow: 2,
                tunnels: tunnels_to_map(&[ Tunnel::new("DD"), Tunnel::new("BB") ]),
            },
            Valve {
                id: ValveId::new("DD"),
                flow: 20,
                tunnels: tunnels_to_map(&[ Tunnel::new("CC"), Tunnel::new("AA"), Tunnel::new("EE") ]),
            },
            Valve {
                id: ValveId::new("EE"),
                flow: 3,
                tunnels: tunnels_to_map(&[ Tunnel::new("FF"), Tunnel::new("DD") ]),
            },
            Valve {
                id: ValveId::new("FF"),
                flow: 0,
                tunnels: tunnels_to_map(&[ Tunnel::new("EE"), Tunnel::new("GG") ]),
            },
            Valve {
                id: ValveId::new("GG"),
                flow: 0,
                tunnels: tunnels_to_map(&[ Tunnel::new("FF"), Tunnel::new("HH") ]),
            },
            Valve {
                id: ValveId::new("HH"),
                flow: 22,
                tunnels: tunnels_to_map(&[ Tunnel::new("GG") ]),
            },
            Valve {
                id: ValveId::new("II"),
                flow: 0,
                tunnels: tunnels_to_map(&[ Tunnel::new("AA"), Tunnel::new("JJ") ]),
            },
            Valve {
                id: ValveId::new("JJ"),
                flow: 21,
                tunnels: tunnels_to_map(&[ Tunnel::new("II") ]),
            },
        ],
    }
}

pub fn simplified_graph() -> Vec<Valve> {
    vec![
        Valve {
            id: ValveId::new("AA"),
            flow: 0,
            tunnels: tunnels_to_map(&[Tunnel::new("DD"), Tunnel::new_with_distance("JJ",2), Tunnel::new("BB")]),
        },
        Valve {
            id: ValveId::new("BB"),
            flow: 13,
            tunnels: tunnels_to_map(&[Tunnel::new("CC"), Tunnel::new("AA")]),
        },
        Valve {
            id: ValveId::new("CC"),
            flow: 2,
            tunnels: tunnels_to_map(&[ Tunnel::new("DD"), Tunnel::new("BB") ]),
        },
        Valve {
            id: ValveId::new("DD"),
            flow: 20,
            tunnels: tunnels_to_map(&[ Tunnel::new("CC"), Tunnel::new("AA"), Tunnel::new("EE") ]),
        },
        Valve {
            id: ValveId::new("EE"),
            flow: 3,
            tunnels: tunnels_to_map(&[ Tunnel::new_with_distance("HH",3), Tunnel::new("DD") ]),
        },
        Valve {
            id: ValveId::new("HH"),
            flow: 22,
            tunnels: tunnels_to_map(&[ Tunnel::new_with_distance("EE",3) ]),
        },
        Valve {
            id: ValveId::new("JJ"),
            flow: 21,
            tunnels: tunnels_to_map(&[ Tunnel::new_with_distance("II",2) ]),
        },
    ]
}


// use Floyd-Warshall algorithm to simplify the graph
fn extend_distances(valves: &[Valve]) -> Vec<Valve> {

    let mut valves = simplify(valves);

    let valve_map: HashMap<ValveId, Valve> = valves.iter()
        .map(|v| (v.id.clone(), v.clone()))
        .collect();

    for _ in 0..valves.len() {
        for valve in valves.iter_mut() {
            for (to, distance) in valve.clone().tunnels {
                let v2 = valve_map.get(&to).unwrap();
                for (to2, distance2) in &v2.tunnels {
                    let distance = distance + distance2;
                    let v1 = valve.tunnels.entry(to2.clone()).or_insert(distance);
                    if distance < *v1  {
                        *v1 = distance;
                    }
                }
            }
        }
    }
    valves
}

pub fn best_path(valves: &[Valve], start: &ValveId, time: u32) -> u32{

    let valves = extend_distances(&simplify(valves));


    let valve_map: HashMap<ValveId, Valve> = valves.iter()
        .map(|v| (v.id.clone(), v.clone()))
        .collect();

    let mut open = vec![(vec![start.clone()], vec![(0, time)], 0, time)];
    let mut best = (vec![], vec![], 0, 0);

    while let Some((path, contrib, pressure, time_remaining)) = open.pop() {
        if pressure > best.2 {
            best = (path.clone(), contrib.clone(), pressure, time_remaining);
        }

        let last_valve_id = path.last().unwrap();
        let valve = valve_map.get(last_valve_id).unwrap();

        for (to, distance) in &valve.tunnels {
            if path.contains(to) || time_remaining < distance + 1 {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(to.clone());
            let time_remaining = time_remaining - distance - 1;
            let mut new_contrib = contrib.clone();
            let to_valve = valve_map.get(to).unwrap();
            new_contrib.push((to_valve.flow, time_remaining));
            let new_pressure = pressure + to_valve.flow * time_remaining;
            open.push((new_path, new_contrib, new_pressure, time_remaining));
        }
    };

    best.2
}

pub fn best_path2(valves: &[Valve], start: &ValveId, time: u32) -> u32{

    let valves = extend_distances(&simplify(valves));

    let valve_map: HashMap<ValveId, Valve> = valves.iter()
        .map(|v| (v.id.clone(), v.clone()))
        .collect();

    let mut open = vec![(vec![start.clone()], 0, time, vec![start.clone()], 0, time)];
    let mut best = (vec![], 0, 0, vec![], 0, 0);

    while let Some((path_h, pressure_h, time_remaining_h, path_e, pressure_e, time_remaining_e))  = open.pop() {
        if pressure_h + pressure_e  > best.1 + best.4 {
            best = (path_h.clone(), pressure_h, time_remaining_h, path_e.clone(), pressure_e, time_remaining_e);
        }

        if time_remaining_h > time_remaining_e {
            let last_valve_id = path_h.last().unwrap();
            let valve = valve_map.get(last_valve_id).unwrap();

            for (to, distance) in &valve.tunnels {
                if path_h.contains(to) || path_e.contains(to) || time_remaining_h < distance + 1 {
                    continue;
                }
                let mut new_path = path_h.clone();
                new_path.push(to.clone());
                let time_remaining = time_remaining_h - distance - 1;
                let to_valve = valve_map.get(to).unwrap();
                let new_pressure = pressure_h + to_valve.flow * time_remaining;
                open.push((new_path, new_pressure, time_remaining, path_e.clone(), pressure_e, time_remaining_e));
            }
        } else {
            let last_valve_id = path_e.last().unwrap();
            let valve = valve_map.get(last_valve_id).unwrap();

            for (to, distance) in &valve.tunnels {
                if path_e.contains(to) || path_h.contains(to) || time_remaining_e < distance + 1 {
                    continue;
                }
                let mut new_path = path_e.clone();
                new_path.push(to.clone());
                let time_remaining = time_remaining_e - distance - 1;
                let to_valve = valve_map.get(to).unwrap();
                let new_pressure = pressure_e + to_valve.flow * time_remaining;
                open.push((path_h.clone(), pressure_h, time_remaining_h, new_path, new_pressure, time_remaining));
            }
        };
    };

    best.1 + best.4
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let result = InputModel::parser(TEST_INPUT);
        let expected = input_data();

        let (rest, actual) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_simplify() {
        let input = input_data();
        let expected = simplified_graph();
        let actual = simplify(&input.valves);
        assert_eq!(actual.len(), expected.len());
    }

    #[test]
    fn test_extend_distances() {
        let input = vec![
            Valve{id: ValveId::new("AA"), flow: 1, tunnels: tunnels_to_map(&[Tunnel::new("BB")])},
            Valve{id: ValveId::new("BB"), flow: 1, tunnels: tunnels_to_map(&[Tunnel::new("AA"), Tunnel::new("CC")])},
            Valve{id: ValveId::new("CC"), flow: 1, tunnels: tunnels_to_map(&[Tunnel::new("BB"), Tunnel::new("DD")])},
            Valve{id: ValveId::new("DD"), flow: 1, tunnels: tunnels_to_map(&[Tunnel::new("BB")])},
        ];
        let expected = vec![
            Valve{id: ValveId::new("AA"), 
                flow: 1, 
                tunnels: tunnels_to_map(&[
                   Tunnel::new("BB"), 
                   Tunnel::new_with_distance("CC", 2),
                   Tunnel::new_with_distance("DD", 3),
                ])
            },
            Valve{
                id: ValveId::new("BB"), 
                flow: 1, 
                tunnels: tunnels_to_map(&[
                    Tunnel::new("CC"), 
                    Tunnel::new("AA"),
                    Tunnel::new_with_distance("DD", 2)
                ])
            },
            Valve{id: ValveId::new("CC"), flow: 1, tunnels: tunnels_to_map(&[Tunnel::new("BB"), Tunnel::new_with_distance("AA", 2), Tunnel::new_with_distance("DD", 1)])},
            Valve{id: ValveId::new("DD"), flow: 1, tunnels: tunnels_to_map(&[Tunnel::new("CC"), 
                                                                           Tunnel::new_with_distance("AA", 3),
            Tunnel::new_with_distance("BB", 2)])},
        ];

        let actual = extend_distances(&input);
        assert_eq!(actual.len(), expected.len());
    }

    #[test]
    fn test_extend_distances2() {
        let actual = extend_distances(&input_data().valves);
        assert_eq!(actual.len(), 7);
    }


    #[test]
    fn test_best_path() {

        let input = input_data();
        let actual = best_path(&input.valves, &ValveId::new("AA"), 30);
        let expected = 1651;
        assert_eq!(actual, expected);

    }
    
    #[test]
    fn test_best_path2() {

        let input = input_data();
        let actual = best_path2(&input.valves, &ValveId::new("AA"), 26);
        let expected = 1707;
        assert_eq!(actual, expected);

    }
}
