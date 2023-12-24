#![feature(test)]
use aoc_2023_24::{
    find_invalid_v_x_ranges, find_invalid_v_z_ranges, find_parallel_stones, find_same_start,
    in_range_2d, pairs_in_range, AocError, InputModel,
};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let stones = input.stones.clone();
    let range = in_range_2d(2.0E14 as f64, 4.0E14 as f64);
    let pairs = pairs_in_range(&stones, range);
    Ok(pairs.to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let same_start = find_same_start(&input.stones);
    println!("same_start: {:?}", same_start);

    let y0 = same_start[0].0.position.y;
    let v_y0 = same_start[0].0.velocity.y;
    println!("y0: {}, v_y0: {}", y0, v_y0);

    // intercept with stone 0
    let stone = &input.stones[0];
    println!("stone: {:?}", stone);
    // y0 + v_y0 * t = stone.position.y + stone.velocity.y * t
    // t = (y0 - stone.position.y) / (stone.velocity.y - v_y0)
    let t1 = (y0 - stone.position.y) as f64 / (stone.velocity.y - v_y0) as f64;
    let x0_t1 = stone.position.x as f64 + stone.velocity.x as f64 * t1;
    let z0_t1 = stone.position.z as f64 + stone.velocity.z as f64 * t1;
    println!("t1: {}, x0_t1: {}, z0_t1: {}", t1, x0_t1, z0_t1);

    // intercept with stone 1
    let stone = &input.stones[1];
    let t2 = (y0 - stone.position.y) as f64 / (stone.velocity.y - v_y0 as f64);
    let x0_t2 = stone.position.x as f64 + stone.velocity.x as f64 * t2;
    let z0_t2 = stone.position.z as f64 + stone.velocity.z as f64 * t2;
    println!("t2: {}, x0_t2: {}, z0_t2: {}", t2, x0_t2, z0_t2);

    let v_x0 = (x0_t1 - x0_t2) / (t1 - t2);
    let v_z0 = (z0_t1 - z0_t2) / (t1 - t2);
    println!("v_x0: {}, v_y0: {}, v_z0: {}", v_x0, v_y0, v_z0);

    let x0 = x0_t1 - v_x0 * t1;
    let z0 = z0_t1 - v_z0 * t1;

    println!("x0: {},  y0: {}, z0: {}", x0, y0, z0);

    Ok((x0 + y0 + z0).to_string())
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

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    pub fn input_data() -> InputModel {
        TEST_INPUT.parse::<InputModel>().unwrap()
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
        assert_eq!(actual.stones.len(), 5);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "0";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

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
