use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub assignments: Vec<((u32, u32), (u32, u32))>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let assignments = s
            .lines()
            .map(parse_line)
            .collect();
        Ok(InputModel { assignments })
    }
}

fn parse_pair(s: &str) -> (u32, u32) {
    let mut parts = s.split('-');
    let first = parts.next().unwrap().parse::<u32>().unwrap();
    let second = parts.next().unwrap().parse::<u32>().unwrap();
    (first, second)
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let mut parts = line.split(',');
    let first = parts.next().unwrap();
    let second = parts.next().unwrap();
    (parse_pair(first), parse_pair(second))
}

pub fn contains(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1 || a.0 >= b.0 && a.1 <= b.1
}

pub fn overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    !(a.1 < b.0 || a.0 > b.1)
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8";

    #[test]
    fn test_parse_line() {
        let actual = parse_line(TEST_INPUT);
        let expected = ((2, 4), (6, 8));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_contains() {
        let examples = vec![
            ((2, 4), (6, 8), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), false),
            ((2, 8), (3, 7), true),
            ((6, 6), (4, 6), true),
            ((2, 6), (4, 8), false),
        ];
        for example in examples { 
            let actual = contains(example.0, example.1);
            let expected = example.2;
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_overlap() {
        let examples = vec![
            ((2, 4), (6, 8), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), true),
            ((2, 8), (3, 7), true),
            ((6, 6), (4, 6), true),
            ((2, 6), (4, 8), true),

        ];
        for example in examples { 
            let actual = overlap(example.0, example.1);
            let expected = example.2;
            println!("{:?} {:?} {} {}", example.0, example.1, actual, expected);
            assert_eq!(actual, expected);
        }
    }

}
