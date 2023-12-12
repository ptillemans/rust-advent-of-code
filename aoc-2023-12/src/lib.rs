use itertools::*;
use std::{str::FromStr, time};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    lines: Vec<MapLine>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MapTile {
    Empty,
    Spring,
    Unknown,
}

impl From<char> for MapTile {
    fn from(c: char) -> Self {
        match c {
            '.' => MapTile::Empty,
            '#' => MapTile::Spring,
            '?' => MapTile::Unknown,
            _ => panic!("Invalid char"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MapLine {
    tiles: Vec<(MapTile, u32)>,
    blocks: Vec<u32>,
}

impl FromStr for MapLine {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(AocError::ParseError);
        }
        let map_tiles = parts[0]
            .chars()
            .map(|c| MapTile::from(c))
            .collect::<Vec<MapTile>>();
        let groups = map_tiles.into_iter().group_by(|t| t.clone());

        let tiles = groups
            .into_iter()
            .map(|(t, g)| (t, g.count() as u32))
            .collect::<Vec<(MapTile, u32)>>();

        let blocks = parts[1]
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Ok(MapLine { tiles, blocks })
    }
}
#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|l| l.parse::<MapLine>().unwrap())
            .collect::<Vec<MapLine>>();
        Ok(InputModel { lines })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_from_str_mapline() {
        let input = TEST_INPUT.lines();
        let expected = vec![
            MapLine {
                tiles: vec![
                    (MapTile::Unknown, 3),
                    (MapTile::Empty, 1),
                    (MapTile::Spring, 3)],
                blocks: vec![1, 1, 3],
            },
            MapLine {
                tiles: vec![
                    (MapTile::Empty, 1),
                    (MapTile::Unknown, 2),
                    (MapTile::Empty, 2),
                    (MapTile::Unknown, 2),
                    (MapTile::Empty, 3),
                    (MapTile::Unknown, 1),
                    (MapTile::Spring, 2),
                    (MapTile::Empty, 1),
                ],
                blocks: vec![1, 1, 3],
            },
            MapLine {
                tiles: vec![
                    (MapTile::Unknown, 1),
                    (MapTile::Spring, 1),
                    (MapTile::Unknown, 1),
                    (MapTile::Spring, 1),
                    (MapTile::Unknown, 1),
                    (MapTile::Spring, 1),
                    (MapTile::Unknown, 1),
                    (MapTile::Spring, 1),
                    (MapTile::Unknown, 1),
                    (MapTile::Spring, 1),
                    (MapTile::Unknown, 1),
                    (MapTile::Spring, 1),
                    (MapTile::Unknown, 1),
                    (MapTile::Spring, 1),
                    (MapTile::Unknown, 1),
                ],

                blocks: vec![1, 3, 1, 6],
            },
            MapLine {
                tiles: vec![
                    (MapTile::Unknown, 4),
                    (MapTile::Empty, 1),
                    (MapTile::Spring, 1),
                    (MapTile::Empty, 3),
                    (MapTile::Spring, 1),
                    (MapTile::Empty, 3),
                ],
                blocks: vec![4, 1, 1],
            },
            MapLine {
                tiles: vec![
                    (MapTile::Unknown, 4),
                    (MapTile::Empty, 1),
                    (MapTile::Spring, 6),
                    (MapTile::Empty, 2),
                    (MapTile::Spring, 5),
                    (MapTile::Empty, 1),
                ],
                blocks: vec![1, 6, 5],
            },
            MapLine {
                tiles: vec![
                    (MapTile::Unknown, 1),
                    (MapTile::Spring, 3),
                    (MapTile::Unknown, 8),
                ],
                blocks: vec![3, 2, 1],
            },
        ];

        for (i, (line, expected)) in input.zip(expected.into_iter()).enumerate() {
            let mapline = line.parse::<MapLine>().unwrap();
            assert_eq!(mapline, expected, "Failed on line {}", i);
        }
    }
}
