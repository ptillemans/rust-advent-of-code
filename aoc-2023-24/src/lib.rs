use std::{
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

use argmin::core::{CostFunction, Executor, Gradient};
use nalgebra::{vector, Const, Matrix, Vector2, Vector3};

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution found")]
    NoSolution,
}

impl From<ParseIntError> for AocError {
    fn from(_: ParseIntError) -> Self {
        AocError::ParseError
    }
}
impl From<ParseFloatError> for AocError {
    fn from(_: ParseFloatError) -> Self {
        AocError::ParseError
    }
}

type FloatScalar = f64;
type Vec3D = Vector3<FloatScalar>;
type Vec2D = Vector2<FloatScalar>;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HailStone {
    pub position: Vec3D,
    pub velocity: Vec3D,
}

fn to_2df(v: &Vec3D) -> Vec2D {
    vector![v.x as FloatScalar, v.y as FloatScalar]
}

impl HailStone {
    fn intersect_2d_in_future(&self, other: &HailStone) -> Option<Vec2D> {
        // P1 + V1.t1 = P2 + V2.t2
        // or
        //   P1x + V1x.t1 = P2x + V2x.t2
        //   P1y + V1y.t1 = P2y + V2y.t2
        //
        //    [V1, -V2] T = P2 - P1
        //
        // solve for T and substitute back to find the point

        let my_position = to_2df(&self.position);
        let my_velocity = to_2df(&self.velocity);
        let other_position = to_2df(&other.position);
        let other_velocity = to_2df(&other.velocity);

        let v_matrix: Matrix<FloatScalar, Const<2>, Const<2>, _> =
            Matrix::from_columns(&[my_velocity, -other_velocity]);
        let p_vector = other_position - my_position;

        let lu = v_matrix.lu();
        let intercept = lu.solve(&p_vector).and_then(|t| {
            let t0 = t[0];
            let t1 = t[1];
            if t0 > 0.0 && t1 > 0.0 {
                Some(my_position + my_velocity * t0)
            } else {
                None
            }
        });

        intercept
    }

    fn distance_path(&self, other: &HailStone) -> Option<FloatScalar> {
        let n = self.velocity.cross(&other.velocity);
        let diff = other.position - self.position;
        let d = n.dot(&diff).abs();
        let l = n.norm();

        if l == 0.0 {
            None
        } else {
            Some(d / l)
        }
    }
}

impl FromStr for HailStone {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" @ ");
        let position: Vec3D = parts
            .next()
            .and_then(|s| parse_vec3d(s).ok())
            .ok_or(AocError::ParseError)?;
        let velocity: Vec3D = parts
            .next()
            .and_then(|s| parse_vec3d(s).ok())
            .ok_or(AocError::ParseError)?;
        Ok(HailStone { position, velocity })
    }
}

fn parse_vec3d(s: &str) -> Result<Vec3D, AocError> {
    let mut parts = s.split(", ");
    let x = parts
        .next()
        .ok_or(AocError::ParseError)?
        .trim()
        .parse::<FloatScalar>()?;
    let y = parts
        .next()
        .ok_or(AocError::ParseError)?
        .trim()
        .parse::<FloatScalar>()?;
    let z = parts
        .next()
        .ok_or(AocError::ParseError)?
        .trim()
        .parse::<FloatScalar>()?;
    Ok(vector![x, y, z])
}

fn with_pairs<T>(stones: &[HailStone], f: &dyn Fn(&HailStone, &HailStone) -> T) -> Vec<T> {
    (0..stones.len() - 1)
        .flat_map(|i| (i + 1..stones.len()).map(move |j| f(&stones[i], &stones[j])))
        .collect::<Vec<T>>()
}

pub fn in_range_2d(mn: FloatScalar, mx: FloatScalar) -> impl Fn(&Vec2D) -> bool {
    move |v| (v.x >= mn && v.x <= mx) && (v.y >= mn && v.y <= mx)
}

pub fn pairs_in_range(stones: &[HailStone], range: impl Fn(&Vec2D) -> bool) -> usize {
    with_pairs(stones, &|s1, s2| s1.intersect_2d_in_future(s2))
        .iter()
        .filter_map(|i| *i)
        .filter(|p| range(p))
        .count()
}

struct Part2Problem {
    stones: Vec<HailStone>,
}

impl CostFunction for Part2Problem {
    type Output = FloatScalar;
    type Param = Vec<FloatScalar>;

    fn cost(&self, p: &Self::Param) -> Result<FloatScalar, argmin::core::Error> {
        let stone = HailStone {
            position: vector![p[0], p[1], p[2]],
            velocity: vector![p[3], p[4], p[5]],
        };
        let dist = self
            .stones
            .iter()
            .filter_map(|s| s.distance_path(&stone))
            .map(|d| d.powi(2))
            .sum();
        Ok(dist)
    }
}

impl Gradient for Part2Problem {
    type Param = Vec<FloatScalar>;
    type Gradient = Vec<FloatScalar>;

    fn gradient(&self, t: &Vec<f64>) -> Result<Vec<f64>, argmin::core::Error> {
        let mut grad = vec![0.0; t.len()];
        for i in 0..t.len() {
            let mut t1 = t.clone();
            let mut t2 = t.clone();
            t1[i] += 0.001;
            t2[i] -= 0.001;
            let c1 = self.cost(&t1)?;
            let c2 = self.cost(&t2)?;
            grad[i] = (c1 - c2) / 0.002;
        }

        Ok(grad)
    }
}

fn optimize_stone(stones: &[HailStone], initial: &[f64]) -> Result<Vec<FloatScalar>, AocError> {
    let mut problem = Part2Problem {
        stones: stones.to_vec(),
    };

    let line_search = argmin::solver::linesearch::MoreThuenteLineSearch::new();
    let solver = argmin::solver::gradientdescent::SteepestDescent::new(line_search);
    let cost = Part2Problem {
        stones: stones.to_vec(),
    };
    let res = Executor::new(cost, solver)
        .configure(|state| {
            state
                .param(initial.to_vec())
                .max_iters(1000)
                .target_cost(0.0)
        })
        .run()
        .map_err(|e| AocError::NoSolution)?;

    Ok(res.state().get_prev_best_param().unwrap().clone())
}

fn is_parallel(s1: &HailStone, s2: &HailStone) -> Option<(HailStone, HailStone)> {
    let v1 = s1.velocity;
    let v2 = s2.velocity;
    let n = v1.cross(&v2);
    let l = n.norm();
    if l < 0.1 {
        Some((s1.clone(), s2.clone()))
    } else {
        None
    }
}

pub fn find_parallel_stones(stones: &[HailStone]) -> Vec<(HailStone, HailStone)> {
    with_pairs(stones, &is_parallel)
        .iter()
        .filter_map(|i| *i)
        .collect::<Vec<(HailStone, HailStone)>>()
}

fn start_stone(stones: &[HailStone]) -> HailStone {
    let parallel = find_parallel_stones(stones);

    stones[0].clone()
}

pub fn find_same_start(stones: &[HailStone]) -> Vec<(HailStone, HailStone)> {
    with_pairs(stones, &|s1, s2| {
        if s1.position.x == s2.position.x
            || s1.position.y == s2.position.y
            || s1.position.z == s2.position.z
        {
            Some((s1.clone(), s2.clone()))
        } else {
            None
        }
    })
    .iter()
    .filter_map(|i| *i)
    .collect::<Vec<(HailStone, HailStone)>>()
}

pub fn find_invalid_v_x_ranges(stones: &[HailStone]) -> Vec<(FloatScalar, FloatScalar)> {
    with_pairs(stones, &|s1, s2| {
        if s1.position.x > s2.position.x && s1.velocity.x > s2.velocity.x {
            return Some((s2.velocity.x, s1.velocity.x));
        }
        if s1.position.x < s2.position.x && s1.velocity.x < s2.velocity.x {
            return Some((s1.velocity.x, s2.velocity.x));
        }
        None
    })
    .iter()
    .filter_map(|i| *i)
    .collect::<Vec<_>>()
}

pub fn find_invalid_v_z_ranges(stones: &[HailStone]) -> Vec<(FloatScalar, FloatScalar)> {
    with_pairs(stones, &|s1, s2| {
        if s1.position.z > s2.position.z && s1.velocity.z > s2.velocity.z {
            return Some((s2.velocity.z, s1.velocity.z));
        }
        if s1.position.z < s2.position.z && s1.velocity.z < s2.velocity.z {
            return Some((s1.velocity.z, s2.velocity.z));
        }
        None
    })
    .iter()
    .filter_map(|i| *i)
    .collect::<Vec<_>>()
}
#[cfg(test)]
mod tests {
    use nalgebra::vector;

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
        let actual = parse_vec3d("19, 13, 30").unwrap();
        let expected = vector![19.0, 13.0, 30.0,];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hailstone_from_str() {
        let actual = "19, 13, 30 @ -2,  1, -2".parse::<HailStone>().unwrap();
        let expected = HailStone {
            position: vector![19.0, 13.0, 30.0,],
            velocity: vector![-2.0, 1.0, -2.0],
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

    #[test]
    fn test_intersect() {
        let stones = test_input().stones;

        let i1_2 = stones[0].intersect_2d_in_future(&stones[1]);
        assert!(i1_2.is_some());

        let i2_3 = stones[0].intersect_2d_in_future(&stones[1]);
        assert!(i2_3.is_some());
    }

    #[test]
    fn test_pairs_in_range() {
        let stones = test_input().stones;
        let in_range = in_range_2d(7 as FloatScalar, 27 as FloatScalar);

        let c = pairs_in_range(&stones, in_range);
        assert_eq!(c, 2)
    }

    #[test]
    fn test_distance_path() {
        let stones = test_input().stones;
        let p0 = vector![24.0, 13.0, 10.0];
        let v0 = vector![-3.0, 1.0, 2.0];
        let stone = HailStone {
            position: p0,
            velocity: v0,
        };

        let stones = test_input().stones;
        let d1 = stone.distance_path(&stones[0]);

        for hs in stones {
            let d = stone.distance_path(&hs);
            assert_eq!(d, Some(0.0))
        }
    }

    #[test]
    fn test_optimize_stone() {
        let stones = test_input().stones;

        let initial = [20.0, 13.0, 10.0, -3.0, 1.0, 1.0];

        let params = optimize_stone(&stones[0..3], &initial);


        let params = params.unwrap();

        let expected = [24.0, 13.0, 10.0, -3.0, 1.0, 2.0];
        params.iter().zip(expected).for_each(|(a, e)| {
            assert!((a - e).abs() < 0.5)
        });
    }

    #[test]
    fn test_parallel_lines() {
        let stones = test_input().stones;

        let parallel = find_parallel_stones(&stones);

        assert_eq!(parallel, vec![(stones[1], stones[2])]);
    }

    #[test]
    fn test_find_same_start() {
        let stones = test_input().stones;

        let same_start = find_same_start(&stones);

        assert_eq!(same_start.len(), 17);
    }
}
