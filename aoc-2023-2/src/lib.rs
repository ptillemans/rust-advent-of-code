use std::{str::FromStr, cmp::max};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub games: Vec<Game>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

fn parse(s: &str) -> Result<InputModel, AocError> {
    s.lines()
        .map(|l| parse_line(l))
        .collect::<Result<Vec<Game>, AocError>>()
        .map(|games| InputModel { games })
}

fn parse_line(s: &str) -> Result<Game, AocError> {
    let mut parts = s.split(":");
    let game_number: u32= parts.next()
        .ok_or(AocError::ParseError)
        .and_then(|s| s.trim().strip_prefix("Game ").ok_or(AocError::ParseError))
        .map(|s| s.trim_end_matches(":"))
        .and_then(|s| s.parse::<u32>().map_err(|_| AocError::ParseError))?;

    parts.next().ok_or(AocError::ParseError)
        .and_then(|s| s.split(";")
                .map(|s| parse_showing(s))
                .collect::<Result<Vec<Showing>, AocError>>())
        .map(|showings| Game { game_number, showings })
}

fn parse_showing(s: &str) -> Result<Showing, AocError> {
    let parts = s.split(",");
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for part in parts {
        let mut part = part.trim().split(" ");
        let count = part.next().unwrap().parse::<u32>().unwrap();
        let color = part.next().unwrap();
        match color {
            "red" => red = count,
            "green" => green = count,
            "blue" => blue = count,
            _ => return Err(AocError::ParseError),
        }
    }
    Ok(Showing { red, green, blue })
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub game_number: u32,
    pub showings: Vec<Showing>,
}

impl Game {
    pub fn is_valid(&self, bag: &Bag) -> bool {
        self.showings.iter().all(|s| s.is_valid(bag))
    }

    pub fn minimal_bag(&self) -> Bag {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for showing in &self.showings {
            if showing.red > red {
                red = showing.red;
            }
            if showing.green > green {
                green = showing.green;
            }
            if showing.blue > blue {
                blue = showing.blue;
            }
        }
        Bag::new(red, green, blue)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Showing {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Showing {
    pub fn is_valid(&self, bag: &Bag) -> bool {
        self.red <= bag.red
            && self.green <= bag.green
            && self.blue <= bag.blue
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bag {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Bag {
    pub fn new(red: u32, green: u32, blue: u32) -> Bag {
        Bag { red, green, blue }
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    pub fn union(&self, other: &Bag) -> Bag {
        Bag {
            red: max(self.red,other.red),
            green: max(self.green, other.green),
            blue: max(self.blue,other.blue),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_showing() {
        let input = "3 blue, 4 red";
        let showing = parse_showing(input).unwrap();
        let expected = Showing {
            red: 4,
            green: 0,
            blue: 3,
        };
        assert_eq!(showing, expected);
    }

    #[test]
    fn test_parse() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
";
        let model: InputModel = input.parse().unwrap();
        let expected = InputModel {
            games: vec![
                Game {
                    game_number: 1,
                    showings: vec![
                        Showing {
                            red: 4,
                            green: 0,
                            blue: 3,
                        },
                        Showing {
                            red: 1,
                            green: 2,
                            blue: 6,
                        },
                        Showing {
                            red: 0,
                            green: 2,
                            blue: 0,
                        },]
                },
                Game {
                    game_number: 2,
                    showings: vec![
                        Showing {
                            red: 0,
                            green: 2,
                            blue: 1,
                        },
                        Showing {
                            red: 1,
                            green: 3,
                            blue: 4,
                        },
                        Showing {
                            red: 0,
                            green: 1,
                            blue: 1,
                        },]
                    
                }
            ],
        };
        model.games.iter().zip(expected.games.iter()).for_each(|(a, b)| {
            assert_eq!(a, b);
        });
        assert_eq!(model, expected);
    }

    #[test]
    fn test_showing_is_valid() {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        let showing1 = Showing {
            red: 4,
            green: 0,
            blue: 3,
        };
        let showing2 = Showing {
            red: 1,
            green: 2,
            blue: 16,
        };
        assert!(showing1.is_valid(&bag));
        assert!(!showing2.is_valid(&bag));

    }

    #[test]
    fn test_game_is_valid() {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        let game1 = Game {
            game_number: 1,
            showings: vec![
                Showing {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Showing {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Showing {
                    red: 0,
                    green: 2,
                    blue: 0,
                },]
        };
        let game2 = Game {
            game_number: 2,
            showings: vec![
                Showing {
                    red: 0,
                    green: 2,
                    blue: 1,
                },
                Showing {
                    red: 1,
                    green: 3,
                    blue: 4,
                },
                Showing {
                    red: 20,
                    green: 1,
                    blue: 1,
                },]
            
        };
        assert!(game1.is_valid(&bag));
        assert!(!game2.is_valid(&bag));
    }

    #[test]
    fn test_bag_power() {
        let bag = Bag::new(4, 2, 6);
        assert_eq!(bag.power(), 48);
    }

    #[test]
    fn test_game_minimal_bag() {
        let game = Game {
            game_number: 1,
            showings: vec![
                Showing {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Showing {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Showing {
                    red: 0,
                    green: 2,
                    blue: 0,
                },]
        };
        let minimal_bag = game.minimal_bag();
        let expected_bag = Bag::new(4, 2, 6);
        assert_eq!(minimal_bag, expected_bag);
    }

    #[test]
    fn test_bag_union() {
        let bag1 = Bag::new(4, 2, 6);
        let bag2 = Bag::new(1, 3, 4);
        let union = bag1.union(&bag2);
        let expected_bag = Bag::new(4, 3, 6);
        assert_eq!(union, expected_bag);
    }
}
