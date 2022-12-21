use std::{str::FromStr, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub expressions: Expressions,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expressions: Expressions = s.lines()
            .map(|line| {
                match line.split(": ").collect::<Vec<&str>>().as_slice() {
                    [id, expression] => {
                        id.parse()
                            .and_then(|id| 
                                  expression.parse()
                                  .map(|expression| (id, expression)))
                    }
                    _ => Err(AocError::ParseError),
                }
            })
            .collect::<Result<Expressions,_>>()?;

        Ok(InputModel { expressions })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MonkeyId(String);

impl MonkeyId {
   pub fn new(id: &str) -> MonkeyId {
       MonkeyId(id.to_string())
   }
}


impl FromStr for MonkeyId {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MonkeyId(s.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Add(MonkeyId, MonkeyId),
    Subtract(MonkeyId, MonkeyId),
    Multiply(MonkeyId, MonkeyId),
    Divide(MonkeyId, MonkeyId),
    Literal(i128),
    Hole,
    Root(MonkeyId, MonkeyId),
}

pub type Expressions = HashMap<MonkeyId, Expression>;

impl FromStr for Expression {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            [a, "+", b] => Ok(Expression::Add(a.parse()?, b.parse()?)),
            [a, "-", b] => Ok(Expression::Subtract(a.parse()?, b.parse()?)),
            [a, "*", b] => Ok(Expression::Multiply(a.parse()?, b.parse()?)),
            [a, "/", b] => Ok(Expression::Divide(a.parse()?, b.parse()?)),
            [a] => Ok(Expression::Literal(a.parse()
                      .map_err(|_| AocError::ParseError)?)),
            _ => Err(AocError::ParseError),
        }
    }
}

pub fn evaluate(monkey: &MonkeyId, expressions: &Expressions) -> Option<i128> {
    match expressions.get(monkey).unwrap() {
        Expression::Add(a, b) => evaluate(a, expressions)
            .and_then(|a| evaluate(b, expressions)
                      .map(|b| a + b)),
        Expression::Subtract(a, b) => evaluate(a, expressions)
            .and_then(|a| evaluate(b, expressions)
                      .map(|b| a - b)),
        Expression::Multiply(a, b) => evaluate(a, expressions)
            .and_then(|a| evaluate(b, expressions)
                      .map(|b| a * b)),
        Expression::Divide(a, b) => evaluate(a, expressions)
            .and_then(|a| evaluate(b, expressions)
                      .map(|b| a / b)),
        Expression::Literal(literal) => Some(*literal),
        _ => None,
    }
}


pub  fn prepare_part2(expressions: &Expressions) -> Expressions {
    let mut expressions = expressions.clone();
    expressions.entry(MonkeyId::new("root"))
        .and_modify(|root|
            *root = match root {
                Expression::Add(a, b) => Expression::Root(a.clone(), b.clone()),
                Expression::Subtract(a, b) => Expression::Root(a.clone(), b.clone()),
                Expression::Multiply(a, b) => Expression::Root(a.clone(), b.clone()),
                Expression::Divide(a, b) => Expression::Root(a.clone(), b.clone()),
                _ => panic!("Unexpected expression"),
            }
        );
    expressions.insert(MonkeyId::new("humn"), Expression::Hole);
    expressions
}

pub fn find_hole(monkey: &MonkeyId, target: i128, expressions: &Expressions) -> Option<i128> {
    let expression = expressions.get(monkey).unwrap();
    println!("find_hole: {expression:?} -> {target}");

    match expression {
        Expression::Add(a, b) => find_hole_add(a, b, target, expressions),
        Expression::Subtract(a, b) => find_hole_subtract(a, b, target, expressions),
        Expression::Multiply(a, b) => find_hole_multiply(a, b, target, expressions),
        Expression::Divide(a, b) => find_hole_divide(a, b, target, expressions),
        Expression::Hole => Some(target),
        Expression::Root(a, b) => find_hole_root(a, b, expressions),
        Expression::Literal(_) => None,  // there in no hole to determine
    }
    
}

fn find_hole_add(a: &MonkeyId, b: &MonkeyId, target: i128, expressions: &HashMap<MonkeyId, Expression>) -> Option<i128> {
    let a_value = evaluate(a, expressions);
    let b_value = evaluate(b, expressions);
    match (a_value, b_value) {
        (None, Some(b)) => find_hole(a, target - b, expressions),
        (Some(a), None) => find_hole(b, target - a, expressions),
        _ => None,  // either both are known or neither
    }
}

fn find_hole_subtract(a: &MonkeyId, b: &MonkeyId, target: i128, expressions: &HashMap<MonkeyId, Expression>) -> Option<i128> {
    let a_value = evaluate(a, expressions);
    let b_value = evaluate(b, expressions);
    println!("subtract a: {a_value:?}, b: {b_value:?}, target: {target}");
    match (a_value, b_value) {
        (None, Some(b)) => find_hole(a, target + b, expressions),
        (Some(a), None) => find_hole(b, a - target, expressions),
        _ => None,
    }
}

fn find_hole_multiply(a: &MonkeyId, b: &MonkeyId, target: i128, expressions: &HashMap<MonkeyId, Expression>) -> Option<i128> {
    let a_value = evaluate(a, expressions);
    let b_value = evaluate(b, expressions);
    match (a_value, b_value) {
        (None, Some(b)) => find_hole(a, target / b, expressions),
        (Some(a), None) => find_hole(b, target / a, expressions),
        _ => None,
    }
}

fn find_hole_divide(a: &MonkeyId, b: &MonkeyId, target: i128, expressions: &HashMap<MonkeyId, Expression>) -> Option<i128> {
    let a_value = evaluate(a, expressions);
    let b_value = evaluate(b, expressions);
    println!("divide a: {a_value:?}, b: {b_value:?}, target: {target}");
    match (a_value, b_value) {
        (None, Some(b)) => find_hole(a, target * b, expressions),
        (Some(a), None) => find_hole(b, a / target, expressions),
        _ => None,
    }
}

fn find_hole_root(a: &MonkeyId, b: &MonkeyId, expressions: &HashMap<MonkeyId, Expression>) -> Option<i128> {
    let a_value = evaluate(a, expressions);
    let b_value = evaluate(b, expressions);
    match (a_value, b_value) {
        (None, Some(b)) => find_hole(a, b, expressions),
        (Some(a), None) => find_hole(b, a, expressions),
        _ => None,
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "a: 1\nb: 2\nc: a + b";
        let expected = InputModel {
            expressions: vec![
                (MonkeyId("a".to_string()), Expression::Literal(1)),
                (MonkeyId("b".to_string()), Expression::Literal(2)),
                (MonkeyId("c".to_string()), Expression::Add(MonkeyId("a".to_string()), MonkeyId("b".to_string()))),
            ].into_iter().collect(),
        };
        let actual: InputModel = input.parse().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_evaluate() {
        let input = "a: 1\nb: 2\nc: a + b";
        let input: InputModel = input.parse().unwrap();
        let expected = Some(3);
        let actual = evaluate(&MonkeyId("c".to_string()), &input.expressions);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_prepare_part2() {
        let input = "a: 1\nb: 2\nc: a + b\nroot: c * humn\nhumn: 3";
        let input: InputModel = input.parse().unwrap();
        let actual = prepare_part2(&input.expressions);
        assert_eq!(&Expression::Hole, actual.get(&MonkeyId::new("humn")).unwrap());
    }

    #[test]
    fn test_find_hole() {

        let test_data = &[
            (1, "a: 1\nroot: a + humn"),
            (6, "a: 9\nb: c + humn\nc: 3\nroot: a + b" ),
            (-3, "a: 9\nb: c - humn\nc: 6\nroot: a + b" ),
            (6, "a: 9\nb: humn + c\nc: 3\nroot: a + b" ),
            (15, "a: 9\nb: humn - c\nc: 6\nroot: a + b" ),
            (3, "a: 9\nb: c * humn\nc: 3\nroot: a + b" ),
            (3, "a: 3\nb: c / humn\nc: 9\nroot: a + b" ),
            (3, "a: 9\nb: humn * c\nc: 3\nroot: a + b" ),
            (9, "a: 3\nb: humn / c\nc: 3\nroot: a + b" ),
        ];

        for (expected, input) in test_data {
            println!("test fine hole in : {input}");
            let model: InputModel = input.parse().unwrap();
            let expressions = prepare_part2(&model.expressions);
            let hole = find_hole(&MonkeyId::new("root"), 0,  &expressions);
            println!("Hole: {hole:?}, expected {expected}");
            assert_eq!(Some(*expected), hole);
        }
    }

}
