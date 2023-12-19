use std::collections::HashMap;

use either::Either::{self, Left, Right};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No such workflow: {0}")]
    MissingWorkflowError(String),
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    pub fn rating(&self) -> u64 {
        self.x as u64 + self.m as u64 + self.a as u64 + self.s as u64
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PartRange {
    pub x: PropertyRange,
    pub m: PropertyRange,
    pub a: PropertyRange,
    pub s: PropertyRange,
}

impl Default for PartRange {
    fn default() -> Self {
        Self {
            x: PropertyRange::Xoolness(vec![(1, 4000)]),
            m: PropertyRange::Musicality(vec![(1, 4000)]),
            a: PropertyRange::Aero(vec![(1, 4000)]),
            s: PropertyRange::Shiny(vec![(1, 4000)]),
        }
    }
}

impl PartRange {
    pub fn with_property(&self, p: PropertyRange) -> Self {
        let mut new = self.clone();
        match p {
            PropertyRange::Xoolness(ranges) => new.x = PropertyRange::Xoolness(ranges),
            PropertyRange::Musicality(ranges) => new.m = PropertyRange::Musicality(ranges),
            PropertyRange::Aero(ranges) => new.a = PropertyRange::Aero(ranges),
            PropertyRange::Shiny(ranges) => new.s = PropertyRange::Shiny(ranges),
        }
        new
    }

    pub fn combinations(&self) -> u64 {
        self.x.count()
            * self.m.count()
            * self.a.count()
            * self.s.count()
    }
}

pub type WorkflowName = String;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Workflow {
    pub name: WorkflowName,
    pub rules: Vec<Expression>,
}

impl Workflow {
    pub fn evaluate(&self, part: &Part) -> Option<Action> {
        self.rules
            .iter()
            .find_map(|expression| expression.evaluate(part))
    }

    pub fn evaluate_range(&self, part: &PartRange) -> Vec<(Action, PartRange)> {
        let mut part = part.clone();
        let mut actions = Vec::new();
        for rule in &self.rules {
            let ((action, matched), not_matched) = rule.evaluate_range(&part);
            actions.push((action, matched.clone()));
            if let Some(remaining) = not_matched {
                part = remaining;
            } else {
                break;
            }
        }
        actions
    }

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    GT,
    LT,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Property {
    Xoolness,
    Musicality,
    Aero,
    Shiny,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PropertyRange {
    Xoolness(Vec<(u32, u32)>),
    Musicality(Vec<(u32, u32)>),
    Aero(Vec<(u32, u32)>),
    Shiny(Vec<(u32, u32)>),
}

impl PropertyRange {
    fn limit_inclusive(&self, min: u32, max: u32) -> PropertyRange {
        let filter_ranges = |ranges: &Vec<(u32, u32)>, min: u32, max: u32| {
            ranges
                .iter()
                .map(|(a, b)| (min.max(*a), max.min(*b)))
                .filter(|(a, b)| a <= b)
                .collect()
        };
        match self {
            PropertyRange::Xoolness(ranges) => {
                PropertyRange::Xoolness(filter_ranges(ranges, min, max))
            }
            PropertyRange::Musicality(ranges) => {
                PropertyRange::Musicality(filter_ranges(ranges, min, max))
            }
            PropertyRange::Aero(ranges) => PropertyRange::Aero(filter_ranges(ranges, min, max)),
            PropertyRange::Shiny(ranges) => PropertyRange::Shiny(filter_ranges(ranges, min, max)),
        }
    }

    fn count(&self) -> u64 {
        let count_ranges = |ranges: &Vec<(u32, u32)>| {
            ranges
                .iter()
                .map(|(a, b)| (b - a + 1) as u64)
                .sum::<u64>()
        };
        match self {
            PropertyRange::Xoolness(ranges) => count_ranges(ranges),
            PropertyRange::Musicality(ranges) => count_ranges(ranges),
            PropertyRange::Aero(ranges) => count_ranges(ranges),
            PropertyRange::Shiny(ranges) => count_ranges(ranges),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Term {
    Property(Property),
    Constant(u32),
}

impl Term {
    pub fn evaluate(&self, part: &Part) -> u32 {
        match self {
            Term::Property(Property::Xoolness) => part.x,
            Term::Property(Property::Musicality) => part.m,
            Term::Property(Property::Aero) => part.a,
            Term::Property(Property::Shiny) => part.s,
            Term::Constant(c) => *c,
        }
    }

    pub fn evaluate_range(&self, part: &PartRange) -> Either<u32, PropertyRange> {
        match self {
            Term::Constant(c) => Left(*c),
            Term::Property(Property::Xoolness) => Right(part.x.clone()),
            Term::Property(Property::Musicality) => Right(part.m.clone()),
            Term::Property(Property::Aero) => Right(part.a.clone()),
            Term::Property(Property::Shiny) => Right(part.s.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    Accepted,
    Rejected,
    Jump(WorkflowName),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Action(Action),
    Condition(Term, Operator, Term, Action),
}

impl Expression {
    pub fn evaluate(&self, part: &Part) -> Option<Action> {
        match self {
            Expression::Action(action) => Some(action.clone()),
            Expression::Condition(t1, op, t2, action) => {
                let t1 = t1.evaluate(part);
                let t2 = t2.evaluate(part);
                match op {
                    Operator::GT => {
                        if t1 > t2 {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                    Operator::LT => {
                        if t1 < t2 {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }

    pub fn evaluate_range(&self, part: &PartRange) -> ((Action, PartRange), Option<PartRange>) {
        match self {
            Expression::Action(action) => ((action.clone(), part.clone()), None),
            Expression::Condition(t1, op, t2, action) => {
                let mut t1 = t1.evaluate_range(part);
                let mut t2 = t2.evaluate_range(part);
                if *op == Operator::LT {
                    std::mem::swap(&mut t1, &mut t2);
                }
                // test for t1 > t2
                match t1 {
                    Left(c) => match t2 {
                        Right(r) => {
                            let matched = r.limit_inclusive(u32::MIN, c - 1);
                            let not_matched = r.limit_inclusive(c, u32::MAX);
                            (
                                (action.clone(), part.with_property(matched)),
                                Some(part.with_property(not_matched)),
                            )
                        }
                        _ => panic!("should not happen"),
                    },
                    Right(r) => match t2 {
                        Left(c2) => {
                            let matched = r.limit_inclusive(c2 + 1, u32::MAX);
                            let not_matched = r.limit_inclusive(u32::MIN, c2);
                            (
                                (action.clone(), part.with_property(matched)),
                                Some(part.with_property(not_matched)),
                            )
                        }
                        _ => panic!("should not happen"),
                    },
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Workflows(HashMap<WorkflowName, Workflow>);

impl Default for Workflows {
    fn default() -> Self {
        Self::new()
    }
}

impl Workflows {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn add(&mut self, workflow: Workflow) {
        self.0.insert(workflow.name.clone(), workflow);
    }

    pub fn get(&self, workflow_name: &str) -> Result<&Workflow, AocError> {
        self.0
            .get(workflow_name)
            .ok_or(AocError::MissingWorkflowError(workflow_name.to_string()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn evaluate(&self, part: &Part) -> Result<Action, AocError> {
        let mut current = self.get("in")?;
        loop {
            match current.evaluate(part) {
                Some(Action::Jump(next)) => {
                    current = self.get(&next)?;
                }
                Some(action) => {
                    return Ok(action);
                }
                None => {
                    return Err(AocError::ParseError);
                }
            }
        }
    }

    pub fn evaluate_range(&self, part: &PartRange) -> Result<Vec<PartRange>, AocError> {
        let mut queue = vec![(Action::Jump("in".to_string()), part.clone())];
        let mut result: Vec<PartRange> = Vec::new();
        while let Some((action, part)) = queue.pop() {
            match action {
                Action::Jump(next) => {
                    let workflow = self.get(&next)?;
                    workflow.evaluate_range(&part)
                        .into_iter()
                        .for_each(|r| queue.push(r));
                }
                Action::Accepted => {
                    result.push(part);
                }
                _ => {
                }
            }
        };
        Ok(result)
    }
}

// test part ranges
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_range() {
        let part = PartRange::default();
        assert_eq!(part.x, PropertyRange::Xoolness(vec![(1, 4000)]));
        assert_eq!(part.m, PropertyRange::Musicality(vec![(1, 4000)]));
        assert_eq!(part.a, PropertyRange::Aero(vec![(1, 4000)]));
        assert_eq!(part.s, PropertyRange::Shiny(vec![(1, 4000)]));
    }

    #[test]
    fn test_part_range_with_property() {
        let part = PartRange::default();
        let part = part.with_property(PropertyRange::Xoolness(vec![(5, 6)]));
        assert_eq!(part.x, PropertyRange::Xoolness(vec![(5, 6)]));
    }

    #[test]
    fn test_workflow_part_range() {
        let pr = PartRange::default();

        let workflow = Workflow {
            name: "in".to_string(),
            rules: vec![
                Expression::Condition(
                    Term::Property(Property::Shiny),
                    Operator::LT,
                    Term::Constant(1351),
                    Action::Jump("px".to_string()),
                ),
                Expression::Action(Action::Jump("qqz".to_string())),
            ],
        };

        let actual = workflow.evaluate_range(&pr);

        let expected = vec![
            (
                Action::Jump("px".to_string()),
                PartRange {
                    x: PropertyRange::Xoolness(vec![(1, 4000)]),
                    m: PropertyRange::Musicality(vec![(1, 4000)]),
                    a: PropertyRange::Aero(vec![(1, 4000)]),
                    s: PropertyRange::Shiny(vec![(1, 1350)]),
                },
            ),
            (
                Action::Jump("qqz".to_string()),
                PartRange {
                    x: PropertyRange::Xoolness(vec![(1, 4000)]),
                    m: PropertyRange::Musicality(vec![(1, 4000)]),
                    a: PropertyRange::Aero(vec![(1, 4000)]),
                    s: PropertyRange::Shiny(vec![(1351, 4000)]),
                },
            ),
        ];

        assert_eq!(actual, expected);
    }

}
