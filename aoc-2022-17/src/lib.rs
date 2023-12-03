use std::str::FromStr;
use std::collections::HashMap;
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
    jets: Vec<Jet>,
    cache: HashMap<(usize, usize), (i64, i64)>,
    j: usize,
    s: usize,
    n: usize,
}


impl Chamber {
    const WIDTH: usize = 7;

    pub fn new(jets: &[Jet]) -> Chamber {
        Chamber {
            blocks: Vec::with_capacity(10000),
            jets: jets.to_vec(),
            cache: HashMap::new(),
            j: 0,
            s: 0,
            n: 0,
        }
    }

    fn max_height(&self) -> i32 {
        self.last_blocks().iter()
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

    fn last_blocks(&self) -> &[Block] {
        let from = self.blocks.len();
        let from = if from < 100 { 0 } else { from - 100 };
        &self.blocks.as_slice()[from..]
    }

    fn is_free(&self, block: &Block) -> bool {
        if !self.is_block_in_chamber(block) {
            return false;
        }
        self.last_blocks().iter()
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
        self.n += 1;
    }

    fn let_shape_fall(&mut self, shape: Shape) -> Block {
        let mut block = self.add_shape(shape);
        loop {
            let jet = self.jets[self.j];
            let l = self.jets.len();
            self.j = (self.j + 1) % l;
            block = self.next_position(&block, jet);
            if block.stuck {
                break;
            }
        }
        block
    }

    pub fn play_rounds(&mut self, rounds: usize) -> usize {
        for i in 0..rounds {
            if i % 10000 == 0 {
                println!("play round {}", i);
            }

            let key = (self.s, self.j);
            let height = self.max_height() as i64;
            if let Some((n, h)) = self.cache.get(&key) {
                let period = self.n as i64 - n;
                if ((rounds as i64 - *n as i64) % period) == 0 {
                    let delta : i64 = height - h;
                    let rest : i64 = rounds as i64 - *n as i64;
                    let result = h + delta*(rest/period);
                    return result as usize
                }
            } else {
                self.cache.insert(key, (self.n as i64, height));
            }

            let shape = Shape::VALUES[self.s];
            self.s = (self.s + 1) % 5;
            self.let_shape_fall(shape);
        }
        self.max_height() as usize
    }

    fn find_start_repeating_pattern(&mut self, period: usize) -> Option<(usize, i64)> {
        println!("playing {} rounds", 2*period);
        self.play_rounds(2*period);
        let codes: Vec<i64> = self.blocks.iter().map(|b| b.encode()).collect();
        (0..period)
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
    let mut chamber = Chamber::new(&jets.clone());
    let period = 7*5*jets.len();
    let (offset, growth) = chamber.find_start_repeating_pattern(period).unwrap();
    let period = period as i64;
    println!("offset: {}, growth: {}", offset, growth);
    let multiple = (rounds - offset as i64) / period;
    let remainder = (rounds - offset as i64) % period;
    println!("multiple: {}, remainder: {}", multiple, remainder);
    let mut chamber = Chamber::new(&jets.clone());
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
        let mut chamber = Chamber::new(&jets);
        let actual = chamber.add_shape(Shape::Bar);
        let expected = Block{position: Position::new(2, 3), shape: Shape::Bar, stuck: false};
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_max_height() {
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves;
        let mut chamber = Chamber::new(&jets);
        chamber.blocks.push(Block::new(Position::new(0, 0), Shape::Bar));
        let actual = chamber.max_height();
        assert_eq!(actual, 1);
    }

    #[test]
    fn next_position() {
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves;
        let mut chamber = Chamber::new(&jets);
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
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves;
        let mut chamber = Chamber::new(&jets);
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
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves;
        let mut chamber = Chamber::new(&jets.clone());
        let actual = chamber.play_rounds(1);
        assert_eq!(actual, 1);
        let mut chamber = Chamber::new(&jets.clone());
        let actual = chamber.play_rounds(2);
        assert_eq!(actual, 4);

        let mut chamber = Chamber::new(&jets.clone());
        let actual = chamber.play_rounds(10);
        assert_eq!(actual, 17);

        // final test
        let mut chamber = Chamber::new(&jets);
        let actual = chamber.play_rounds(2022);
        assert_eq!(actual, 3068);
    }

    #[test]
    fn find_height_large() {
        let jets = TEST_INPUT.parse::<InputModel>().unwrap().moves;
        let mut chamber = Chamber::new(&jets.clone());

        let rounds = 1000000000000;
        let actual = chamber.play_rounds(rounds);
        let expected = 1514285714288;

        assert_eq!(actual, expected);
    }

}
