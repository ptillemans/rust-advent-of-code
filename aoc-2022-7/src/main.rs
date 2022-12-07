#![feature(test)]
use aoc_2022_7::*;

const INPUT: &str = include_str!("../data/input.txt");


fn part1(_input: &InputModel) -> Result<String,AocError> {
    return Ok("Not implemented".to_string())
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    return Ok("Not implemented".to_string())
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
                    FileSystemNode::Directory{name: "a".to_string(), contents: vec![]},
                    FileSystemNode::File{name: "b.txt".to_string(), size: 14848514},
                    FileSystemNode::File{name: "c.dat".to_string(), size: 8504156},
                    FileSystemNode::Directory{name: "d".to_string(), contents: vec![]},
                ]),
                Command::CD("a".to_string()),
                Command::LS(vec![
                    FileSystemNode::Directory{name: "e".to_string(), contents: vec![]},
                    FileSystemNode::File{name: "f".to_string(), size: 29116},
                    FileSystemNode::File{name: "g".to_string(), size: 2557},
                    FileSystemNode::File{name: "h.lst".to_string(), size: 62596},
                ]),
                Command::CD("e".to_string()),
                Command::LS(vec![
                    FileSystemNode::File{name: "i".to_string(), size: 584},
                ]),
                Command::CDUP,
                Command::CDUP,
                Command::CD("d".to_string()),
                Command::LS(vec![
                    FileSystemNode::File{name: "j".to_string(), size: 4060174},
                    FileSystemNode::File{name: "d.log".to_string(), size: 8033020},
                    FileSystemNode::File{name: "d.ext".to_string(), size: 5626152},
                    FileSystemNode::File{name: "k".to_string(), size: 7214296},
                ]),
            ]
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
        let expected = "";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(&input_data()))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(&input_data()))
    }

}
