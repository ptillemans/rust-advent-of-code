use std::str::FromStr;

use model::{Part, Workflows};
use parser::parse_input;

use crate::model::AocError;

pub mod model;
mod parser;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub workflows: Workflows,
    pub parts: Vec<Part>,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s;
        let result = parse_input(&mut s);
        result
            .map(|(workflows, parts)| {
                let workflows: Workflows =
                    workflows
                        .into_iter()
                        .fold(Workflows::default(), |mut wfs, wf| {
                            wfs.add(wf);
                            wfs
                        });
                InputModel { workflows, parts }
            })
            .map_err(|_| AocError::ParseError)
    }
}

#[cfg(test)]
mod tests {

    use crate::model::Action;

    use super::*;

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_parse() {
        let input: InputModel = TEST_INPUT.parse().unwrap();
        assert_eq!(input.workflows.len(), 11);
        assert_eq!(input.parts.len(), 5);
    }

    #[test]
    fn test_apply_workflow() {
        let input: InputModel = TEST_INPUT.parse().unwrap();
        let part = input.parts[0].clone();
        let workflow = input.workflows.get("in").unwrap();
        let actual = workflow.evaluate(&part);
        assert_eq!(Some(Action::Jump("qqz".to_string())), actual);
    }

    #[test]
    fn test_apply_workflows() {
        let input: InputModel = TEST_INPUT.parse().unwrap();
        let part = input.parts[0].clone();
        let actual = &input.workflows.evaluate(&part).unwrap();
        assert_eq!(Action::Accepted, *actual);
    }

    #[test]
    fn test_part_rating() {
        let input: InputModel = TEST_INPUT.parse().unwrap();
        let part = input.parts[0].clone();
        let actual = part.rating();
        assert_eq!(actual, 7540);
    }
}
