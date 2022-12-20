use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub coordinates: Vec<i32>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| line.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map(|coordinates| InputModel { coordinates })
            .map_err(|_| AocError::ParseError)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CodeRing {
    code: Vec<(usize, i32)>,
}

impl CodeRing {
   
    pub fn new(code: Vec<i32>) -> Self {
        let code = code
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>();
        CodeRing { code }
    }

    pub fn decode(&mut self) -> i32 {
        let original = self.code.clone();
        for (orig_pos, value) in original.iter() {
            let from_idx = self.code.iter().position(|(pos, _)| pos == orig_pos).unwrap();
            let to_idx = ((from_idx as i32 + value).rem_euclid((self.code.len() - 1) as i32)) as usize;
            let t = self.code.remove(from_idx);
            self.code.insert(to_idx, t);
        };
        let offset = self.code.iter().position(|(_, v)| *v == 0).unwrap();
        
        (1000..=3000).step_by(1000)
            .map(|i| (i + offset) % self.code.len())
            .map(|i| self.code[i].1)
            .sum()
        
    }


}



pub const TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

pub fn input_data() -> InputModel {
    InputModel {
        coordinates: vec![
            1,
            2,
            -3,
            3,
            -2,
            0,
            4,
        ],
    }
}

#[cfg(test)]
mod tests {


    const SEQUENCE: [[i32; 7]; 7] = [
        [2, 1, -3, 3, -2, 0, 4],
        [1, -3, 2, 3, -2, 0, 4],
        [1, 2, 3, -2, -3, 0, 4],
        [1, 2, -2, -3, 0, 3, 4],
        [1, 2, -3, 0, 3, 4, -2],
        [1, 2, -3, 0, 3, 4, -2],
        [1, 2, -3, 4, 0, 3, -2],
    ];

    #[test]
    fn test_parse() {
        let actual = super::TEST_INPUT.parse::<super::InputModel>().unwrap();
        let expected = super::input_data();

        assert_eq!(actual, expected);
    }

    //#[test]
    fn test_sum_coordinates() {
        let input = super::input_data();
        let mut code_ring = super::CodeRing::new(input.coordinates);
        assert_eq!(code_ring.decode(), 2);
    }

}
