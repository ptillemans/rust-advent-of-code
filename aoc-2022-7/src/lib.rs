use std::str::FromStr;
use nom::{IResult, Parser, 
    character::complete::*,
    bytes::complete::tag,
    branch::*,
    sequence::*,
    multi::*,
    combinator::*,
};

#[derive(Debug, PartialEq, Eq)]
pub enum FileSystemNode {
    File{name: String, size: usize},
    Directory{name: String, contents: Vec<FileSystemNode>},
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    CD (String),
    CDUP,
    CDROOT,
    LS (Vec<FileSystemNode>),
}



#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub commands: Vec<Command>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        separated_list1(newline, command).parse(s)
            .map(|(_, commands)| InputModel { commands })
            .map_err(|_| AocError::ParseError)
    }
}

fn file_name_parser(input: &str) -> IResult<&str, String> {
    many1(alt((alphanumeric1, tag("."))))
        .map(|v: Vec<&str>| v.into_iter().collect::<String>())
        .parse(input)
}

fn file_parser(input: &str) -> IResult<&str, FileSystemNode> {
    tuple((map_res(digit1, str::parse), space1, file_name_parser))
        .map (|(size, _, name)| {
            FileSystemNode::File{name, size}
        })
        .parse(input)
}

fn directory_parser(input: &str) -> IResult<&str, FileSystemNode> {
    tuple((tag("dir"), space1::<&str, _>, alpha1))
        .map (|(_, _, name)| {
            FileSystemNode::Directory {name: name.to_string(), contents: vec![]}
        })
        .parse(input)
}

fn ls_output(output: &str) -> IResult<&str, Vec<FileSystemNode>> {
    separated_list0(newline, alt(( file_parser, directory_parser,)))
        .parse(output)
}

fn command(input: &str) -> IResult<&str, Command> {
   preceded(
        tag("$ "),
        alt((
            preceded(terminated(tag("ls"),newline), ls_output).map(Command::LS),
            tag("cd ..").map(|_| Command::CDUP),
            tag("cd /").map(|_| Command::CDROOT),
            preceded(tag("cd "), file_name_parser).map(Command::CD),
        )))
    .parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_ls_ouput() {
        let input = "dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a";
        let (rest, actual) = ls_output(input).unwrap();
        let expected = vec![
            FileSystemNode::Directory{name: "a".to_string(), contents: vec![]},
            FileSystemNode::File{name: "b.txt".to_string(), size: 14848514},
            FileSystemNode::File{name: "c.dat".to_string(), size: 8504156},
            FileSystemNode::Directory{name: "d".to_string(), contents: vec![]},
        ];

        println!("rest: {}", rest);
        println!("actual: {:?}", actual);
        assert_eq!(rest, "\n$ cd a");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_ls_command() {
        let input = "$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a";
        let (rest, actual) = command(input).unwrap();
        let expected = Command::LS(vec![
            FileSystemNode::Directory{name: "a".to_string(), contents: vec![]},
            FileSystemNode::File{name: "b.txt".to_string(), size: 14848514},
            FileSystemNode::File{name: "c.dat".to_string(), size: 8504156},
            FileSystemNode::Directory{name: "d".to_string(), contents: vec![]},
        ]);

        println!("rest: {}", rest);
        println!("actual: {:?}", actual);
        assert_eq!(rest, "\n$ cd a");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_cd_command() {
        let input = "$ cd a
$ ls";
        let (rest, actual) = command(input).unwrap();
        let expected = Command::CD("a".to_string());

        println!("rest: {}", rest);
        println!("actual: {:?}", actual);
        assert_eq!(rest, "\n$ ls");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_cd_up_command() {
        let input = "$ cd ..
$ ls";
        let (rest, actual) = command(input).unwrap();
        let expected = Command::CDUP;

        println!("rest: {}", rest);
        println!("actual: {:?}", actual);
        assert_eq!(rest, "\n$ ls");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_cd_root_command() {
        let input = "$ cd /
$ ls";
        let (rest, actual) = command(input).unwrap();
        let expected = Command::CDROOT;

        println!("rest: {}", rest);
        println!("actual: {:?}", actual);
        assert_eq!(rest, "\n$ ls");
        assert_eq!(actual, expected);
    }

}
