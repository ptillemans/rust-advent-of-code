use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub trees: Vec<Vec<char>>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution found")]
    NoSolution,
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

pub fn bounds(trees: &Vec<Vec<char>>) -> (usize, usize) {
    let l = trees.len();
    let w = trees[0].len();
    (l, w)
}

pub fn find_visible(trees: &Vec<Vec<char>>) -> Vec<Vec<bool>> {

    let (l, w) = bounds(trees);

    let mut visible = vec![vec![false; w]; l];

    for i in 0..l {
        visible[i][0] = true;
        let mut max_height = trees[i][0];
        for j in 1..w {
            if trees[i][j] > max_height {
                visible[i][j] = true;
            }
            max_height = max(max_height, trees[i][j]);
        }

        visible[i][w-1] = true; 
        let mut max_height = trees[i][w-1];
        for j in (0..w-1).rev() {
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
            if trees[i][j] > max_height {
                visible[i][j] = true;
            }
            max_height = max(max_height, trees[i][j])
        }
        visible[l-1][j] = true; 
        let mut max_height = trees[l-1][j];
        for i in (0..l-1).rev() {
            if trees[i][j] > max_height {
                visible[i][j] = true;
            } 
            max_height = max(max_height, trees[i][j])
        }
    };
    visible
}

pub fn scenic_score(trees: &Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    let (l, w) = bounds(trees);
    let (x, y) = pos;
    let height = trees[x][y];
    let mut north_count = 0;
    let mut south_count = 0;
    let mut east_count = 0;
    let mut west_count = 0;
    for j in (0..y).rev() {
        north_count += 1;
        if height <= trees[x][j] {
            break;
        }
    }
    for j in y+1..w {
        south_count += 1;
        if height <= trees[x][j] {
            break;
        }
    }
    for i in x+1..l {
        east_count += 1;
        if height <= trees[i][y] {
            break;
        }
    }
    for i in (0..x).rev() {
        west_count += 1;
        if height <= trees[i][y] {
            break;
        }
    }
    let score = north_count * south_count * east_count * west_count;
    score
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

    #[test]
    pub fn test_scenic_score_1_2() {
        let actual = scenic_score(&input_data().trees, (1,2));
        let expected = 4;
        assert_eq!(actual, expected)
    }

    #[test]
    pub fn test_scenic_score_3_2() {
        let actual = scenic_score(&input_data().trees, (3,2));
        let expected = 8;
        assert_eq!(actual, expected)
    }

}
