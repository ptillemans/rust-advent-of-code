use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub plays: Vec<(char, char)>,
}


#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines()
            .map(|line| {
                let mut chars = line.chars();
                let first = chars.next().ok_or(AocError::ParseError)?;
                let _ = chars.next().ok_or(AocError::ParseError)?;
                let second = chars.next().ok_or(AocError::ParseError)?;
                Ok((first, second))
            })
        .collect::<Result<Vec<_>, _>>()?;
        Ok(InputModel{plays: lines})
    }
}


pub fn score(player1: Play, player2: Play) -> i32 {
    let play_score = player2 as i32 + 1;
    let outcome = (3 + player2 as i32 - player1 as i32) % 3;
    let match_score = match outcome {
        0 => 3,
        1 => 6,
        _ => 0,
    };
    play_score + match_score
}

pub fn play_for_outcome(play: Play, outcome: Outcome) -> Play {
    let response = (2 + play as i32 + outcome as i32) % 3;
    println!("{} {} {}", play as i32, outcome as i32, response);
    match response {
        0 => Play::Rock,
        1 => Play::Paper,
        _ => Play::Scissors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_1() {
        let actual = score(Play::Rock, Play::Paper);
        assert_eq!(actual, 8);
    }

    #[test]
    fn test_score_2() {
        let actual = score(Play::Paper, Play::Rock);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_score_3() {
        let actual = score(Play::Scissors, Play::Scissors);
        assert_eq!(actual, 6);
    }

    #[test]
    fn test_play_for_outcome_1() {
        let actual = play_for_outcome(Play::Rock, Outcome::Draw);
        assert_eq!(actual, Play::Rock);
    }

    #[test]
    fn test_play_for_outcome_2() {
        let actual = play_for_outcome(Play::Paper, Outcome::Lose);
        assert_eq!(actual, Play::Rock);
    }

    #[test]
    fn test_play_for_outcome_3() {
        let actual = play_for_outcome(Play::Scissors, Outcome::Win);
        assert_eq!(actual, Play::Rock);
    }
}
