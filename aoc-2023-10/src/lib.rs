use std::str::FromStr;
use itertools::iterate;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub pipes: Vec<Pipe>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Pipe {
    pub loc: (i64, i64),
    pub section: Section,
}

impl Pipe {
    pub fn valid_next_pipe(&self, next: &Pipe) -> bool {
        if next.section == Section::Start {
            return true;
        }

        let diff = (self.loc.0 - next.loc.0, self.loc.1 - next.loc.1);
        match diff {
            (0, -1) => next.section == Section::Vertical || next.section == Section::CornerNE || next.section == Section::CornerNW,
            (0, 1) => next.section == Section::Vertical || next.section == Section::CornerSE || next.section == Section::CornerSW,
            (1, 0) => next.section == Section::Horizontal || next.section == Section::CornerNE || next.section == Section::CornerSE,
            (-1, 0) => next.section == Section::Horizontal || next.section == Section::CornerNW || next.section == Section::CornerSW,
            _ => false,
        }
    }

    pub fn connecting_pipes(&self, pipes: &[Pipe]) -> Vec<Pipe> {
        let (x, y) = self.loc;
        let mut locations = vec![];
        match self.section {
            Section::Start => {
                locations.push((x, y + 1));
                locations.push((x, y - 1));
                locations.push((x + 1, y));
                locations.push((x - 1, y));
            }
            Section::Horizontal => {
                locations.push((x + 1, y));
                locations.push((x - 1, y));
            }
            Section::Vertical => {
                locations.push((x, y + 1));
                locations.push((x, y - 1));
            }
            Section::CornerNE => {
                locations.push((x, y - 1));
                locations.push((x + 1, y));
            }
            Section::CornerNW => {
                locations.push((x, y - 1));
                locations.push((x - 1, y));
            }
            Section::CornerSE => {
                locations.push((x, y + 1));
                locations.push((x + 1, y));
            }
            Section::CornerSW => {
                locations.push((x, y + 1));
                locations.push((x - 1, y));
            }
        }
        locations
            .iter()
            .filter_map(|(x, y)| pipes.iter().find(|p| p.loc == (*x, *y)))
            .filter(|p| self.valid_next_pipe(*p))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn find_path(&self, pipes: &[Pipe], prev_pipe: &Pipe) -> Option<Vec<Pipe>> {
        iterate((*prev_pipe, *self, vec![*self]), |(last, this,  path)| {
            
            let next_pipes = this.connecting_pipes(&pipes);
            let next_pipe = next_pipes
                .iter()
                .filter(|&p| *p != *last && !path.contains(p))
                .cloned()
                .next();
            let mut path =path.clone();
            if let Some(next_pipe) = next_pipe  {
                if !path.contains(&next_pipe) {
                    path.push(next_pipe);
                    (*this, next_pipe, path)
                } else {
                    (*this, *this, path)
                }
            } else {
                (*this, *this, path)
            }
        })
            .find(|(last, this, path)| this.section == Section::Start || *last == *this)
            .map(|(_, _, path)| path)
    }


}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Section {
    Start,
    Horizontal,
    Vertical,
    CornerNE,
    CornerNW,
    CornerSE,
    CornerSW,
}

impl FromStr for Section {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Section::Start),
            "-" => Ok(Section::Horizontal),
            "|" => Ok(Section::Vertical),
            "L" => Ok(Section::CornerNE),
            "J" => Ok(Section::CornerNW),
            "F" => Ok(Section::CornerSE),
            "7" => Ok(Section::CornerSW),
            _ => Err(AocError::ParseError("Invalid section".to_string())),
        }
    }
}

impl Section {}
#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError(String),
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(|(x, c)| {
                        let loc = (x as i64, y as i64);
                        let section = Section::from_str(&c.to_string())?;
                        Ok(Pipe { loc, section })
                    })
                    .collect::<Result<Vec<Pipe>, AocError>>()
            })
            .collect::<Result<Vec<Vec<Pipe>>, AocError>>()
            .map(|pipes| InputModel {
                pipes: pipes.into_iter().flatten().collect(),
            })
    }
}

pub fn find_start(pipes: &[Pipe]) -> Pipe {
    pipes
        .iter()
        .find(|p| p.section == Section::Start)
        .cloned()
        .unwrap()
}

pub fn find_loop(pipes: &[Pipe]) -> Option<Vec<Pipe>> {
    let start = find_start(pipes);
    start.connecting_pipes(pipes).iter()
        .find_map(|p| start.find_path(pipes, p))
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    const TEXT_INPUT: &str = "
.S-7.
.|.|.
.L-J.
.....";

    const TEXT_INPUT_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";


    const TEST_INPUT_2_2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn test_parse() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;

        let expected = vec![
            Pipe {
                loc: (1, 1),
                section: Section::Start,
            },
            Pipe {
                loc: (2, 1),
                section: Section::Horizontal,
            },
            Pipe {
                loc: (3, 1),
                section: Section::CornerSW,
            },
            Pipe {
                loc: (1, 2),
                section: Section::Vertical,
            },
            Pipe {
                loc: (3, 2),
                section: Section::Vertical,
            },
            Pipe {
                loc: (1, 3),
                section: Section::CornerNE,
            },
            Pipe {
                loc: (2, 3),
                section: Section::Horizontal,
            },
            Pipe {
                loc: (3, 3),
                section: Section::CornerNW,
            },
        ];
        assert_eq!(pipes, expected);
    }

    #[test]
    fn test_connecting_sections() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;

        let mut cs = pipes[0].connecting_pipes(&pipes);
        cs.sort();
        assert_eq!(cs, vec![pipes[3], pipes[1]]);

        let mut cs = pipes[1].connecting_pipes(&pipes);
        cs.sort();
        assert_eq!(cs, vec![pipes[0], pipes[2]]);

        let mut cs = pipes[2].connecting_pipes(&pipes);
        cs.sort();
        assert_eq!(cs, vec![pipes[1], pipes[4]]);

        let mut cs = pipes[3].connecting_pipes(&pipes);
        cs.sort();
        assert_eq!(cs, vec![pipes[0], pipes[5]]);

        let mut cs = pipes[4].connecting_pipes(&pipes);
        cs.sort();
        assert_eq!(cs, vec![pipes[2], pipes[7]]);

        let mut cs = pipes[5].connecting_pipes(&pipes);
        cs.sort();
        assert_eq!(cs, vec![pipes[3], pipes[6]]);

        let mut cs = pipes[6].connecting_pipes(&pipes);
        cs.sort();
        assert_eq!(cs, vec![pipes[5], pipes[7]]);

        let mut cs = pipes[7].connecting_pipes(&pipes);
        cs.sort();
        assert_eq!(cs, vec![pipes[6], pipes[4]]);
    }

    #[test]
    fn test_find_start() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;
        let start = find_start(&pipes);
        assert_eq!(start.loc, (1, 1));
    }

    #[test]
    fn test_valid_next_pipe() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;
        assert!(pipes[0].valid_next_pipe(&pipes[1]));
        assert!(pipes[0].valid_next_pipe(&pipes[3]));

        assert!(pipes[1].valid_next_pipe(&pipes[0]));
        assert!(pipes[1].valid_next_pipe(&pipes[2]));

        assert!(pipes[2].valid_next_pipe(&pipes[1]));
        assert!(pipes[2].valid_next_pipe(&pipes[4]));

        assert!(pipes[3].valid_next_pipe(&pipes[0]));
        assert!(pipes[3].valid_next_pipe(&pipes[5]));

        assert!(pipes[4].valid_next_pipe(&pipes[2]));
        assert!(pipes[4].valid_next_pipe(&pipes[7]));

        assert!(pipes[5].valid_next_pipe(&pipes[3]));

        assert!(pipes[6].valid_next_pipe(&pipes[5]));
        assert!(pipes[6].valid_next_pipe(&pipes[7]));

        assert!(pipes[7].valid_next_pipe(&pipes[6]));
        assert!(pipes[7].valid_next_pipe(&pipes[4]));
        
    }
    
    #[test]
    fn test_find_path() {
        let input = InputModel::from_str(TEXT_INPUT).unwrap();
        let pipes = input.pipes;
        let path = pipes[1].find_path(&pipes, &pipes[0]);
        assert_eq!(
            path,
            Some(vec![pipes[1], pipes[2], pipes[4], pipes[7], pipes[6], pipes[5], pipes[3], pipes[0]])
        );


        let input = InputModel::from_str(TEXT_INPUT_2).unwrap();
        let pipes = input.pipes;
        let start = find_start(&pipes);
        let next_pipes = start.connecting_pipes(&pipes);
        println!("{:?}", next_pipes);
        assert_eq!(next_pipes.len(), 2);
        let path = next_pipes[0].find_path(&pipes, &start).unwrap();
        assert_eq!(path.len(), 16);
        let path = next_pipes[1].find_path(&pipes, &start).unwrap();
        assert_eq!(path.len(), 16);


        let input = InputModel::from_str(TEXT_INPUT_2).unwrap();
        let pipes = input.pipes;
        let start = find_start(&pipes);
        let next_pipes = start.connecting_pipes(&pipes);
        println!("{:?}", next_pipes);
        assert_eq!(next_pipes.len(), 2);
        let path = next_pipes[0].find_path(&pipes, &start).unwrap();
        assert_eq!(path.len(), 16);
        let path = next_pipes[1].find_path(&pipes, &start).unwrap();
        assert_eq!(path.len(), 16);
    }
}
