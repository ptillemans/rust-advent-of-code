use std::str::FromStr;

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        parts.next()
            .and_then(|instruction|
                match instruction {
                    "noop" => Some(Instruction::Noop),
                    "addx" => parts.next()
                        .and_then(|x| x.parse::<i32>().ok())
                        .map(Instruction::AddX),
                    _ => None,
                })
            .ok_or(AocError::ParseError)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub instructions: Vec<Instruction>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
    #[error("No solution found")]
    NoSolutionFound,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<Vec<Instruction>, Self::Err> = s.lines()
            .map(|l| l.parse::<Instruction>())
            .collect();
        result.map(|instructions| InputModel { instructions })
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Registers {
    cycle: usize,
    pc: usize,
    x: i32,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            cycle: 1,
            pc: 0,
            x: 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn execute(&self, registers: &Registers) -> Registers {
        match self {
            Instruction::Noop => Registers {
                pc: registers.pc + 1,
                cycle: registers.cycle + 1,
                ..*registers
            },
            Instruction::AddX(x) => Registers{
                pc: registers.pc + 1,
                x: registers.x + x,
                cycle: registers.cycle + 2,
                ..*registers
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Computer {
    code: Vec<Instruction>,
    registers: Registers,
}

impl Computer {
    pub fn new(code: Vec<Instruction>) -> Self {
        Computer { code, registers: Registers::new() }
    }

    fn step(&mut self) {
        if self.is_halted() {
            return;
        }
        let instruction = &self.code[self.registers.pc];
        self.registers = instruction.execute(&self.registers);
    }

    fn is_halted(&self) -> bool {
        self.registers.pc >= self.code.len()
    }
    
    fn log_execution(&mut self) -> impl Iterator<Item=Registers> + '_ {
        vec![self.registers.clone()].into_iter()
            .chain(std::iter::from_fn(move || {
                if self.is_halted() {
                    None
                } else {
                    self.step();
                    Some(self.registers.clone())
                }
            }))
    }

    fn run_till_cycle(&mut self, cycle: usize) -> Option<Registers>{
        std::iter::from_fn(move || {
            if self.is_halted() {
                None
            } else {
                self.step();
                Some(self.registers.clone())
            }
        })
        .take_while(|r| r.cycle <= cycle)
        .last()
    }

    pub fn signal_strength_at_cycle(&mut self, cycle: usize) -> Option<i32> {
        self.run_till_cycle(cycle)
            .map(|r| r.x * cycle as i32)
    }

}


fn pixel_on(cycle: usize, x: i32) -> bool {
    let col = (cycle - 1) as i32 % 40;
    (col - x).abs() < 2
}

pub fn crt_output(code: &Vec<Instruction>) -> Result<String, AocError> {    
    let mut computer = Computer::new(code.clone());
    let mut crt = vec!['.';240];
    computer.log_execution()
        .reduce(|last_r, r| {
            for p in last_r.cycle..r.cycle {
                if pixel_on(p, last_r.x) {
                    crt[p-1] = '#';
                }
            };
            r
        });

    Ok(crt
       .chunks(40)
       .map(|chunk| chunk.iter().collect::<String>())
       .collect::<Vec<String>>()
       .join("\n"))
}


#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "noop
addx 3
addx -5";

    const LARGER_TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";


    fn test_input() -> InputModel {
        let instructions = vec![
            Instruction::Noop,
            Instruction::AddX(3),
            Instruction::AddX(-5),
        ];
        InputModel{instructions}
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = test_input();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_state_log() {
        let mut computer = Computer::new(test_input().instructions);
        let log = computer.log_execution().collect::<Vec<_>>();
        let expected = vec![
            Registers { cycle: 1, pc: 0, x: 1 },
            Registers { cycle: 2, pc: 1, x: 1 },
            Registers { cycle: 4, pc: 2, x: 4 },
            Registers { cycle: 6, pc: 3, x: -1 },
        ];
        assert_eq!(log, expected);
    }

    #[test]
    fn test_sample_cycle() {
        let code = LARGER_TEST_INPUT.parse::<InputModel>().unwrap().instructions;
        let mut computer = Computer::new(code);
        let samples = vec![
            (20, 21),
            (60, 19),
            (100, 18),
            (140, 21),
            (180, 16),
            (220, 18),
        ];
        for (cycle, expected_x) in samples {
            let actual = computer.run_till_cycle(cycle).unwrap();
            assert_eq!(actual.x, expected_x);
        }
    }

    #[test]
    fn test_signal_strength() {
        let code = LARGER_TEST_INPUT.parse::<InputModel>().unwrap().instructions;
        let mut computer = Computer::new(code);
        let samples = vec![
            (20, 420),
            (60, 1140),
            (100, 1800),
            (140, 2940),
            (180, 2880),
            (220, 3960),
        ];
        for (cycle, expected_signal) in samples {
            let actual = computer.signal_strength_at_cycle(cycle).unwrap();
            assert_eq!(actual, expected_signal);
        }
    }

    const TEST_CRT_IMAGE: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    
    #[test]
    fn test_crt_output() {
        let code = LARGER_TEST_INPUT.parse::<InputModel>().unwrap().instructions;
        let actual = crt_output(&code).unwrap();
        assert_eq!(actual, TEST_CRT_IMAGE);
    }
}
