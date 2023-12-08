use std::{collections::HashMap, str::FromStr};
use time::{macros::datetime, PrimitiveDateTime};

const INPUT: &str = include_str!("../data/input.txt");

type GuardPatterns = HashMap<Guard, [u16; 60]>;

fn aggregate_sleep_patterns(events: &[Event]) -> GuardPatterns {
    let naps = events.iter()
        .scan(
            (Guard { id: 0 }, datetime!(1000-01-01 00:00)),
            |(guard, sleep_time), ev| match ev {
                Event::BeginShift(_, g) => {
                    *guard = *g;
                    Some((*guard, 0, 0)) 
                }
                Event::FallsAsleep(dt) => {
                    *sleep_time = *dt;
                    Some((*guard, 0, 0))
                }
                Event::WakesUp(dt) => Some((*guard, sleep_time.minute(), dt.minute())),
            },
        )
        .filter(|(_, _, till)| *till > 0);
    let mut patterns: GuardPatterns = HashMap::new();
    for (guard, from, till) in naps {
        for m in from..till {
            patterns.entry(guard).or_insert([0; 60])[m as usize] += 1
        }
    }
    patterns
}

fn part1(input: &InputModel) -> Result<String, AocError> {
    let patterns: GuardPatterns = aggregate_sleep_patterns(&input.events);
    let max_sleep_guard = patterns
        .iter()
        .max_by(|a, b| {
            let sum_a: u16 = a.1.iter().sum();
            let sum_b: u16 = b.1.iter().sum();
            sum_a.cmp(&sum_b)
        })
        .map(|(guard, _)| guard)
        .ok_or(AocError::NoSolution)?;

    let pattern = patterns.get(max_sleep_guard).unwrap();
    let max_minute = (0_usize ..60)
        .max_by(|&a, &b| pattern[a].cmp(&pattern[b]))
        .ok_or(AocError::NoSolution)?;

    Ok((max_sleep_guard.id * max_minute).to_string())
}

fn part2(input: &InputModel) -> Result<String, AocError> {
    let patterns: GuardPatterns = aggregate_sleep_patterns(&input.events);
    let max_sleep_guard = patterns
        .iter()
        .max_by(|a, b| {
            let sum_a: u16 = a.1.iter().max().unwrap().to_owned();
            let sum_b: u16 = b.1.iter().max().unwrap().to_owned();
            sum_a.cmp(&sum_b)
        })
        .map(|(guard, _)| guard)
        .ok_or(AocError::NoSolution)?;

    let pattern = patterns.get(max_sleep_guard).unwrap();
    let max_minute = (0_usize ..60)
        .max_by(|&a, &b| pattern[a].cmp(&pattern[b]))
        .ok_or(AocError::NoSolution)?;

    Ok((max_sleep_guard.id * max_minute).to_string())
}

fn main() -> Result<(), AocError> {
    let input: InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct InputModel {
    events: Vec<Event>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution found")]
    NoSolution,
}

fn parse_event(s: &str) -> Result<Event, AocError> {
    parsers::parse_event(s)
        .map(move |(_, ev)| ev)
        .map_err(|_| AocError::ParseError)
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut events = s
            .lines()
            .map(parse_event)
            .collect::<Result<Vec<Event>, AocError>>()?;
        events.sort();
        Ok(InputModel { events })
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
pub struct Guard {
    id: usize,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Event {
    BeginShift(PrimitiveDateTime, Guard),
    FallsAsleep(PrimitiveDateTime),
    WakesUp(PrimitiveDateTime),
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let o_dt = match other {
            Event::BeginShift(dt, _) => dt,
            Event::FallsAsleep(dt) => dt,
            Event::WakesUp(dt) => dt,
        };
        match self {
            Self::BeginShift(dt, _) => dt.cmp(o_dt),
            Self::FallsAsleep(dt) => dt.cmp(o_dt),
            Self::WakesUp(dt) => dt.cmp(o_dt),
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take, take_while},
        character::complete::{char, digit1},
        sequence::{self, delimited},
        IResult,
    };
    use time::{macros::format_description, PrimitiveDateTime};

    use crate::{Event, Guard};

    pub fn parse_instant(s: &str) -> IResult<&str, PrimitiveDateTime> {
        let format = format_description!("[year]-[month]-[day] [hour]:[minute]");
        delimited(char('['), take(16_usize), char(']'))(s)
            .map(|(s, dt)| (s, PrimitiveDateTime::parse(dt, format).unwrap()))
    }

    pub fn parse_begin_shift(s: &str) -> IResult<&str, usize> {
        sequence::tuple((tag("Guard #"), digit1, tag(" begins shift")))(s)
            .map(|(s, (_, guard_id, _))| (s, guard_id.parse::<usize>().unwrap()))
    }

    pub fn parse_observation(dt: PrimitiveDateTime) -> impl Fn(&str) -> IResult<&str, Event> {
        move |s: &str| {
            alt((
                move |s| tag("falls asleep")(s).map(|(s, _)| (s, Event::FallsAsleep(dt))),
                move |s| tag("wakes up")(s).map(|(s, _)| (s, Event::WakesUp(dt))),
                move |s| {
                    parse_begin_shift(s)
                        .map(move |(s, id)| (s, Event::BeginShift(dt, Guard { id })))
                },
            ))(s)
        }
    }

    pub fn parse_event(s: &str) -> IResult<&str, Event> {
        let (s, dt) = parse_instant(s)?;
        let (s, _) = take_while(char::is_whitespace)(s)?;
        parse_observation(dt)(s)
    }

    #[cfg(test)]
    mod tests {
        use time::macros::datetime;

        use crate::Guard;

        use super::*;

        const TEST_LINE_ASLEEP: &str = "[1518-11-01 00:05] falls asleep";
        const TEST_LINE_WAKES: &str = "[1518-11-01 00:25] wakes up";
        const TEST_LINE_BEGIN_SHIFT: &str = "[1518-11-01 00:00] Guard #10 begins shift";

        #[test]
        fn test_parse_instant() {
            let actual = parse_instant(TEST_LINE_ASLEEP).unwrap();
            let expected = (" falls asleep", datetime!(1518-11-01 00:05));
            assert_eq!(actual, expected)
        }

        #[test]
        fn test_parse_fall_asleep_observation() {
            let actual = parse_event(TEST_LINE_ASLEEP).unwrap();
            let expected = ("", Event::FallsAsleep(datetime!(1518-11-01 00:05)));
            assert_eq!(actual, expected)
        }

        #[test]
        fn test_parse_wakes_up_observation() {
            let actual = parse_event(TEST_LINE_WAKES).unwrap();
            let expected = ("", Event::WakesUp(datetime!(1518-11-01 00:25)));
            assert_eq!(actual, expected)
        }

        #[test]
        fn test_parse_fall_begins_shift() {
            let actual = parse_event(TEST_LINE_BEGIN_SHIFT).unwrap();
            let expected = (
                "",
                Event::BeginShift(datetime!(1518-11-01 00:00), Guard { id: 10 }),
            );

            assert_eq!(actual, expected)
        }
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    const TEST_INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:45] falls asleep
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:55] wakes up";

    fn input_data() -> InputModel {
        InputModel {
            events: vec![
                Event::BeginShift(datetime!(1518-11-01 00:00), Guard { id: 10 }),
                Event::FallsAsleep(datetime!(1518-11-01 00:05)),
                Event::WakesUp(datetime!(1518-11-01 00:25)),
                Event::FallsAsleep(datetime!(1518-11-01 00:30)),
                Event::WakesUp(datetime!(1518-11-01 00:55)),
                Event::BeginShift(datetime!(1518-11-01 23:58), Guard { id: 99 }),
                Event::FallsAsleep(datetime!(1518-11-02 00:40)),
                Event::WakesUp(datetime!(1518-11-02 00:50)),
                Event::BeginShift(datetime!(1518-11-03 00:05), Guard { id: 10 }),
                Event::FallsAsleep(datetime!(1518-11-03 00:24)),
                Event::WakesUp(datetime!(1518-11-03 00:29)),
                Event::BeginShift(datetime!(1518-11-04 00:02), Guard { id: 99 }),
                Event::FallsAsleep(datetime!(1518-11-04 00:36)),
                Event::WakesUp(datetime!(1518-11-04 00:46)),
                Event::BeginShift(datetime!(1518-11-05 00:03), Guard { id: 99 }),
                Event::FallsAsleep(datetime!(1518-11-05 00:45)),
                Event::WakesUp(datetime!(1518-11-05 00:55)),
            ],
        }
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "240";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "4455";

        assert_eq!(actual, expected);
    }
}
