use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub instructions: Vec<String>,
}


#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Add(u32, String, u32),
    Remove(u32, String)
}


#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.trim().lines()
            .flat_map(|l| l.split(','))
            .map(|s| s.to_string())
            .collect();
        Ok(InputModel { instructions })
    }
}

pub fn hash(s: &str) -> u32 {
    s.as_bytes().iter()
        .fold(0, |acc, &b| ((acc + b as u32) * 17) % 256 )

    
}

pub fn parse_instruction(s: &str) -> Instruction {
    let parts = s.split('=').collect::<Vec<_>>();
    if parts.len() == 2 {
        let label = parts[0].to_string();
        let slot = parts[1].parse::<u32>().unwrap();
        let _box = hash(&label);
        Instruction::Add(_box, label, slot)
    } else {
        let label = s.strip_suffix('-').unwrap().to_string();
        let _box = hash(&label);
        Instruction::Remove(_box, label)
    }
}

pub fn execute(instructions: &[String]) -> Vec<Vec<(String, u32)>>{
    let mut boxes = vec![Vec::new(); 256];
    instructions.iter()
        .map(|s| parse_instruction(s))
        
        .for_each(|i| match i {
            Instruction::Add(b, l, s) => {
                let b = &mut boxes[b as usize];
                if let Some((_, v)) = b.iter_mut().find(|(k, _)| k == &l) {
                    *v = s;
                } else {
                    b.push((l, s));
                }
            },
            Instruction::Remove(b, l) => {
                let b = &mut boxes[b as usize];
                if let Some((i, _)) = b.iter().enumerate().find(|(_, (k, _))| k == &l) {
                    b.remove(i);
                }
                
            }
        });
    boxes
}

pub fn focussing_power(boxes: &[Vec<(String, u32)>]) -> u32 {
    boxes.iter()
        .enumerate()
        .map(|(box_no, b)| (box_no as u32 + 1) * b.iter()
             .enumerate()
             .map(|(slot, (_, v))| (slot as u32 + 1) * v)
             .sum::<u32>())
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "R1,U2,L3,D4";
        let expected = InputModel {
            instructions: vec!["R1".to_string(), "U2".to_string(), "L3".to_string(), "D4".to_string()],
        };
        assert_eq!(expected, input.parse().unwrap());
    }

    #[test]
    fn test_hash() {
        let string = "HASH";
        assert_eq!(hash(string), 52 );
    }

    #[test]
    fn test_parse_instruction() {
        let input = "rn=1";
        let expected = Instruction::Add(0, "rn".to_string(), 1);
        assert_eq!(expected, parse_instruction(input));

        let input = "qp-";
        let expected = Instruction::Remove(1, "qp".to_string());
        assert_eq!(expected, parse_instruction(input));

    }
}
