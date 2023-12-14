use std::{str::FromStr, collections::HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub platform: Vec<Vec<char>>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let platform = s
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(InputModel { platform })
    }
}

pub fn slide_north(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = platform[0].len();
    let height = platform.len();

    let mut new_platform = vec![vec!['.'; width]; height];

    for c in 0..width {
        let mut stop_height = 0;
        for r in 0..height {
            match platform[r][c] {
                '#' => {
                    new_platform[r][c] = '#';
                    stop_height = r + 1;
                }
                'O' => {
                    new_platform[r][c] = '.';
                    new_platform[stop_height][c] = 'O';
                    stop_height += 1;
                }
                x => new_platform[r][c] = x,
            }
        }
    }
    new_platform
}

pub fn slide_east(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = platform[0].len();
    let height = platform.len();

    let mut new_platform = vec![vec!['.'; width]; height];

    for r in 0..height {
        let mut stop_col = width - 1;
        for ct in 0..width {
            let c = width - ct - 1;
            match platform[r][c] {
                '#' => {
                    new_platform[r][c] = '#';
                    stop_col = if c > 0 { c - 1 } else { 0 };
                }
                'O' => {
                    new_platform[r][c] = '.';
                    new_platform[r][stop_col] = 'O';
                    if stop_col > 0 {
                        stop_col -= 1;
                    }
                }
                x => new_platform[r][c] = x,
            }
        }
    }
    new_platform
}

pub fn slide_south(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = platform[0].len();
    let height = platform.len();

    let mut new_platform = vec![vec!['.'; width]; height];

    for c in 0..width {
        let mut stop_height = height - 1;
        for rt in 0..height {
            let r = height - rt - 1;
            match platform[r][c] {
                '#' => {
                    new_platform[r][c] = '#';
                    stop_height = if r > 0 { r - 1 } else { 0 };
                }
                'O' => {
                    new_platform[r][c] = '.';
                    new_platform[stop_height][c] = 'O';
                    if stop_height > 0 {
                        stop_height -= 1
                    }
                }
                x => new_platform[r][c] = x,
            }
        }
    }
    new_platform
}

pub fn slide_west(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = platform[0].len();
    let height = platform.len();

    let mut new_platform = vec![vec!['.'; width]; height];

    for r in 0..height {
        let mut stop_col = 0;
        for c in 0..width {
            match platform[r][c] {
                '#' => {
                    new_platform[r][c] = '#';
                    stop_col = c + 1;
                }
                'O' => {
                    new_platform[r][c] = '.';
                    new_platform[r][stop_col] = 'O';
                    stop_col += 1;
                }
                x => new_platform[r][c] = x,
            }
        }
    }
    new_platform
}
pub fn load_north(platform: &Vec<Vec<char>>) -> u64 {
    let width = platform[0].len();
    let height = platform.len();

    let mut score: u64 = 0;
    for c in 0..width {
        for r in 0..height {
            match platform[r][c] {
                'O' => {
                    let load = (height - r) as u64;

                    score += load;
                }
                _ => {}
            }
        }
    }
    score
}

pub fn spin_cycle(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let platform = slide_north(&platform);
    let platform = slide_west(&platform);
    let platform = slide_south(&platform);
    let platform = slide_east(&platform);
    platform
}

pub fn print_platform(platform: &Vec<Vec<char>>) {
    for line in platform {
        println!("{}", line.iter().collect::<String>());
    }
    println!("=====");
}

pub fn spin_load(platform: &Vec<Vec<char>>, n: usize) -> u64 {
    let mut platforms = Vec::with_capacity(10000);
    let mut platform = platform.clone();

    while !platforms.contains(&platform) {
        platforms.push(platform.clone());
        platform = spin_cycle(&platform);
    }

    let start = platforms.iter().position(|p| *p == platform).unwrap();
    let cycle_len = platforms.len() - start;

    let p_n = start + (n - start) % cycle_len;
    load_north(&platforms[p_n as usize])
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const TEST_OUTPUT: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    const TEST_SPIN: [&str; 3] = [
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
    ];

    pub fn input_data() -> InputModel {
        TEST_INPUT.parse::<InputModel>().unwrap()
    }
    #[test]
    fn test_from_str() {
        let input = "#.\n#O\n";
        let model = InputModel::from_str(input).unwrap();
        assert_eq!(model.platform, vec![vec!['#', '.'], vec!['#', 'O']]);
    }

    #[test]
    fn test_slide() {
        let input = "#.\n#O\n".parse::<InputModel>().unwrap();
        let actual = slide_north(&input.platform);
        let expected = vec![vec!['#', 'O'], vec!['#', '.']];
        assert_eq!(actual, expected);

        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let actual = slide_north(&input.platform);
        let expected = TEST_OUTPUT.parse::<InputModel>().unwrap().platform;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_load() {
        let input = TEST_OUTPUT.parse::<InputModel>().unwrap();
        let actual = load_north(&input.platform);
        let expected = 136;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_spin() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let actual = spin_cycle(&input.platform);
        let expected = TEST_SPIN[0].parse::<InputModel>().unwrap().platform;
        assert_eq!(actual, expected);

        let actual = spin_cycle(&actual);
        let expected = TEST_SPIN[1].parse::<InputModel>().unwrap().platform;
        assert_eq!(actual, expected);

        let actual = spin_cycle(&actual);
        let expected = TEST_SPIN[2].parse::<InputModel>().unwrap().platform;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_spin_load() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let actual = spin_load(&input.platform, 1000000000);
        let expected = 64;
        assert_eq!(actual, expected);
    }
}
