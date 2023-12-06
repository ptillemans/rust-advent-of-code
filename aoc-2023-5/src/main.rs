#![feature(test)]
use std::collections::HashMap;

use aoc_2023_5::{AocError, InputModel, MapRange};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let min_location = input
        .seeds
        .iter()
        .filter_map(|seed| {
            transform_chain(&input.mappings, "seed", "location", *seed)
                .iter()
                .map(|(_, value)| *value)
                .last()
        })
        .min();

    Ok(min_location.unwrap().to_string())
}

fn transform(
    transforms: &HashMap<String, (String, Vec<MapRange>)>,
    source: &str,
    value: u64,
) -> Option<(String, u64)> {
    transforms.get(source).and_then(|(destination, ranges)| {
        ranges
            .iter()
            .find_map(|range| {
                range
                    .transform(value)
                    .map(|transformed| (destination.clone(), transformed))
            })
            .or_else(|| Some((destination.clone(), value)))
    })
}

fn transform_chain(
    transforms: &HashMap<String, (String, Vec<MapRange>)>,
    source: &str,
    target: &str,
    value: u64,
) -> Vec<(String, u64)> {
    let mut source = source.to_string();
    let mut value = value;
    let mut acc = vec![(source.clone(), value)];

    while let Some((new_source, new_value)) = transform(transforms, &source, value) {
        source = new_source;
        value = new_value;
        acc.push((source.clone(), value));
        if source == target {
            break;
        }
    }

    return acc;
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let available_seeds: Vec<(u64, u64)> = input
        .seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect();
    let deltas = collapse_mapranges(input, "seed", "location");

    let sus = deltas
        .iter()
        .filter(|(start, delta)| start + delta == 0)
        .collect::<Vec<_>>();


    let triplets: Vec<(i64, i64, i64)> = deltas
        .iter()
        .zip(deltas.iter().skip(1))
        .map(|((start1, delta1), (start2, delta2))| (*start1, *start2, *delta1))
        .collect();

    let mut relevant_triplets: Vec<(i64, i64, i64)> = vec![];
    for (seed_start, seed_end) in available_seeds {
        for (start, end, delta) in triplets.iter() {
            if *end < seed_start as i64 || *start >= seed_end as i64 {
                continue;
            }
            let min = if *start > seed_start as i64 { *start } else {seed_start as i64};
            let max = if *end < seed_end as i64 { *end } else {seed_end as i64};
            relevant_triplets.push((min, max, min + *delta));
        }
    }

    let min_locations = relevant_triplets
        .iter()
        .map(|(_, _,  min_loc)| min_loc) 
        .collect::<Vec<_>>();

    let min_location = min_locations.iter().min().unwrap();
    
    Ok(min_location.to_string())
}

fn mapranges_to_deltas(mapranges: &[MapRange]) -> Vec<(i64, i64)> {
    let mut mapranges = mapranges.to_vec();
    let mut last_delta: i64 = 0;
    let mut next_event: i64 = 0;
    mapranges.sort_by_key(|maprange| maprange.from);
    let mut deltas: Vec<(i64, i64)> = mapranges.iter().fold(vec![], |mut acc, maprange| {
        let delta = maprange.to as i64 - maprange.from as i64;

        if next_event > 0 && next_event < maprange.from as i64 {
            acc.push((next_event, 0));
            last_delta = 0;
        }
        if last_delta != delta {
            acc.push((maprange.from as i64, delta));
        }
        next_event = maprange.from as i64 + maprange.length as i64;
        last_delta = delta;
        acc
    });
    deltas.push((next_event, 0));
    deltas
}

fn merge_deltas(deltas_1: &[(i64, i64)], deltas_2: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut acc = vec![];

    let mut last: i64 = 0;
    let mut offset = 0;

    for i in 0..deltas_1.len() {
        let (start1, delta1) = deltas_1[i];
        let (_, delta2) = deltas_2
            .iter()
            .take_while(|(start2, _)| *start2 < start1 + delta1)
            .last()
            .unwrap_or(&(0, 0));
        let next2: Vec<&(i64, i64)> = deltas_2
            .iter()
            .skip_while(|(start2, _)| *start2 < last + offset)
            .take_while(|(start2, _)| *start2 < start1 + offset)
            .collect();
        for (start2, delta2) in next2 {
            acc.push((start2 - offset, delta2 + offset));
        }
        acc.push((start1, delta1 + delta2));
        last = start1;
        offset = delta1;
    }
    for (start2, delta2) in deltas_2.iter().skip_while(|(start2, _)| *start2 < last) {
        acc.push((*start2, *delta2));
    }

    let first = *acc.first().unwrap();
    let shifted = acc[1..].to_vec();
    let mut events: Vec<(i64, i64)> = acc
        .into_iter()
        .zip(shifted.into_iter())
        .filter_map(|((start1, delta1), (start2, delta2))| {
            if delta1 != delta2 {
                Some((start2, delta2))
            } else {
                None
            }
        })
        .map(|(start, delta)| {
            if start == delta {
                println !("***** start == delta: {} {}", start, delta);
            };
            (start, delta)
        })
        .collect();
    events.insert(0, first);
    events
}

fn collapse_mapranges(input: &InputModel, source: &str, destination: &str) -> Vec<(i64, i64)> {
    let mut thing = source.to_string();
    let mut deltas: Vec<(i64, i64)> = vec![(core::i64::MAX, 0)];
    while let Some((next, mapranges)) = input.mappings.get(&thing) {
        deltas = merge_deltas(&deltas, &mapranges_to_deltas(mapranges));
        thing = next.to_string();
        if thing == destination {
            break;
        }
    }
    deltas
}

fn main() -> Result<(), AocError> {
    let input: InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

fn apply_deltas(deltas: &[(i64, i64)], value: i64) -> i64 {
    let delta = deltas
        .iter()
        .take_while(|(from, _)| value >= *from)
        .last()
        .unwrap_or(&(0, 0))
        .1;
    value + delta
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    pub fn input_data() -> InputModel {
        InputModel {
            seeds: vec![79, 14, 55, 13],
            mappings: vec![
                (
                    "seed".to_string(),
                    (
                        "soil".to_string(),
                        vec![
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
                    ),
                ),
                (
                    "soil".to_string(),
                    (
                        "fertilizer".to_string(),
                        vec![
                            MapRange {
                                to: 0,
                                from: 15,
                                length: 37,
                            },
                            MapRange {
                                to: 37,
                                from: 52,
                                length: 2,
                            },
                            MapRange {
                                to: 39,
                                from: 0,
                                length: 15,
                            },
                        ],
                    ),
                ),
                (
                    "fertilizer".to_string(),
                    (
                        "water".to_string(),
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
                    ),
                ),
                (
                    "water".to_string(),
                    (
                        "light".to_string(),
                        vec![
                            MapRange {
                                to: 88,
                                from: 18,
                                length: 7,
                            },
                            MapRange {
                                to: 18,
                                from: 25,
                                length: 70,
                            },
                        ],
                    ),
                ),
                (
                    "light".to_string(),
                    (
                        "temperature".to_string(),
                        vec![
                            MapRange {
                                to: 45,
                                from: 77,
                                length: 23,
                            },
                            MapRange {
                                to: 81,
                                from: 45,
                                length: 19,
                            },
                            MapRange {
                                to: 68,
                                from: 64,
                                length: 13,
                            },
                        ],
                    ),
                ),
                (
                    "temperature".to_string(),
                    (
                        "humidity".to_string(),
                        vec![
                            MapRange {
                                to: 0,
                                from: 69,
                                length: 1,
                            },
                            MapRange {
                                to: 1,
                                from: 0,
                                length: 69,
                            },
                        ],
                    ),
                ),
                (
                    "humidity".to_string(),
                    (
                        "location".to_string(),
                        vec![
                            MapRange {
                                to: 60,
                                from: 56,
                                length: 37,
                            },
                            MapRange {
                                to: 56,
                                from: 93,
                                length: 4,
                            },
                        ],
                    ),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "35";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_transform() {
        let transforms = input_data().mappings;
        assert_eq!(
            Some(("soil".to_string(), 81)),
            transform(&transforms, "seed", 79)
        );
        assert_eq!(
            Some(("soil".to_string(), 14)),
            transform(&transforms, "seed", 14)
        );
        assert_eq!(
            Some(("soil".to_string(), 57)),
            transform(&transforms, "seed", 55)
        );
        assert_eq!(
            Some(("soil".to_string(), 13)),
            transform(&transforms, "seed", 13)
        );
    }

    #[test]
    fn test_transform_chain() {
        let transforms = input_data().mappings;
        let cases = vec![
            vec![
                ("seed".to_string(), 79),
                ("soil".to_string(), 81),
                ("fertilizer".to_string(), 81),
                ("water".to_string(), 81),
                ("light".to_string(), 74),
                ("temperature".to_string(), 78),
                ("humidity".to_string(), 78),
                ("location".to_string(), 82),
            ],
            vec![
                ("seed".to_string(), 14),
                ("soil".to_string(), 14),
                ("fertilizer".to_string(), 53),
                ("water".to_string(), 49),
                ("light".to_string(), 42),
                ("temperature".to_string(), 42),
                ("humidity".to_string(), 43),
                ("location".to_string(), 43),
            ],
            vec![
                ("seed".to_string(), 55),
                ("soil".to_string(), 57),
                ("fertilizer".to_string(), 57),
                ("water".to_string(), 53),
                ("light".to_string(), 46),
                ("temperature".to_string(), 82),
                ("humidity".to_string(), 82),
                ("location".to_string(), 86),
            ],
            vec![
                ("seed".to_string(), 13),
                ("soil".to_string(), 13),
                ("fertilizer".to_string(), 52),
                ("water".to_string(), 41),
                ("light".to_string(), 34),
                ("temperature".to_string(), 34),
                ("humidity".to_string(), 35),
                ("location".to_string(), 35),
            ],
        ];
        for expected in cases {
            let (source, value) = expected[0].clone();
            let actual = transform_chain(&transforms, &source, "location", value);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_mapranges_to_deltas() {
        let mapranges = vec![
            MapRange {
                from: 18,
                to: 88,
                length: 7,
            },
            MapRange {
                from: 25,
                to: 18,
                length: 70,
            },
        ];

        let expected = vec![(18, 70), (25, -7), (95, 0)];
        let actual = mapranges_to_deltas(&mapranges);
        assert_eq!(actual, expected);

        let mapranges = vec![
            MapRange {
                from: 15,
                to: 0,
                length: 37,
            },
            MapRange {
                from: 52,
                to: 37,
                length: 2,
            },
            MapRange {
                from: 54,
                to: 39,
                length: 15,
            },
        ];
        let expected = vec![(15, -15), (69, 0)];
        let actual = mapranges_to_deltas(&mapranges);
        assert_eq!(actual, expected);

        let mapranges = vec![
            MapRange {
                from: 15,
                to: 0,
                length: 37,
            },
            MapRange {
                from: 54,
                to: 39,
                length: 15,
            },
        ];
        let expected = vec![(15, -15), (52, 0), (54, -15), (69, 0)];
        let actual = mapranges_to_deltas(&mapranges);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge_deltas() {
        let deltas_1 = vec![(18, 70), (25, -7), (95, 0)];
        let deltas_2 = vec![(0, 39), (15, -15), (52, -15), (54, 0)];

        let expected = vec![(0, 39), (15, -15), (18, 70), (25, -22), (61, -7), (95, 0)];
        let actual = merge_deltas(&deltas_1, &deltas_2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_collapse_mapranges() {
        let mappings = &input_data().mappings;

        let source = "seed";
        let destination = "location";

        let deltas = collapse_mapranges(&input_data(), source, destination);

        for value in 0..=1000 {
            let actual = apply_deltas(&deltas, value);
            let expected = transform_chain(mappings, source, destination, value as u64)
                .last()
                .unwrap()
                .1;
            println!("{}: {} <-> {}", value, actual, expected);
            assert_eq!(actual, expected as i64);
        }
    }

    #[test]
    fn test_all_mapranges_to_deltas() {
        let mappings = &input_data().mappings;

        for (source, (destination, mapping)) in &input_data().mappings {
            let deltas = mapranges_to_deltas(mapping);
            for value in 0..=1000 {
                let actual = apply_deltas(&deltas, value);
                let expected = transform(mappings, source, value as u64).unwrap().1;
                assert_eq!(actual, expected as i64);
            }
        }
    }

    #[test]
    fn test_inconsistent_deltas() {
        let input = INPUT.parse::<InputModel>().unwrap();
        let source = "seed";
        let target = "fertilizer";
        let seed: u64 = 2720501486;

        let mapping1 = &input.mappings.get(source).unwrap().1;
        let deltas1 = mapranges_to_deltas(mapping1);
        let actual = apply_deltas(&deltas1, seed as i64);
        let expected = transform(&input.mappings, "seed", seed).unwrap().1 as i64;
        assert_eq!(actual, expected);

        let soil = actual as u64;
        let mut mapping2 = input.mappings.get("soil").unwrap().1.clone();
        mapping2.sort_by(|a, b| a.from.cmp(&b.from));
        let deltas2 = mapranges_to_deltas(&mapping2);
        println!("mappings2: {:?}", mapping2);
        println!("deltas2: {:?}", deltas2);
        let actual = apply_deltas(&deltas2, soil as i64);
        let expected = transform(&input.mappings, "soil", soil).unwrap().1 as i64;
        println!("{}: {} <-> {}", soil, actual, expected);
        for (from, _) in deltas2.clone() {
            let from = from - 100000;
            let a = apply_deltas(&deltas2, from as i64);
            let b = transform(&input.mappings, "soil", from as u64).unwrap().1 as i64;
            println!("boundaries {}: {} {} -> {}", from, a, b, a == b);
        }
        assert_eq!(actual, expected);


        let deltas = collapse_mapranges(&input, source, target);
        let actual = apply_deltas(&deltas, seed as i64);
        let expected = transform_chain(&input.mappings, source, target, seed)
            .last()
            .unwrap()
            .1 as i64;

        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "46";

        assert_eq!(actual, expected);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&data))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&data))
    }
}
