use std::{
    num::ParseIntError,
    ops::{Add, Sub},
    str::FromStr,
};

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

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub stones: Vec<HailStone>,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|l| l.parse::<HailStone>())
            .collect::<Result<Vec<HailStone>, AocError>>()
            .map(|stones| InputModel { stones })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector {
    x: i128,
    y: i128,
    z: i128,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl FromStr for Vector {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(", ");
        let x = parts
            .next()
            .ok_or(AocError::ParseError)?
            .trim()
            .parse::<i128>()?;
        let y = parts
            .next()
            .ok_or(AocError::ParseError)?
            .trim()
            .parse::<i128>()?;
        let z = parts
            .next()
            .ok_or(AocError::ParseError)?
            .trim()
            .parse::<i128>()?;
        Ok(Vector { x, y, z })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HailStone {
    position: Vector,
    velocity: Vector,
}

impl FromStr for HailStone {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" @ ");
        let position = parts
            .next()
            .ok_or(AocError::ParseError)?
            .trim()
            .parse::<Vector>()?;
        let velocity = parts
            .next()
            .ok_or(AocError::ParseError)?
            .trim()
            .parse::<Vector>()?;
        Ok(HailStone { position, velocity })
    }
}

fn with_pairs<T>(stones: &[HailStone], f: &dyn Fn(&HailStone, &HailStone) -> T) -> Vec<T> {
    (0..stones.len() - 1)
        .flat_map(|i| (i + 1..stones.len()).map(move |j| f(&stones[i], &stones[j])))
        .collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    fn test_input() -> InputModel {
        TEST_INPUT.parse::<InputModel>().unwrap()
    }

    #[test]
    fn test_vector_fromstr() {
        let actual = "19, 13, 30".parse::<Vector>().unwrap();
        let expected = Vector {
            x: 19,
            y: 13,
            z: 30,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hailstone_from_str() {
        let actual = "19, 13, 30 @ -2,  1, -2".parse::<HailStone>().unwrap();
        let expected = HailStone {
            position: Vector {
                x: 19,
                y: 13,
                z: 30,
            },
            velocity: Vector { x: -2, y: 1, z: -2 },
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hailstone_pairs() {
        let stones = test_input().stones;

        let actual = with_pairs(&stones, &|s1, s2| (*s1, *s2));

        let expected = vec![
            (stones[0], stones[1]),
            (stones[0], stones[2]),
            (stones[0], stones[3]),
            (stones[0], stones[4]),
            (stones[1], stones[2]),
            (stones[1], stones[3]),
            (stones[1], stones[4]),
            (stones[2], stones[3]),
            (stones[2], stones[4]),
            (stones[3], stones[4]),
        ];

        assert_eq!(actual, expected);
    }
}
