use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub trees: Vec<Vec<char>>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s.lines().map(|line| line.chars().collect()).collect();
        Ok(InputModel { trees })
    }
}


fn max<T:Ord> (a: T, b:T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn find_visible(trees: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let l = trees.len();
    let w = trees[0].len();

    let mut visible = vec![vec![false; w]; l];

    for i in 0..l {
        visible[i][0] = true;
        let mut max_height = trees[i][0];
        for j in 1..w {
            println!("({}.{})  {} > {}", i, j,  trees[i][j], max_height);
            if trees[i][j] > max_height {
                visible[i][j] = true;
            }
            max_height = max(max_height, trees[i][j]);
        }

        visible[i][w-1] = true; 
        let mut max_height = trees[i][w-1];
        for j in (0..w-1).rev() {
            println!("({}.{})  {} < {}", i, j,  trees[i][j], max_height);
            if trees[i][j] > max_height {
                visible[i][j] = true;
            }
            max_height = max(max_height, trees[i][j]);
        }
    }
    for j in 0..w {
        visible[0][j] = true; 
        let mut max_height = trees[0][j];
        for i in 1..l {
            println!("({}.{})  {} v {}", i, j,  trees[i][j], max_height);
            if trees[i][j] > max_height {
                visible[i][j] = true;
            }
            max_height = max(max_height, trees[i][j])
        }
        visible[l-1][j] = true; 
        let mut max_height = trees[l-1][j];
        for i in (0..l-1).rev() {
            println!("({}.{})  {} ^ {}", i, j,  trees[i][j], max_height);
            if trees[i][j] > max_height {
                visible[i][j] = true;
            } 
            max_height = max(max_height, trees[i][j])
        }
    };
    visible
}

#[cfg(test)]
mod tests {

    use super::*;

    fn input_data() -> InputModel {
        InputModel {
            trees: vec![
                vec!['3', '0', '3', '7', '3', ],
                vec!['2', '5', '5', '1', '2', ],
                vec!['6', '5', '3', '3', '2', ],
                vec!['3', '3', '5', '4', '9', ],
                vec!['3', '5', '3', '9', '0', ],
            ],
        }
    }

    #[test]
    pub fn test_find_visible() {
        let actual = find_visible(&input_data().trees);
        let expected = vec![
            vec![  true,  true,  true,  true,  true, ], 
            vec![  true,  true,  true, false,  true, ], 
            vec![  true,  true, false,  true,  true, ], 
            vec![  true, false,  true, false,  true, ], 
            vec![  true,  true,  true,  true,  true, ], 
        ];
        assert_eq!(actual, expected)
    }
}
