use std::str::FromStr;
use std::ops::Add;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    cubes: Vec<Pos>
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s.lines()
            .map(Pos::from_str)
            .collect::<Result<Vec<Pos>, AocError>>()?;
        Ok(InputModel{ cubes })
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos (i32, i32, i32);

impl FromStr for Pos {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> = s.split(',')
            .map(|p| p.parse::<i32>().unwrap())
            .collect();
        if coords.len() == 3 {
            Ok(Pos(coords[0], coords[1], coords[2]))
        } else {
            Err(AocError::ParseError)
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Pos(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Material {
    Air,
    Lava,
    Water,
}

pub struct Cube {
    matrix: [[[Material; Cube::WIDTH as usize]; Cube::WIDTH as usize]; Cube::WIDTH as usize],
    default: Material,
}

impl Cube {
    const WIDTH: i32 = 22;

    fn new(material: Material) -> Self {
        Cube {
            matrix: [[[material; Cube::WIDTH as usize]; Cube::WIDTH as usize]; Cube::WIDTH as usize],
            default: material,
        }
    }

    fn set_default(&mut self, material: Material) {
        self.default = material;
    }

    fn get(&self, pos: &Pos) -> Material {
        let Pos(x, y, z) = pos;
        if *x < 0 || *x >= Cube::WIDTH || *y < 0 || *y >= Cube::WIDTH || *z < 0 || *z >= Cube::WIDTH {
            self.default
        } else {
            self.matrix[*x as usize][*y as usize][*z as usize]
        }
    }

    fn set(&mut self, pos: &Pos, value: Material) {
        let Pos(x, y, z) = pos;
        if *x < 0 || *x >= Cube::WIDTH || *y < 0 || *y >= Cube::WIDTH || *z < 0 || *z >= Cube::WIDTH {
            panic!("Out of bounds{pos:?}");
        } else {
            self.matrix[*x as usize][*y as usize][*z as usize] = value;
        };
    }
    
}

const NEIGHBOURS: &[Pos] = &[
    Pos(0, 0, -1), Pos(0, 0, 1),
    Pos(0, -1, 0), Pos(0, 1, 0),
    Pos(-1, 0, 0), Pos(1, 0, 0),
];


pub fn soak(cube: &mut Cube) {
    cube.set_default(Material::Water);
    // make a list of positions just outside the cube
    let mut open: Vec<Pos> = (0..Cube::WIDTH)
        .flat_map(|i| (0..Cube::WIDTH).map(move |j| (i, j)))
        .flat_map(|(i, j)| vec![
                  Pos(-1, i, j), Pos(Cube::WIDTH, i, j), 
                  Pos(i, -1, j), Pos(i, Cube::WIDTH, j),
                  Pos(i, j, -1), Pos(i, j, Cube::WIDTH), 
        ])
        .collect();
    let seen: HashSet<Pos> = HashSet::from_iter(open.iter().cloned());
    while !open.is_empty() {
        let pos = open.pop().unwrap();
        for neighbour in NEIGHBOURS {
            let new_pos = pos + *neighbour;
            if cube.get(&new_pos) == Material::Air {
                cube.set(&new_pos, Material::Water);
                if !seen.contains(&new_pos) {
                    open.push(new_pos);
                }
            }
        }
    }
}

pub fn exposed_surface(input: &InputModel) -> i32 {
    let mut matrix = Cube::new(Material::Air);

    for cube in &input.cubes {
        matrix.set(cube, Material::Lava);
    }

    let mut covered_sides = 0;
    for cube in &input.cubes {
        for neighbour in NEIGHBOURS {
            let pos: Pos = *cube + *neighbour;
            if matrix.get(&pos) == Material::Lava {
                covered_sides += 1;
            }
        }
    }

    let surface = (input.cubes.len() * 6) - covered_sides;
    surface as i32
}


pub fn exposed_to_water(input: &InputModel) -> i32 {
    let mut matrix = Cube::new(Material::Air);

    for cube in &input.cubes {
        matrix.set(cube, Material::Lava);
    }

    soak(&mut matrix);

    let mut exposed_sides = 0;
    for cube in &input.cubes {
        for neighbour in NEIGHBOURS {
            let pos: Pos = *cube + *neighbour;
            if matrix.get(&pos) == Material::Water {
                exposed_sides += 1;
            }
        }
    }

    exposed_sides
}

pub const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

pub fn input_data() -> InputModel {
    InputModel { cubes: vec![    
        Pos(2,2,2),
        Pos(1,2,2),
        Pos(3,2,2),
        Pos(2,1,2),
        Pos(2,3,2),
        Pos(2,2,1),
        Pos(2,2,3),
        Pos(2,2,4),
        Pos(2,2,6),
        Pos(1,2,5),
        Pos(3,2,5),
        Pos(2,1,5),
        Pos(2,3,5),
    ]}
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_pos_parser() {
        let actual: InputModel = TEST_INPUT.parse().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_exposed_surface() {
        let input = input_data();
        let actual = exposed_surface(&input);
        let expected = 64;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_exposed_to_water() {
        let input = input_data();
        let actual = exposed_to_water(&input);
        let expected = 58;

        assert_eq!(actual, expected);
    }
}
