use std::{
    collections::{HashMap, BTreeMap},
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub circuit: Circuit,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Module {
    Broadcaster(String),
    FlipFlop(String, bool),
    Conjunction(String, BTreeMap<String, bool>),
}

impl Module {
    fn name(&self) -> String {
        match self {
            Module::Broadcaster(name) => name,
            Module::FlipFlop(name, _) => name,
            Module::Conjunction(name, _) => name,
        }
        .to_string()
    }

    fn step(&self, pulse: &Pulse) -> (Module, Option<bool>) {
        match self {
            Module::Broadcaster(_) => (self.clone(), Some(pulse.is_high())),
            Module::FlipFlop(_, state) => match pulse {
                Pulse::High(_, _) => (self.clone(), None),
                Pulse::Low(_, _) => (Module::FlipFlop(self.name(), !state), Some(!state)),
            },
            Module::Conjunction(_, inputs) => {
                let mut new_inputs = inputs.clone();
                new_inputs.insert(pulse.from(), pulse.is_high());
                let all_high = new_inputs.values().all(|v| *v);
                let new_module = Module::Conjunction(self.name(), new_inputs);
                (new_module, Some(!all_high))
            }
        }
    }

    fn encode_state(&self) -> u64 {
        match self {
            Module::Broadcaster(_) => 0,
            Module::FlipFlop(_, state ) => if *state { 1 } else { 0 },
            Module::Conjunction(_, state ) => {
                state.iter()
                    .fold(0, |acc, (_, v)| acc * 2 + if *v {0} else {1})
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Circuit {
    modules: HashMap<String, Module>,
    connections: HashMap<String, Vec<String>>,
}

impl Circuit {

    pub fn inputs(&self, name: &str) -> Vec<String> {
        self.connections.iter()
            .filter(|(_, targets)| targets.contains(&name.to_string()))
            .map(|(k, _)| k.clone())
            .collect()
    }

    fn step(&mut self, pulses: Vec<Pulse>) -> Vec<Pulse> {
        let convert_pulse = |mp: Option<bool>, from: &str, to: &str| match mp {
            Some(true) => Some(Pulse::high(from, to)),
            Some(false) => Some(Pulse::low(from, to)),
            _ => None,
        };

        let module_pulses = |incoming: &Pulse| {
            let conns = &self.connections;
            let mut outgoing_pulses = vec![];
            self.modules.entry(incoming.to()).and_modify(|module| {
                let (new_module, outgoing) = module.step(incoming);
                *module = new_module;
                conns
                    .get(&module.name())
                    .into_iter()
                    .flat_map(|targets| targets.iter())
                    .filter_map(|target| convert_pulse(outgoing, &module.name(), target))
                    .for_each(|pulse| outgoing_pulses.push(pulse));
            });
            outgoing_pulses
        };

        pulses.iter().flat_map(module_pulses).collect()
    }

    pub fn push_button(&mut self) -> Vec<Pulse> {
        let button = Pulse::low("button", "broadcaster");
        let mut all_pulses = vec![button.clone()];
        let mut pulses = vec![button.clone()];
        while !pulses.is_empty() {
            pulses = self.step(pulses);
            all_pulses.extend(pulses.clone());
        }
        all_pulses
    }

    fn state_summary(&mut self) -> Vec<(String, u64)> {
        self.modules.iter()
            .filter(|(_, m)| matches!(*m, Module::Conjunction(_, _)))
            .map(|(name, m)| (name.clone(), m.encode_state()))
            .collect()
    }


    pub fn find_cycles(&mut self) -> HashMap<String, (usize, usize)> {
        const LOOPS: usize = 100000;

        let mut states = HashMap::<String, Vec<u64>>::new();
        (0..LOOPS)
            .for_each(|_| {
            self.push_button();
                self.state_summary().iter()
                    .for_each(|(name, state)| {
                        states.entry(name.clone())
                            .and_modify(|v| (*v).push(*state))
                            .or_insert(vec![*state]);
                    })
            });

        states.iter()
            .map(|(k, samples)| {
                (k.clone(), cycle_detection(samples))
            })
            .collect()
    }
}

fn cycle_detection(samples: &[u64]) -> (usize, usize) {
    // assume the halfway point is in a cycle
    let half_way = samples[samples.len()/2];
    let cycle = (samples.len()/2+16..samples.len())
        .filter(|&i| samples[i] == half_way)   // find candidates
        .find(|&i| {
            let len = i - samples.len()/2;
            (0..len).all(|j| samples[i+j] == samples[i + j + len])
        })
        .unwrap();
    let len = cycle - samples.len()/2;

    let first = (0..)
        .find(|&i| (0..cycle).all(|j| samples[i+j] == samples[i + j + len]))
        .unwrap();

    (first, len)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pulse {
    Low(String, String),
    High(String, String),
}

impl Pulse {
    pub fn low(from: &str, to: &str) -> Pulse {
        Pulse::Low(from.to_string(), to.to_string())
    }

    pub fn high(from: &str, to: &str) -> Pulse {
        Pulse::High(from.to_string(), to.to_string())
    }

    pub fn from(&self) -> String {
        match self {
            Pulse::Low(from, _) => from.to_string(),
            Pulse::High(from, _) => from.to_string(),
        }
    }

    pub fn to(&self) -> String {
        match self {
            Pulse::Low(_, to) => to.to_string(),
            Pulse::High(_, to) => to.to_string(),
        }
    }

    pub fn is_high(&self) -> bool {
        match self {
            Pulse::Low(_, _) => false,
            Pulse::High(_, _) => true,
        }
    }
}

impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Low(from, to) => write!(f, "{} -low-> {}", from, to),
            Pulse::High(from, to) => write!(f, "{} -high-> {}", from, to),
        }
    }
}
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(parse_line).collect::<Result<Vec<_>, _>>()?;

        let mut modules = lines.iter()
            .map(|(module, _)| (module.name(), module.clone()))
            .collect::<HashMap<_, _>>();

        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        lines
            .iter()
            .flat_map(|(_, connections)| connections)
            .cloned()
            .for_each(|(from, to)| connections.entry(from).or_default().push(to));

        modules.iter_mut()
            .filter(|(_, m)| matches!(m, Module::Conjunction(_, _)))
            .for_each(|(_, v)| {
                let inputs =  connections.iter()
                    .filter(|(_, target)| target .contains(&v.name()))
                    .map(|(k, _)| (k.clone(), false))
                    .collect::<BTreeMap<String, bool>>();
                *v = Module::Conjunction(v.name(), inputs);
            });
    
        let circuit = Circuit {
            modules,
            connections,
        };
        Ok(InputModel { circuit })
    }
}

fn parse_line(s: &str) -> Result<(Module, Vec<(String, String)>), AocError> {
    let mut tokens = s.split(" -> ");
    let first = tokens.next().ok_or(AocError::ParseError)?;
    let second = tokens.next().ok_or(AocError::ParseError)?;

    let comp = if let Some(name) = first.strip_prefix('%') {
        Module::FlipFlop(name.to_string(), false)
    } else if let Some(name) = first.strip_prefix('&') {
        Module::Conjunction(name.to_string(), BTreeMap::new())
    } else {
        Module::Broadcaster(first.to_string())
    };

    let connections = second
        .split(", ")
        .map(|s| (comp.name(), s.to_string()))
        .collect::<Vec<_>>();

    Ok((comp, connections))
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    #[test]
    fn test_parse_line() {
        let (comp, connections) = parse_line("broadcaster -> a, b, c").unwrap();
        assert_eq!(comp, Module::Broadcaster("broadcaster".to_string()));
        assert_eq!(
            connections,
            vec![
                ("broadcaster".to_string(), "a".to_string()),
                ("broadcaster".to_string(), "b".to_string()),
                ("broadcaster".to_string(), "c".to_string())
            ]
        );

        let (comp, connections) = parse_line("%a -> b").unwrap();
        assert_eq!(comp, Module::FlipFlop("a".to_string(), false));
        assert_eq!(connections, vec![("a".to_string(), "b".to_string())]);

        let (comp, connections) = parse_line("%b -> c").unwrap();
        assert_eq!(comp, Module::FlipFlop("b".to_string(), false));
        assert_eq!(connections, vec![("b".to_string(), "c".to_string())]);

        let (comp, connections) = parse_line("%c -> inv").unwrap();
        assert_eq!(comp, Module::FlipFlop("c".to_string(), false));
        assert_eq!(connections, vec![("c".to_string(), "inv".to_string())]);

        let (comp, connections) = parse_line("&inv -> a").unwrap();
        assert_eq!(comp, Module::Conjunction("inv".to_string(), BTreeMap::new()));
        assert_eq!(comp, Module::Conjunction("inv".to_string(), BTreeMap::new()));
        assert_eq!(connections, vec![("inv".to_string(), "a".to_string())]);
    }

    #[test]
    fn test_parse_input() {
        let input = InputModel::from_str(TEST_INPUT).unwrap();
        let mut modules = HashMap::new();
        vec![
            Module::Broadcaster("broadcaster".to_string()),
            Module::FlipFlop("a".to_string(), false),
            Module::FlipFlop("b".to_string(), false),
            Module::FlipFlop("c".to_string(), false),
            Module::Conjunction(
                "inv".to_string(),
                vec![("c".to_string(), false)]
                    .into_iter()
                    .collect::<BTreeMap::<String, bool>>(),
            ),
        ]
        .into_iter()
        .for_each(|m| {
            modules.insert(m.name(), m);
        });
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        vec![
            ("broadcaster".to_string(), "a".to_string()),
            ("broadcaster".to_string(), "b".to_string()),
            ("broadcaster".to_string(), "c".to_string()),
            ("a".to_string(), "b".to_string()),
            ("b".to_string(), "c".to_string()),
            ("c".to_string(), "inv".to_string()),
            ("inv".to_string(), "a".to_string()),
        ]
        .into_iter()
        .for_each(|(k, v)| connections.entry(k).or_default().push(v));

        assert_eq!(
            input,
            InputModel {
                circuit: Circuit {
                    modules,
                    connections
                }
            }
        );
    }

    #[test]
    fn test_broadcaster() {
        let mut input = InputModel::from_str(TEST_INPUT).unwrap();
        let in_pulses = vec![Pulse::low("button", "broadcaster")];
        let out_pulses = input.circuit.step(in_pulses);
        assert_eq!(
            out_pulses,
            vec![
                Pulse::low("broadcaster", "a"),
                Pulse::low("broadcaster", "b"),
                Pulse::low("broadcaster", "c"),
            ]
        );
    }

    #[test]
    fn test_flipflop() {
        let mut input = InputModel::from_str(TEST_INPUT).unwrap();
        let in_pulses = vec![Pulse::low("broadcaster", "a")];
        let out_pulses = input.circuit.step(in_pulses.clone());
        let out_pulses_2 = input.circuit.step(in_pulses);
        assert_eq!(out_pulses, vec![Pulse::high("a", "b"),]);
        assert_eq!(out_pulses_2, vec![Pulse::low("a", "b"),]);
        let in_pulses = vec![Pulse::high("broadcaster", "a")];
        let out_pulses = input.circuit.step(in_pulses.clone());
        let out_pulses_2 = input.circuit.step(in_pulses);
        assert_eq!(out_pulses, vec![]);
        assert_eq!(out_pulses_2, vec![]);
    }

    #[test]
    fn test_inverter() {
        let mut input = InputModel::from_str(TEST_INPUT).unwrap();
        let in_pulses = vec![Pulse::low("c", "inv")];
        let out_pulses = input.circuit.step(in_pulses);
        assert_eq!(out_pulses, vec![Pulse::high("inv", "a"),]);
        let in_pulses = vec![Pulse::high("c", "inv")];
        let out_pulses = input.circuit.step(in_pulses);
        assert_eq!(out_pulses, vec![Pulse::low("inv", "a"),]);
    }

    const TEST_PULSES: &str = "button -low-> broadcaster
broadcaster -low-> a
broadcaster -low-> b
broadcaster -low-> c
a -high-> b
b -high-> c
c -high-> inv
inv -low-> a
a -low-> b
b -low-> c
c -low-> inv
inv -high-> a";

    #[test]
    fn test_display_pulse() {
        let in_pulse = Pulse::low("button", "broadcaster");
        let actual = format!("{}", in_pulse);
        let expected = TEST_PULSES.lines().next().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_push_button() {
        let mut input = InputModel::from_str(TEST_INPUT).unwrap();
        let in_pulses = input.circuit.push_button();
        let actual = in_pulses
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(actual, TEST_PULSES);
    }

    const ALT_TEST_INPUT: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    const ALT_PULSES_1: &str = "button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -high-> output
b -high-> con
con -low-> output";
    const ALT_PULSES_2: &str = "button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output";
    const ALT_PULSES_3: &str = "button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -low-> output
b -low-> con
con -high-> output";
    const ALT_PULSES_4: &str = "button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output";

    #[test]
    fn test_alt_push_button() {
        let mut input = InputModel::from_str(ALT_TEST_INPUT).unwrap();
        let in_pulses = input.circuit.push_button();
        let actual = in_pulses
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(actual, ALT_PULSES_1);
        let in_pulses = input.circuit.push_button();
        let actual = in_pulses
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(actual, ALT_PULSES_2);
        let in_pulses = input.circuit.push_button();
        let actual = in_pulses
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(actual, ALT_PULSES_3);
        let in_pulses = input.circuit.push_button();
        let actual = in_pulses
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(actual, ALT_PULSES_4);
    }
}
