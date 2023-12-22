use std::{
    collections::{BTreeMap, HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub bricks: Vec<Brick>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl From<ParseIntError> for AocError {
    fn from(_: ParseIntError) -> Self {
        AocError::ParseError
    }
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bricks = s
            .lines()
            .map(|line| line.parse::<Brick>())
            .collect::<Result<Vec<Brick>, AocError>>()?;
        Ok(InputModel { bricks })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Position {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let x = parts.next().ok_or(AocError::ParseError)?.parse::<i32>()?;
        let y = parts.next().ok_or(AocError::ParseError)?.parse::<i32>()?;
        let z = parts.next().ok_or(AocError::ParseError)?.parse::<i32>()?;
        Ok(Position { x, y, z })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Brick {
    head: Position,
    tail: Position,
}

impl FromStr for Brick {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("~");
        let head = parts
            .next()
            .and_then(|s| s.parse::<Position>().ok())
            .ok_or(AocError::ParseError)?;
        let tail = parts
            .next()
            .and_then(|s| s.parse::<Position>().ok())
            .ok_or(AocError::ParseError)?;
        Ok(Brick { head, tail })
    }
}

impl Brick {
    pub fn x_min(&self) -> i32 {
        self.head.x.min(self.tail.x)
    }

    pub fn x_max(&self) -> i32 {
        self.head.x.max(self.tail.x)
    }

    pub fn y_min(&self) -> i32 {
        self.head.y.min(self.tail.y)
    }

    pub fn y_max(&self) -> i32 {
        self.head.y.max(self.tail.y)
    }

    pub fn z_min(&self) -> i32 {
        self.head.z.min(self.tail.z)
    }

    pub fn z_max(&self) -> i32 {
        self.head.z.max(self.tail.z)
    }

    pub fn xy_overlaps_with(&self, other: &Brick) -> bool {
        let x_overlap = overlap(self.head.x, self.tail.x, other.head.x, other.tail.x);
        let y_overlap = overlap(self.head.y, self.tail.y, other.head.y, other.tail.y);
        return x_overlap.is_some() && y_overlap.is_some();
    }

    pub fn drop(&self, dz: i32) -> Brick {
        Brick {
            head: Position {
                x: self.head.x,
                y: self.head.y,
                z: self.head.z - dz,
            },
            tail: Position {
                x: self.tail.x,
                y: self.tail.y,
                z: self.tail.z - dz,
            },
        }
    }
}

fn overlap(a1: i32, a2: i32, b1: i32, b2: i32) -> Option<(i32, i32)> {
    let (a_min, a_max) = (a1.min(a2), a1.max(a2));
    let (b_min, b_max) = (b1.min(b2), b1.max(b2));
    if b_max < a_min || b_min > a_max {
        None
    } else {
        let r_min = b_min.max(a_min);
        let r_max = b_max.min(a_max);
        if r_max >= r_min {
            Some((r_min, r_max))
        } else {
            None
        }
    }
}

fn bricks_by_height(bricks: &[Brick]) -> BTreeMap<i32, Vec<Brick>> {
    let mut bricks_by_z: BTreeMap<i32, Vec<Brick>> = BTreeMap::new();
    bricks.iter().for_each(|brick| {
        let z_min = brick.head.z.min(brick.tail.z);
        bricks_by_z.entry(z_min).or_default().push(*brick);
    });
    bricks_by_z
}

fn settle_bricks(bricks: &[Brick]) -> Vec<Brick> {
    let bricks_by_z = bricks_by_height(bricks);
    let mut todo = bricks_by_z.values().flatten().collect::<Vec<_>>();
    todo.sort_by_key(|b| -b.z_min());
    let mut height_map = [[0 as i32; 10]; 10];

    let mut pile = Vec::new();
    while let Some(b) = todo.pop() {
        let max_floor = (b.x_min()..=b.x_max())
            .map(|x| {
                (b.y_min()..=b.y_max())
                    .map(|y| height_map[x as usize][y as usize])
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        let new_z = max_floor + 1;
        let new_b = b.drop(b.z_min() - new_z);
        pile.push(new_b);
        (b.x_min()..=b.x_max()).for_each(|x| {
            (b.y_min()..=b.y_max()).for_each(|y| {
                height_map[x as usize][y as usize] = new_b.z_max();
            });
        });
    }
    pile
}

pub struct BrickPile {
    bricks: Vec<Brick>,
    bricks_by_top: HashMap<i32, Vec<Brick>>,
    bricks_by_bottom: HashMap<i32, Vec<Brick>>,
}

impl BrickPile {
    pub fn new(bricks: &[Brick]) -> BrickPile {
        let bricks = settle_bricks(bricks);
        let bricks_by_top = bricks.iter().map(|&b| (b.z_max(), b)).into_group_map();
        let bricks_by_bottom = bricks.iter().map(|&b| (b.z_min(), b)).into_group_map();
        BrickPile {
            bricks,
            bricks_by_top,
            bricks_by_bottom,
        }
    }

    fn supported_bricks(&self, brick: &Brick) -> Vec<Brick> {
        if let Some(supported) = self.bricks_by_bottom.get(&(brick.z_max() + 1)) {
            supported
                .iter()
                .filter(|b| brick.xy_overlaps_with(b))
                .cloned()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    }

    fn supporting_bricks(&self, brick: &Brick) -> Vec<Brick> {
        if let Some(supporting) = self.bricks_by_top.get(&(brick.z_min() - 1)) {
            supporting
                .iter()
                .filter(|b| brick.xy_overlaps_with(b))
                .cloned()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    }

    pub fn disintegrable_bricks(&self) -> Vec<Brick> {
        self.bricks
            .iter()
            .filter(|brick| {
                let supported = self.supported_bricks(brick);
                let n_b = supported.len();
                // check how many supports have the supported bricks
                let s_cnt = supported
                    .iter()
                    .map(|s| self.supporting_bricks(s).len())
                    .collect::<Vec<_>>();
                // check if any is only supported by this brick
                !s_cnt.iter().any(|c| *c == 1)
            })
            .cloned()
            .collect::<Vec<_>>()
    }

    fn chain_reaction(&self, brick: &Brick) -> Vec<Brick> {
        let mut todo = Vec::new();
        todo.push(*brick);
        let mut disintegrated = HashSet::<Brick>::new();

        let is_unsupported = |brick: &Brick, di: &HashSet<Brick>| {
            self.supporting_bricks(brick)
                .iter()
                .filter(|b| !&di.contains(b))
                .next()
                .is_none()
        };

        while let Some(brick) = todo.pop() {
            {
                let _ = &disintegrated.insert(brick.clone());
            }

            // find brick supported by this one
            let supported = self.supported_bricks(&brick);
            let unsupported = supported
                .iter()
                .filter(|&b| is_unsupported(b, &disintegrated))
                .cloned()
                .collect::<Vec<_>>();

            unsupported.iter().for_each(|&b| {
                todo.push(b.clone());

            })
        }

        disintegrated.into_iter()
            .filter(|b| b != brick) // remove start brick
            .collect::<Vec<_>>()
    }

    pub fn chain_reaction_sum(&self) -> usize {
        self.bricks.iter()
            .map(|b| self.chain_reaction(b).len())
            .sum()
    }
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;

    use super::*;

    const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    fn named_bricks() -> BTreeMap<String, Brick> {
        let bricks = TEST_INPUT.parse::<InputModel>().unwrap().bricks;
        ('A'..'Z')
            .map(|c| c.to_string())
            .zip(bricks)
            .collect::<BTreeMap<String, Brick>>()
    }

    fn brick_pile() -> BrickPile {
        let bricks = TEST_INPUT.parse::<InputModel>().unwrap().bricks;
        BrickPile::new(&bricks)
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = InputModel {
            bricks: vec![
                Brick {
                    // A
                    head: Position { x: 1, y: 0, z: 1 },
                    tail: Position { x: 1, y: 2, z: 1 },
                },
                Brick {
                    // B
                    head: Position { x: 0, y: 0, z: 2 },
                    tail: Position { x: 2, y: 0, z: 2 },
                },
                Brick {
                    // C
                    head: Position { x: 0, y: 2, z: 3 },
                    tail: Position { x: 2, y: 2, z: 3 },
                },
                Brick {
                    // D
                    head: Position { x: 0, y: 0, z: 4 },
                    tail: Position { x: 0, y: 2, z: 4 },
                },
                Brick {
                    // E
                    head: Position { x: 2, y: 0, z: 5 },
                    tail: Position { x: 2, y: 2, z: 5 },
                },
                Brick {
                    // F
                    head: Position { x: 0, y: 1, z: 6 },
                    tail: Position { x: 2, y: 1, z: 6 },
                },
                Brick {
                    // G
                    head: Position { x: 1, y: 1, z: 8 },
                    tail: Position { x: 1, y: 1, z: 9 },
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_overlap() {
        let actual = overlap(11, 15, 1, 5);
        assert_eq!(None, actual);

        let actual = overlap(11, 15, 1, 25);
        assert_eq!(Some((11, 15)), actual);

        let actual = overlap(11, 15, 1, 13);
        assert_eq!(Some((11, 13)), actual);

        let actual = overlap(11, 15, 13, 25);
        assert_eq!(Some((13, 15)), actual);

        let actual = overlap(11, 15, 12, 13);
        assert_eq!(Some((12, 13)), actual);
    }

    #[test]
    fn test_brick_xy_overlap() {
        let bricks = named_bricks();

        assert!(bricks["A"].xy_overlaps_with(&bricks["B"]));
        assert!(!bricks["B"].xy_overlaps_with(&bricks["C"]));
    }

    #[test]
    fn test_settle_bricks() {
        let bricks = named_bricks();
        let pile = settle_bricks(&bricks.values().cloned().collect::<Vec<_>>());

        assert_eq!(pile[0], bricks["A"].drop(0));
        assert_eq!(pile[1], bricks["B"].drop(0));
        assert_eq!(pile[2], bricks["C"].drop(1));
        assert_eq!(pile[3], bricks["D"].drop(1));
        assert_eq!(pile[4], bricks["E"].drop(2));
        assert_eq!(pile[5], bricks["F"].drop(2));
        assert_eq!(pile[6], bricks["G"].drop(3));
    }

    #[test]
    fn test_disintegrable_bricks() {
        let bricks = named_bricks();
        let pile = brick_pile();
        let disintegrable = pile.disintegrable_bricks();

        assert_eq!(disintegrable[0], bricks["B"].drop(0));
        assert_eq!(disintegrable[1], bricks["C"].drop(1));
        assert_eq!(disintegrable[2], bricks["D"].drop(1));
        assert_eq!(disintegrable[3], bricks["E"].drop(2));
        assert_eq!(disintegrable[4], bricks["G"].drop(3));

        assert_eq!(disintegrable.len(), 5);
    }

    #[test]
    fn test_chain_reaction() {
        let bricks = named_bricks();
        let pile = brick_pile();

        let reaction_a = pile.chain_reaction(&bricks["A"]);
        assert_eq!(reaction_a.len(), 6);
        let reaction_b = pile.chain_reaction(&bricks["B"]);
        assert_eq!(reaction_b.len(), 0);
        let reaction_f = pile.chain_reaction(&bricks["F"].drop(2));
        assert_eq!(reaction_f.len(), 1);
        let reaction_g = pile.chain_reaction(&bricks["G"].drop(3));
        assert_eq!(reaction_g.len(), 0);
    }


    #[test]
    fn test_chain_reaction_sum() {
        let pile = brick_pile();
        assert_eq!(7, pile.chain_reaction_sum())
    }
}
