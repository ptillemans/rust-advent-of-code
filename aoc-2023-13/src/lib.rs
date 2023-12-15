use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub mirrors: Vec<Vec<String>>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let mirrors = lines
            .split(|l| l.is_empty())
            .to_owned()
            .map(|m| m.to_vec())
            .collect::<Vec<Vec<_>>>();
        Ok(InputModel { mirrors })
    }
}

fn is_mirror_image(s1: &str, s2: &str) -> bool {
    let l = if s1.len() < s2.len() {
        s1.len()
    } else {
        s2.len()
    };

    s1.chars()
        .rev()
        .take(l)
        .zip(s2.chars().take(l))
        .all(|(c1, c2)| c1 == c2)
}

pub fn find_horizontal_mirror(mirror: &[String]) -> Option<usize> {
    (1..mirror[0].len()).find(|i| {
        mirror
            .iter()
            .map(|l| l.split_at(*i))
            .all(|(s1, s2)| is_mirror_image(s1, s2))
    })
}

pub fn find_vertical_mirror(mirror: &[String]) -> Option<usize> {
    let cols = (0..mirror[0].len())
        .map(|i| {
            mirror
                .iter()
                .map(|l| l.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    find_horizontal_mirror(&cols)
}

pub fn find_horizontal_smudge(mirror: &[String]) -> Option<usize> {
    (1..mirror[0].len()).find(|i| {
        mirror
            .iter()
            .map(|l| l.split_at(*i))
            .filter(|(s1, s2)| !is_mirror_image(s1, s2))
            .count() == 1})
}

pub fn find_vertical_smudge(mirror: &[String]) -> Option<usize> {
    let cols = (0..mirror[0].len())
        .map(|i| {
            mirror
                .iter()
                .map(|l| l.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    find_horizontal_smudge(&cols)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    fn input_data() -> InputModel {
        TEST_INPUT.parse::<InputModel>().unwrap()
    }

    #[test]
    fn test_parse() {
        let input = "line1\nline2\nline3\n\nline4\nline5";
        let expected = InputModel {
            mirrors: vec![
                vec![
                    "line1".to_string(),
                    "line2".to_string(),
                    "line3".to_string(),
                ],
                vec!["line4".to_string(), "line5".to_string()],
            ],
        };
        assert_eq!(expected, input.parse().unwrap());
    }

    #[test]
    fn test_find_horizontal_mirror() {
        let input = input_data().mirrors;
        let expected = [Some(5), None];

        let actual = input
            .iter()
            .map(|m| find_horizontal_mirror(m))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_vertical_mirror() {
        let input = input_data().mirrors;
        let expected = [None, Some(4)];

        let actual = input
            .iter()
            .map(|m| find_vertical_mirror(m))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_horizontal_smudge() {
        let input = input_data().mirrors;
        let expected = [None, None];

        let actual = input
            .iter()
            .map(|m| find_horizontal_smudge(m))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_vertical_smudge() {
        let input = input_data().mirrors;
        let expected = [Some(3), Some(1)];

        let actual = input
            .iter()
            .map(|m| find_vertical_smudge(m))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}
