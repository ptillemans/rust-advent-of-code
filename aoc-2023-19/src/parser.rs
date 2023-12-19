use crate::model::*;
use winnow::{
    ascii::{alpha1, digit1, newline},
    combinator::*,
    error::StrContext,
    prelude::*,
    Parser,
};

fn term(s: &mut &str) -> PResult<Term> {
    alt((
        "x".map(|_| Term::Property(Property::Xoolness)),
        "m".map(|_| Term::Property(Property::Musicality)),
        "a".map(|_| Term::Property(Property::Aero)),
        "s".map(|_| Term::Property(Property::Shiny)),
        digit1.try_map(|s: &str| s.parse().map(Term::Constant)),
    ))
    .context(StrContext::Label("term"))
    .parse_next(s)
}

fn action(s: &mut &str) -> PResult<Action> {
    alt((
        "A".map(|_| Action::Accepted),
        "R".map(|_| Action::Rejected),
        alpha1.map(|s: &str| Action::Jump(s.to_string())),
    ))
    .context(StrContext::Label("action"))
    .parse_next(s)
}

fn expression(s: &mut &str) -> PResult<Expression> {
    alt((
        (
            term,
            alt(("<".map(|_| Operator::LT), ">".map(|_| Operator::GT)))
                .context(StrContext::Label("operator")),
            term,
            ":",
            action,
        )
            .map(|(t1, op, t2, _, action)| Expression::Condition(t1, op, t2, action)),
        action.map(|a| Expression::Action(a)),
    ))
    .context(StrContext::Label("expression"))
    .parse_next(s)
}

fn parse_workflow(s: &mut &str) -> PResult<Workflow> {
    (
        alpha1::<&str, _>.context(StrContext::Label("workflow name")),
        "{".context(StrContext::Label("workflow rules start")),
        separated(1.., expression, ",").context(StrContext::Label("expressions")),
        "}".context(StrContext::Label("workflow rules end")),
    )
        .map(|(name, _, rules, _)| Workflow {
            name: name.to_string(),
            rules,
        })
        .parse_next(s)
}

fn parse_part(s: &mut &str) -> PResult<Part> {
    let property = (
        alt((
            "x".map(|_| Property::Xoolness),
            "m".map(|_| Property::Musicality),
            "a".map(|_| Property::Aero),
            "s".map(|_| Property::Shiny),
        )),
        "=",
        digit1.try_map(|s: &str| s.parse()),
    )
        .map(|(prop, _, value)| (prop, value));

    ("{", separated(1.., property, ","), "}")
        .map(|(_, properties, _): (&str, Vec<(Property, u32)>, &str)| {
            properties
                .into_iter()
                .fold(Part::default(), |mut part, (prop, value)| {
                    match prop {
                        Property::Xoolness => part.x = value,
                        Property::Musicality => part.m = value,
                        Property::Aero => part.a = value,
                        Property::Shiny => part.s = value,
                    }
                    part
                })
        })
        .parse_next(s)
}

pub fn parse_input(s: &mut &str) -> PResult<(Vec<Workflow>, Vec<Part>)> {
    (
        separated(1.., parse_workflow, newline),
        newline,
        newline,
        separated(1.., parse_part, newline),
        opt(newline),
    )
        .map(|(workflows, _, _, parts, _)| (workflows, parts))
        .parse_next(s)
}
