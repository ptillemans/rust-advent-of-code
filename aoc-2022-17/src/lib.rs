#![feature(result_option_inspect)]
use std::str::FromStr;
use aoc_common::position::Position;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub moves: Vec<Jet>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim().chars()
            .map(|c| match c {
                '<' => Ok(Jet::Left),
                '>' => Ok(Jet::Right),
                _ => Err(AocError::ParseError),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|moves| InputModel { moves })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Jet {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Bar,
    Plus,
    Hook,
    Pole,
    Square,
}

impl Shape {
    const VALUES: [Shape; 5] = [
        Shape::Bar, 
        Shape::Plus, 
        Shape::Hook, 
        Shape::Pole, 
        Shape::Square
    ];

    fn size(&self) -> (i32, i32) {
        match self {
            Shape::Bar => (4, 1),
            Shape::Plus => (3, 3),
            Shape::Hook => (3, 3),
            Shape::Pole => (1, 4),
            Shape::Square => (2, 2),
        }
    }

    // give relative positions of the shape starting from the top left corner
    fn shape(&self) -> Vec<Position> {
        match self {
            Shape::Bar => vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(3, 0),
            ],
            Shape::Plus => vec![
                Position::new(1, 0),
                Position::new(0, -1),
                Position::new(1, -1),
                Position::new(2, -1),
                Position::new(1, -2),
            ],
            Shape::Hook => vec![
                Position::new(2, 0),
                Position::new(2, -1),
                Position::new(0, -2),
                Position::new(1, -2),
                Position::new(2, -2),
            ],
            Shape::Pole => vec![
                Position::new(0, 0),
                Position::new(0, -1),
                Position::new(0, -2),
                Position::new(0, -3),
            ],
            Shape::Square => vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(0, -1),
                Position::new(1, -1),
            ],
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Block {
    position: Position,
    shape: Shape,
    stuck: bool,
}

impl Block {
    fn new(position: Position, shape: Shape) -> Self {
        Self { position, shape, stuck: false }
    }
    
    // return top left corner and bottom right corner
    fn bounds(&self) -> (Position, Position) {
        let (width, height) = self.shape.size();
        (
            self.position,
            Position::new(
                self.position.x + width - 1,
                self.position.y - height + 1,
            ) 
        )
    }

    fn shape(&self) -> Vec<Position> {
        self.shape
            .shape()
            .iter()
            .map(|p| *p + self.position)
            .collect()
    }

    fn fall(&self) -> Self {
        Self {
            position: self.position + Position::new(0, -1),
            ..*self
        }
    }

    fn push(&self, direction: Jet) -> Self {
        Self {
            position: self.position + match direction {
                Jet::Left => Position::new(-1, 0),
                Jet::Right => Position::new(1, 0),
            },
            ..*self
        }
    }

    fn stuck(&self) -> Self {
        Self {
            stuck: true,
            ..*self
        }
    }

    fn collide(&self, other: &Self) -> bool {
        let my_shape = self.shape();
        let other_shape = other.shape();

        my_shape
            .iter()
            .any(|p| other_shape.contains(p))
    }

    fn encode(&self) -> i64 {
        let shape_code = match self.shape {
            Shape::Bar => 0,
            Shape::Plus => 1,
            Shape::Hook => 2,
            Shape::Pole => 3,
            Shape::Square => 4,
        };
        let position_code = self.position.x as i64;
        shape_code + 10 * position_code
    }

}

pub struct Chamber {

    // we keep track of the position of the blocks resting
    // place in order they are fallen.
    // i.e. the highest blocks are at the end of the vector
    blocks: Vec<Block>,
    jets: Box<dyn Iterator<Item=Jet>>,
    max_heights: Vec<(i32, Option<Block>)>,
}


impl Chamber {
    const WIDTH: usize = 7;

    pub fn new(jets: Box<dyn Iterator<Item=Jet>>) -> Chamber {
        Chamber {
            blocks: Vec::with_capacity(10000),
            jets,
            max_heights: vec![(0, None); Self::WIDTH],
        }
    }

    fn max_height(&self) -> i32 {
        self.blocks
            .iter()
            .map(|b| b.position.y + 1)
            .max()
            .unwrap_or(0)
    }

    // check if position is between the walls and above the floor
    fn is_position_in_chamber(&self, position: Position) -> bool {
        position.x >= 0 && position.y >= 0 && position.x < Chamber::WIDTH as i32
    }

    fn is_block_in_chamber(&self, block: &Block) -> bool {
        let (top_left, bottom_right) = block.bounds();
        self.is_position_in_chamber(top_left) && self.is_position_in_chamber(bottom_right)
    }

    fn add_shape(&mut self, shape: Shape) -> Block {
        let x = 2;
        let y = self.max_height() + shape.size().1 + 2;
        let position = Position::new(x, y);
        Block::new(position, shape)
    }

    fn is_free(&self, block: &Block) -> bool {
        if !self.is_block_in_chamber(block) {
            return false;
        }
        let from = self.blocks.len();
        let from = if from < 100 { 0 } else { from - 100 };
        self.blocks.as_slice()[from..].iter()
            .all(|b| !b.collide(block))
    }

    fn next_position(&mut self, block: &Block, jet: Jet) -> Block {
        let shifted = block.push(jet);

        let shifted = if self.is_free(&shifted) {
            shifted
        } else {
            *block
        };
        let dropped = shifted.fall();
        if self.is_free(&dropped) {
            dropped
        } else {
            let stuck_block = shifted.stuck();
            self.fix_block(stuck_block);
            stuck_block
        }
    }

    fn fix_block(&mut self, block: Block) {
        self.blocks.push(block);
        block.shape().iter().for_each(|p| {
            if p.y >= self.max_heights[p.x as usize].0 {
                self.max_heights[p.x as usize] = (p.y + 1, Some(block));
            }
        });
        self.blocks.sort_by_key(|b| b.position.y);
    }

    fn let_shape_fall(&mut self, shape: Shape) -> Block {
        let mut block = self.add_shape(shape);
        loop {
            let jet = self.jets.next().unwrap();
            block = self.next_position(&block, jet);
            if block.stuck {
                break;
            }
        }
        block
    }

    pub fn play_rounds(&mut self, rounds: usize) -> usize {
        let mut shapes = Shape::VALUES.iter().cycle();
        for i in 0..rounds {
            if i % 10000 == 0 {
                println!("play round {}", i);
            }
            self.let_shape_fall(*shapes.next().unwrap());
        }
        self.max_height() as usize
    }

    fn find_start_repeating_pattern(&mut self, period: usize) -> Option<(usize, i64)> {
        println!("playing {} rounds", 3*period);
        self.play_rounds(3*period);
        let codes: Vec<i64> = self.blocks.iter().map(|b| b.encode()).collect();
        (0..2*period)
            .inspect(|i| if i % 10000 == 0 {println!("checking round {}", i)})
            .find(|i| (*i..*i+100).all(|j| codes[j] == codes[j+period]))
            .inspect(|i| println!("found round {}", i))
            .map(|i| (
                    i, 
                    (self.blocks[i+period].position.y - self.blocks[i].position.y) as i64)
                )
    }

}

pub fn play_many_rounds(jets: Vec<Jet>, rounds: i64) -> i64 {
    let c_jets = Box::new(jets.clone().into_iter().cycle());
    let mut chamber = Chamber::new(c_jets);
    let period = 5*7*jets.len();
    let (offset, growth) = chamber.find_start_repeating_pattern(period).unwrap();
    let period = period as i64;
    println!("offset: {}, growth: {}", offset, growth);
    let multiple = (rounds - offset as i64) / period;
    let remainder = (rounds - offset as i64) % period;
    println!("multiple: {}, remainder: {}", multiple, remainder);
    let c_jets = Box::new(jets.into_iter().cycle());
    let mut chamber = Chamber::new(c_jets);
    let base_height = chamber.play_rounds(offset + remainder as usize) as i64;
    println!("base_height: {}", base_height);
    base_height + (multiple * growth)
}

pub const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

pub fn input_data() -> InputModel {
    TEST_INPUT.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = vec![Jet::Right, Jet::Right, Jet::Right, Jet::Left, Jet::Left];

        assert_eq!(actual.moves.len(), TEST_INPUT.len());
        actual.moves.iter().zip(expected.iter()).for_each(|(a, e)| assert_eq!(a, e));
    }

    #[test]
    fn add_shape() {
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves;
        let mut chamber = Chamber::new(Box::new(jets.into_iter()));
        let actual = chamber.add_shape(Shape::Bar);
        let expected = Block{position: Position::new(2, 3), shape: Shape::Bar, stuck: false};
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_max_height() {
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves;
        let mut chamber = Chamber::new(Box::new(jets.into_iter()));
        chamber.blocks.push(Block::new(Position::new(0, 0), Shape::Bar));
        let actual = chamber.max_height();
        assert_eq!(actual, 1);
    }

    #[test]
    fn next_position() {
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves;
        let mut chamber = Chamber::new(Box::new(jets.into_iter()));
        let block = Block::new(Position::new(2, 3), Shape::Bar);
        let block = chamber.next_position(&block, Jet::Right);
        let expected = Block::new(Position::new(3, 2), Shape::Bar);
        assert_eq!(block, expected);
        let block = chamber.next_position(&block, Jet::Right);
        let expected = Block::new(Position::new(3, 1), Shape::Bar);
        assert_eq!(block, expected);
        let block = chamber.next_position(&block, Jet::Right);
        let expected = Block::new(Position::new(3, 0), Shape::Bar);
        assert_eq!(block, expected);
        let block = chamber.next_position(&block, Jet::Left);
        let expected = Block::new(Position::new(2, 0), Shape::Bar).stuck();
        assert_eq!(block, expected);

        // next shape should be added
        let block = chamber.add_shape(Shape::Plus);
        let expected = Block::new(Position::new(2, 6), Shape::Plus);
        assert_eq!(block, expected);
        let block = chamber.next_position(&block, Jet::Left);
        let expected = Block::new(Position::new(1, 5), Shape::Plus);
        assert_eq!(block, expected);
        let block = chamber.next_position(&block, Jet::Right);
        let expected = Block::new(Position::new(2, 4), Shape::Plus);
        assert_eq!(block, expected);
        let block = chamber.next_position(&block, Jet::Left);
        let expected = Block::new(Position::new(1, 3), Shape::Plus);
        assert_eq!(block, expected);
        let block = chamber.next_position(&block, Jet::Right);
        let expected = Block::new(Position::new(2, 3), Shape::Plus).stuck();
        assert_eq!(block, expected);
    }

    #[test]
    fn test_drop_shape() {
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves.into_iter().cycle();
        let mut chamber = Chamber::new(Box::new(jets));
        let block = chamber.let_shape_fall(Shape::Bar);
        let expected = Block::new(Position::new(2, 0), Shape::Bar).stuck();
        assert_eq!(block, expected);
        let block = chamber.let_shape_fall(Shape::Plus);
        let expected = Block::new(Position::new(2, 3), Shape::Plus).stuck();
        assert_eq!(block, expected);
        let block = chamber.let_shape_fall(Shape::Hook);
        let expected = Block::new(Position::new(0, 5), Shape::Hook).stuck();
        assert_eq!(block, expected);
        let block = chamber.let_shape_fall(Shape::Pole);
        let expected = Block::new(Position::new(4, 6), Shape::Pole).stuck();
        assert_eq!(block, expected);
        let block = chamber.let_shape_fall(Shape::Square);
        let expected = Block::new(Position::new(4, 8), Shape::Square).stuck();
        assert_eq!(block, expected);
    }

    #[test]
    fn test_play_rounds() {
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves.into_iter().cycle();
        let mut chamber = Chamber::new(Box::new(jets.clone()));
        let actual = chamber.play_rounds(1);
        assert_eq!(actual, 1);
        let mut chamber = Chamber::new(Box::new(jets.clone()));
        let actual = chamber.play_rounds(2);
        assert_eq!(actual, 4);

        let mut chamber = Chamber::new(Box::new(jets.clone()));
        let actual = chamber.play_rounds(10);
        assert_eq!(actual, 17);

        // final test
        let mut chamber = Chamber::new(Box::new(jets));
        let actual = chamber.play_rounds(2022);
        assert_eq!(actual, 3068);
    }

    #[test]
    fn test_find_start_repeating_pattern() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();
        let jets = input.moves.clone().into_iter().cycle();
        let mut chamber = Chamber::new(Box::new(jets));
        let period = 7 * input.moves.len() * 5;

        let actual = chamber.find_start_repeating_pattern(period);
        let expected = Some((15, 2120));

        assert!(actual.is_some());
        assert_eq!(actual, expected);
    }

    #[test]
    fn find_height_large() {
        let input = TEST_INPUT.parse::<InputModel>().unwrap();

        let rounds = 1000000000000;
        let actual = play_many_rounds(input.moves, rounds);
        let expected: i64 = 1514285714288;

        assert_eq!(actual, expected);
    }

}
