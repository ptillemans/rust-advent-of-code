#![feature(test)]
use aoc_2022_7::*;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &InputModel) -> Result<String,AocError> {
    let mut shell = Shell::new();
    shell.execute(input.commands.clone())?;
    let solution: usize = shell.flat_node_list().into_iter()
        .filter(|node| node.is_dir())
        .map(|node| node.total_size())
        .filter(|size| *size <= 100000)
        .sum();
    Ok(solution.to_string())
}

const VOLUME_SIZE: usize = 70000000;
const UPGRADE_SIZE: usize = 30000000;

fn part2(input: &InputModel) -> Result<String, AocError> {
    let mut shell = Shell::new();
    shell.execute(input.commands.clone())?;
    let total_size = shell.get_root().total_size();
    let needed_space = total_size + UPGRADE_SIZE - VOLUME_SIZE;

    let solution = shell.flat_node_list().into_iter()
        .filter(|node| node.is_dir())
        .map(|node| node.total_size())
        .filter(|size| *size >= needed_space)
        .min()
        .ok_or(AocError::NoSolution)?;

   return Ok(solution.to_string())
}

fn main() -> Result<(), AocError> {
    let input:InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    pub fn input_data() -> InputModel {
        InputModel {
            commands: vec![
                Command::CDROOT,
                Command::LS(vec![
                    FileSystemNode::new_dir("a", vec![]),
                    FileSystemNode::new_file("b.txt", 14848514),
                    FileSystemNode::new_file("c.dat", 8504156),
                    FileSystemNode::new_dir("d", vec![]),
                ]),
                Command::CD("a".to_string()),
                Command::LS(vec![
                    FileSystemNode::new_dir("e", vec![]),
                    FileSystemNode::new_file("f", 29116),
                    FileSystemNode::new_file("g", 2557),
                    FileSystemNode::new_file("h.lst", 62596),
                ]),
                Command::CD("e".to_string()),
                Command::LS(vec![
                    FileSystemNode::new_file("i", 584),
                ]),
                Command::CDUP,
                Command::CDUP,
                Command::CD("d".to_string()),
                Command::LS(vec![
                    FileSystemNode::new_file("j", 4060174),
                    FileSystemNode::new_file("d.log", 8033020),
                    FileSystemNode::new_file("d.ext", 5626152),
                    FileSystemNode::new_file("k", 7214296),
                ]),
            ]
        }
    }

    const FS_OUTPUT: &str = "- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)";

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_file_system() {
        let mut shell = Shell::new();
        shell.execute(input_data().commands.clone()).unwrap();
        let actual = shell.get_root().format_with_indent("");
        assert_eq!(actual, FS_OUTPUT);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "95437";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "24933642";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input:InputModel = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input:InputModel = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&input))
    }

}
