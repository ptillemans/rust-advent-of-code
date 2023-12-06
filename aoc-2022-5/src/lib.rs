use std::str::FromStr;

use nom::{IResult, Parser, bytes::complete::tag, sequence::tuple, combinator::map_res, character::complete::{digit1, line_ending, anychar}, branch::alt, multi::{separated_list1, many_till}};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputModel  {
    pub stacks: Stacks,
    pub moves: Moves,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("Non existing stack")]
    IllegalStackError,
    #[error("Popping from empty stack")]
    EmptyStackError,
    #[error("No solution found")]
    NoSolution,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InputModel::parse(s)
            .map(|(_, input)| input)
            .map_err(|_| AocError::ParseError)
    }
}

impl InputModel {
    fn parse(s: &str) -> IResult<&str, InputModel> {
        let (s, stacks) = Stacks::parse(s)?;
        let (s, _) = line_ending(s)?;
        let (s, moves) = Moves::parse(s)?;

        Ok((s, InputModel { stacks, moves }))
    }
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Crate(char);

impl Crate {

    fn from_str(s: &str) -> Result<Crate, AocError> {
        s.chars()
            .next()
            .map(|c| Crate(c))
            .ok_or(AocError::ParseError)
    }

    fn parse(line: &str) -> IResult<&str, Option<Crate>>{
        let crate_parser = map_res(
            tuple((
                tag("["),
                nom::character::complete::alpha1,
                tag("]"),
            ))
            ,|(_, c, _)| (Crate::from_str(c).map(Some)));
        let empty_parser = map_res(tag("   "), |_| Ok::<Option<Crate>, AocError>(None));
        let mut parser = alt(( crate_parser, empty_parser));
        parser(line)
    }

    pub fn to_char(&self) -> char {
        self.0
    }

}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Stack(Vec<Crate>);

impl Stack {
    pub fn new() -> Stack {
        Stack(vec![])
    }
    
    pub fn from_chars(stack: Vec<char>) -> Stack {
        Stack(stack.into_iter().map(|c| Crate(c)).collect())
    }

    pub fn push(&mut self, c: Crate) {
        self.0.push(c);
    }

    pub fn pop(&mut self) -> Result<Crate, AocError> {
        self.0.pop().ok_or(AocError::EmptyStackError)
    }

    pub fn peek(&self) -> Result<&Crate, AocError> {
        self.0.last().ok_or(AocError::EmptyStackError)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Stacks(Vec<Stack>);

impl Stacks {
    pub fn new(stacks: Vec<Stack>) -> Stacks {
        Stacks(stacks)
    }

    fn parse(input: &str) -> IResult<&str, Stacks>{
        tuple((
            separated_list1(line_ending, Stacks::parse_stack_line),
            line_ending,
            many_till(anychar, line_ending)
        ))
        .map(|(stack_lines, _, _)| {
            let width = stack_lines.iter().map(|s| s.len()).max().unwrap();
            let mut stacks = vec![Stack::new(); width];
            for stack_line in stack_lines.iter().rev() {
                for (i, crate_) in stack_line.into_iter().enumerate() {
                    if let Some(c) = crate_ {
                        stacks[i].push(*c);
                    }
                }
            }
            Stacks(stacks)
        })
        .parse(input)
    }

    fn parse_stack_line(line: &str) -> IResult<&str, Vec<Option<Crate>>> {
        separated_list1(tag(" "), Crate::parse)(line)
    }

    fn pop(&mut self, i: usize) -> Result<Crate, AocError> {
        if i <= 0 {
            return Err(AocError::IllegalStackError);
        }
        let stacks = &mut self.0;
        if i > stacks.len() {
            return Err(AocError::IllegalStackError);
        }
        stacks[i-1].pop()
    }
    
    fn push(&mut self, i: usize, c: Crate) -> Result<(),AocError> {
        if i <= 0 {
            return Err(AocError::IllegalStackError);
        }
        let stacks = &mut self.0;
        if i > stacks.len() {
            return Err(AocError::IllegalStackError);
        }
        Ok(stacks[i-1].push(c))
    }

    pub fn apply_move_9000(&mut self, m: Move) -> Result<(), AocError> {
        let Move { amount, from, to } = m;
        
        for _ in 0..amount {
            let c = self.pop(from)?;
            self.push(to, c)?;
        }
        Ok(())
    }

    pub fn apply_move_9001(&mut self, m: Move) -> Result<(), AocError> {
        let Move { amount, from, to } = m;
        
        let taken = (0..amount)
            .map(|_| self.pop(from))
            .collect::<Result<Vec<Crate>, AocError>>()?;
        taken.into_iter()
            .rev()
            .map(|c| self.push(to, c))
            .collect::<Result<Vec<()>, AocError>>()?;
        Ok(())
    }

    pub fn heads(&self) -> Vec<Option<&Crate>> {
        self.0.iter().map(|s| s.peek().ok()).collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl Move {
    fn parse(line: &str) -> IResult<&str, Move> {
        let mut parser = map_res(
            tuple((
                tag("move "),
                map_res(digit1, usize::from_str),
                tag(" from "),
                map_res(digit1, usize::from_str),
                tag(" to "),
                map_res(digit1, usize::from_str),
            ))
            ,|(_, amount, _, from , _, to)| Ok::<Move, AocError>(Move{amount, from, to}));
        parser(line)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Moves(Vec<Move>);
impl Moves {

    pub fn new(moves: Vec<Move>) -> Moves {
        Moves(moves)
    }

    fn parse(s: &str) -> IResult<&str, Moves> {
        let mut parser = map_res(
            separated_list1(
                line_ending,
                Move::parse
            ),
            |moves| Ok::<Moves, AocError>(Moves(moves))
        );
        parser(s)
    }
}

impl IntoIterator for Moves {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_to_char() {
        let c = Crate::from_str("A").unwrap();
        assert_eq!(c.to_char(), 'A');
    }

    #[test]
    fn test_parse() {
        let (rest, actual) = Move::parse("move 1 from 2 to 3").unwrap();
        let expected = Move{from: 2, to: 3, amount: 1};
        assert_eq!(actual, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_moves() {
        let (rest, actual) = Moves::parse("move 2 from 2 to 1
move 4 from 1 to 3
move 3 from 2 to 1
move 2 from 1 to 2").unwrap();
        let expected = Moves(vec![
            Move{amount: 2, from: 2, to: 1},
            Move{amount: 4, from: 1, to: 3},
            Move{amount: 3, from: 2, to: 1},
            Move{amount: 2, from: 1, to: 2},
        ]);
        assert_eq!(actual, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_crate() {
        let (rest, actual) = Crate::parse("[A]").unwrap();
        let expected = Some(Crate('A'));
        assert_eq!(actual, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_empty() {
        let (rest, actual) = Crate::parse("   ").unwrap();
        let expected = None;
        assert_eq!(actual, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_stack_line() {
        let actual = Stacks::parse_stack_line("    [B] [C]");
        let expected = vec![None, Some(Crate('B')), Some(Crate('C'))];
        assert_eq!(actual.unwrap().1, expected);
    }

    #[test]
    fn test_parse_stacks() {
        let actual = Stacks::parse("    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
");
        let expected = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            Stack(vec![Crate('P')]),
        ]);
        let (rest, stacks) = actual.unwrap();
        assert_eq!(stacks, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_push_stack() {
        let mut stack = Stack::new();
        stack.push(Crate('A'));
        let expected = Stack(vec![Crate('A')]);
        assert_eq!(stack, expected);
    }

    #[test]
    fn test_pop_stack() {
        let mut stack = Stack(vec![Crate('A')]);
        let c = stack.pop().unwrap();
        let expected = Stack(vec![]);
        assert_eq!(stack, expected);
        assert_eq!(c, Crate('A'));
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack = Stack(vec![]);
        let result = stack.pop();
        assert_eq!(result, Err(AocError::EmptyStackError));
    }

    #[test]
    fn test_pop_stacks_legal() {
        let mut stacks = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            Stack(vec![Crate('P')]),
        ]);
        let c = stacks.pop(2).unwrap();
        let expected = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C')]),
            Stack(vec![Crate('P')]),
        ]);
        assert_eq!(c, Crate('D'));
        assert_eq!(stacks, expected);
    }
    
    #[test]
    fn test_pop_stacks_illegal() {
        let mut stacks = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            Stack(vec![Crate('P')]),
        ]);
        assert!(stacks.pop(4).is_err());
        assert!(stacks.pop(0).is_err());
    }
    #[test]
    fn test_push_stacks_legal() {
        let mut stacks = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            Stack(vec![Crate('P')]),
        ]);
        let c = Crate('X');
        stacks.push(3, c).unwrap();
        let expected = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            Stack(vec![Crate('P'), Crate('X')]),
        ]);
        assert_eq!(stacks, expected);
    }
    
    #[test]
    fn test_push_stacks_illegal() {
        let mut stacks = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            Stack(vec![Crate('P')]),
        ]);
        let c = Crate('X');
        assert!(stacks.push(4, c).is_err());
        assert!(stacks.push(0, c).is_err());
    }

    #[test]
    fn test_stacks_apply_move() {
        let mut stacks = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            Stack(vec![Crate('P')]),
        ]);
        let m = Move{amount: 2, from: 2, to: 3};
        stacks.apply_move_9000(m).unwrap();
        let expected = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M')]),
            Stack(vec![Crate('P'), Crate('D'), Crate('C')]),
        ]);
        assert_eq!(stacks, expected);
    }
    
    #[test]
    fn test_stacks_apply_move_leaves_empty() {
        let mut stacks = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C')]),
            Stack(vec![Crate('P')]),
        ]);
        let m = Move{amount: 2, from: 2, to: 3};
        stacks.apply_move_9000(m).unwrap();
        let expected = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![]),
            Stack(vec![Crate('P'), Crate('C'), Crate('M')]),
        ]);
        assert_eq!(stacks, expected);
    }
    
    #[test]
    fn test_heads() {
        let stacks = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            Stack(vec![Crate('P')]),
        ]);
        let actual = stacks.heads().into_iter().map(|oc| oc.unwrap().to_owned()).collect::<Vec<Crate>>();
        let expected = vec![Crate('N'), Crate('D'),Crate('P')];
        assert_eq!(actual, expected);
    }
}
    #[test]
    fn test_stacks_apply_move_9001() {
        let mut stacks = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            Stack(vec![Crate('P')]),
        ]);
        let m = Move{amount: 2, from: 2, to: 3};
        stacks.apply_move_9001(m).unwrap();
        let expected = Stacks(vec![
            Stack(vec![Crate('Z'), Crate('N')]),
            Stack(vec![Crate('M')]),
            Stack(vec![Crate('P'), Crate('C'), Crate('D')]),
        ]);
        assert_eq!(stacks, expected);
    }
    
