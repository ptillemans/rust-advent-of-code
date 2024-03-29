use std::str::FromStr;
use nom::{
    IResult, Parser, 
    character::complete::*, 
    branch::*,
    bytes::complete::*, 
    combinator::*, 
    multi::*, 
    sequence::*,
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub monkeys: Vec<Monkey>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = separated_list1(pair(newline, newline), Monkey::parser)(s)
            .map_err(|_| AocError::ParseError);
        let (_remaining, monkeys) = result?;

        Ok(InputModel { monkeys })  
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Item(pub i64);

impl Item {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(digit1, |s: &str| Item(s.parse().unwrap()))(input)
    }

    fn inspect(&self, operation: &Operation, modulo: i64) -> Item {
        if modulo == i64::MAX {
            Item(operation.evaluate(self.0) / 3)
        } else {
            Item(operation.evaluate(self.0) % modulo)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    Old,
    Literal(i64),
}

impl Operand {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("old"), |_| Operand::Old),
            map(digit1, |s: &str| Operand::Literal(s.parse().unwrap())),
        ))(input)
    }

    fn evaluate(&self, old: i64) -> i64 {
        match self {
            Operand::Old => old,
            Operand::Literal(literal) => *literal,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum  Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

impl Operation {
    fn parser(input: &str) -> IResult<&str, Self> {
        preceded(
            tag("new = "),
            alt((
                tuple((Operand::parser, tag(" + "), Operand::parser)).map(|(a, _, b)| Operation::Add(a, b)),
                tuple((Operand::parser, tag(" * "), Operand::parser)).map(|(a, _, b)| Operation::Multiply(a, b)),
            ))
        ).parse(input)
    }

    fn evaluate(&self, old: i64) -> i64 {
        match self {
            Operation::Add(a, b) => a.evaluate(old) + b.evaluate(old),
            Operation::Multiply(a, b) => a.evaluate(old) * b.evaluate(old),
        }
    }   
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Test {
    pub divisor: i64,
    pub if_true: MonkeyId,
    pub if_false: MonkeyId,
}

impl Test {
    fn parser(input: &str) -> IResult<&str, Self> {

        map(tuple((
                delimited(
                    tag("  Test: divisible by "),
                    map(digit1, |s: &str| s.parse().unwrap()),
                    tag("\n"),
                ),
                delimited(tag("    If true: throw to monkey "), MonkeyId::parser, tag("\n")),
                preceded(tag("    If false: throw to monkey "), MonkeyId::parser)
            )),
            |(divisor, if_true, if_false)| Test { divisor, if_true, if_false })
            (input)
    }

    fn evaluate(&self, item: &Item) -> MonkeyId {
        if item.0 % self.divisor == 0 {
            self.if_true.clone()
        } else {
            self.if_false.clone()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MonkeyId(pub usize);

impl MonkeyId {
    pub fn new(id: usize) -> Self {
        MonkeyId(id)
    }

    pub fn parser(input: &str) -> nom::IResult<&str, Self> {
        map(digit1, |s: &str| MonkeyId(s.parse().unwrap()))(input)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    pub id: MonkeyId,
    pub items: Vec<Item>,
    pub operation: Operation,
    pub test: Test,
    pub inspections: i64,
}

impl Monkey {
    pub(crate) fn parser(input: &str) -> nom::IResult<&str, Self> {
        tuple((
            delimited(tag("Monkey "), MonkeyId::parser, tag(":\n")),
            delimited(tag("  Starting items: "), separated_list0(tag(", "), Item::parser), tag("\n")),
            delimited(tag("  Operation: "), Operation::parser, tag("\n")),
            Test::parser,
        ))
        .map(|(id, starting_items, operation, test)| 
             Monkey{id, items: starting_items, operation, test, inspections: 0})
        .parse(input)
    }

    pub fn inspect_inventory(&mut self, modulo: i64) -> Vec<(Item, MonkeyId)> {
        let thrown_items = self.items.iter()
            .map(|item| item.inspect(&self.operation, modulo))
            .map(|item| (item, self.test.evaluate(&item)))
            .collect();
        self.inspections += self.items.len() as i64;
        self.items.clear();
        thrown_items
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MonkeyTroop {
    monkeys: Vec<Monkey>,
    modulo: i64,
}

impl MonkeyTroop {
    pub fn new(monkeys: Vec<Monkey>, part2: bool) -> Self {
        if part2 {
            let modulo = monkeys.iter().map(|monkey| monkey.test.divisor).product::<i64>();
            MonkeyTroop { monkeys, modulo }
        } else {
            MonkeyTroop { monkeys, modulo: i64::MAX }
        }
    }

    pub(crate) fn inspection_round(&mut self) {
        let ids = self.monkeys.iter().map(|monkey| monkey.id.clone()).collect::<Vec<_>>();
        
        for id in ids {
            let monkey = &mut self.monkeys[id.0];
            let thrown_items = monkey.inspect_inventory(self.modulo);
            for (item, target) in thrown_items {
                self.monkeys[target.0].items.push(item);
            }
        }
    }

    pub fn monkey_business(&mut self, rounds: usize) -> i64 {
        for _ in 0..rounds {
            self.inspection_round();
        }
        
        let mut inspections = self.monkeys.iter().map(|monkey| monkey.inspections)
            .collect::<Vec<_>>();

        inspections.sort();
        inspections.iter()
            .rev()
            .take(2)
            .copied()
            .product()
    }
}

// publish test data to allow use here and in main.rs
pub fn input_data() -> InputModel {
    InputModel {
        monkeys:  vec![
            Monkey {
                id: MonkeyId(0),
                items: vec![Item(79), Item(98)],
                operation: Operation::Multiply(Operand::Old, Operand::Literal(19)),
                test: Test{divisor: 23, if_true: MonkeyId(2), if_false: MonkeyId(3),},
                inspections: 0,
            },
            Monkey {
                id: MonkeyId(1),
                items: vec![Item(54), Item(65), Item(75), Item(74)],
                operation: Operation::Add(Operand::Old, Operand::Literal(6)),
                test: Test{divisor: 19, if_true: MonkeyId(2), if_false: MonkeyId(0),},
                inspections: 0,
            },
            Monkey {
                id: MonkeyId(2),
                items: vec![Item(79), Item(60), Item(97)],
                operation: Operation::Multiply(Operand::Old, Operand::Old),
                test: Test{divisor: 13, if_true: MonkeyId(1), if_false: MonkeyId(3),},
                inspections: 0,
            },
            Monkey {
                id: MonkeyId(3),
                items: vec![Item(74)],
                operation: Operation::Add(Operand::Old, Operand::Literal(3)),
                test: Test{divisor: 17, if_true: MonkeyId(0), if_false: MonkeyId(1),},
                inspections: 0,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        let expected = Monkey {
            id: MonkeyId(0),
            items: vec![Item(79), Item(98)],
            operation: Operation::Multiply(Operand::Old, Operand::Literal(19)),
            test: Test{divisor: 23, if_true: MonkeyId(2), if_false: MonkeyId(3),},
            inspections: 0,
        };
        let (rest, actual) = Monkey::parser(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_inspection() {
        // test data with start item, operation and expected Result
        let test_data = vec![
            (Item(79), Operation::Multiply(Operand::Old, Operand::Literal(19)), Item(500)),
            (Item(54), Operation::Add(Operand::Old, Operand::Literal(6)), Item(20)),
            (Item(79), Operation::Multiply(Operand::Old, Operand::Old), Item(2080)),
        ];

        for (start_item, operation, expected) in test_data {
            let actual = start_item.inspect(&operation, i64::MAX);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_next_monkey_test() {
        let test_data = vec![
            (Test{divisor: 23, if_true: MonkeyId(2), if_false: MonkeyId(3),}, Item(500), MonkeyId(3)),
            (Test{divisor: 19, if_true: MonkeyId(2), if_false: MonkeyId(0),}, Item(20), MonkeyId(0)),
            (Test{divisor: 13, if_true: MonkeyId(1), if_false: MonkeyId(3),}, Item(2080), MonkeyId(1)),
        ];

        for (test, item, expected) in test_data {
            println!("test: {:?}, item: {:?}, expected: {:?}", test, item, expected);
            let actual = test.evaluate(&item);
            println!("actual: {:?}, expected: {:?}", actual, expected);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_inspection_round() {
        let expected: Vec<Vec<Item>> = vec![
            vec![20, 23, 27, 26],
            vec![2080, 25, 167, 207, 401, 1046],
            vec![],
            vec![],
        ].into_iter()
            .map(|items| items.into_iter().map(Item).collect())
            .collect();

        let mut troop = MonkeyTroop::new(input_data().monkeys, false);
        troop.inspection_round();

        let actual = troop.monkeys.iter().map(|m| m.items.clone()).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_inspection_round_20() {
        let expected: Vec<Vec<Item>> = vec![
            vec![10, 12, 14, 26, 34],
            vec![245, 93, 53, 199, 115],
            vec![],
            vec![],
        ].into_iter()
            .map(|items| items.into_iter().map(Item).collect())
            .collect();

        let mut troop = MonkeyTroop::new(input_data().monkeys, false);
        for _ in 0..20 {
            troop.inspection_round();
        }

        let actual = troop.monkeys.iter().map(|m| m.items.clone()).collect::<Vec<_>>();
        assert_eq!(actual, expected);
        assert_eq!(troop.monkeys[0].inspections, 101);
        assert_eq!(troop.monkeys[1].inspections, 95);
        assert_eq!(troop.monkeys[2].inspections, 7);
        assert_eq!(troop.monkeys[3].inspections, 105);
    }

    #[test]
    fn test_monkey_business() {
        let expected = 10605;
        let mut troop = MonkeyTroop::new(input_data().monkeys, false);
        let actual = troop.monkey_business(20);
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_inspection_round_1000() {
        let mut troop = MonkeyTroop::new(input_data().monkeys, true);
        for _ in 0..1000 {
            troop.inspection_round();
        }

        assert_eq!(troop.monkeys[0].inspections, 5204);
        assert_eq!(troop.monkeys[1].inspections, 4792);
        assert_eq!(troop.monkeys[2].inspections, 199);
        assert_eq!(troop.monkeys[3].inspections, 5192);
    }
    
    #[test]
    fn test_part2_inspection_round_10000() {
        let mut troop = MonkeyTroop::new(input_data().monkeys, true);
        for _ in 0..10000 {
            troop.inspection_round();
        }

        assert_eq!(troop.monkeys[0].inspections, 52166);
        assert_eq!(troop.monkeys[1].inspections, 47830);
        assert_eq!(troop.monkeys[2].inspections, 1938);
        assert_eq!(troop.monkeys[3].inspections, 52013);
    }
}
