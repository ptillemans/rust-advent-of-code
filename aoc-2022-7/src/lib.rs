use std::str::FromStr;
use nom::{IResult, Parser, 
    character::complete::*,
    bytes::complete::tag,
    branch::*,
    sequence::*,
    multi::*,
    combinator::*,
};

#[derive(Debug, PartialEq, Eq, Clone)]  
pub enum NodeType {
    File(usize),
    Directory(Vec<FileSystemNode>)
}

impl NodeType {
    fn set_contents(&mut self, contents: Vec<FileSystemNode>) {
        if let NodeType::Directory(ref mut dir) = self {
            *dir = contents.into_iter().collect();
        }
    }

    fn new_file(size: usize) -> Self {
        NodeType::File(size)
    }

    fn new_dir(contents: Vec<FileSystemNode>) -> Self {
        NodeType::Directory(contents.into_iter().collect())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]  
pub struct FileSystemNode {
    name: String,
    node_type: NodeType,
}

impl FileSystemNode {

    pub fn new_file(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            node_type: NodeType::new_file(size),
        }
    }

    pub fn new_dir(name: &str, contents: Vec<FileSystemNode>) -> Self {
        Self {
            name: name.to_string(),
            node_type: NodeType::new_dir(contents),
        }
    }

    pub fn is_dir(&self) -> bool {
        match self.node_type {
            NodeType::File(_) => false,
            NodeType::Directory(_) => true,
        }
    }

    fn set_contents(&mut self, new_contents: Vec<FileSystemNode>) -> Result<(), AocError> {
        if self.is_dir() {
            self.node_type.set_contents(new_contents);
            Ok(())
        } else {
            Err(AocError::IllegalState)
        }
    }
    
    pub fn total_size(&self) -> usize {
        match self.node_type {
            NodeType::File(size) => size,
            NodeType::Directory(ref contents) => {
                contents.iter().map(|node| node.total_size()).sum()
            }
        }
    }

    pub fn format_with_indent(&self, indent: &str) -> String {
        match self.node_type {
            NodeType::File(size) => format!("{}- {} (file, size={})", indent, self.name, size),
            NodeType::Directory(ref contents) => {
                let dir_name = format!("{}- {} (dir)", indent, self.name);
                let dir_contents = contents.iter()
                    .map(|node| node.format_with_indent(&format!("{}  ", indent)));
                vec![dir_name].into_iter()
                    .chain(dir_contents)
                    .collect::<Vec<String>>().join("\n")
            }
        }
    }

    fn find_path(&self, path: &[String]) -> Result<&FileSystemNode,AocError> {
        if path.len() == 0 {
            return Err(AocError::NoSuchDirectory("???".to_string()));
        }
        let (first, rest) = path.split_first().unwrap();
        if first != &self.name {
            return Err(AocError::IllegalState)
        };
        if rest.len() == 0 {
            return Ok(self);
        }
        let (next, _) = rest.split_first().unwrap();
        match self.node_type {
            NodeType::File(_) => Err(AocError::IllegalState),
            NodeType::Directory(ref contents) => {
                contents.iter()
                    .find(|node| node.name == *next)
                    .ok_or(AocError::IllegalState)
                    .and_then(|node| node.find_path(rest))
            }
        }
    }

    fn find_path_mut(&mut self, path: &[String]) -> Result<&mut FileSystemNode,AocError> {
        if path.len() == 0 {
            return Err(AocError::NoSuchDirectory("???".to_string()));
        }
        let (first, rest) = path.split_first().unwrap();
        if first != &self.name {
            return Err(AocError::IllegalState)
        };
        if rest.len() == 0 {
            return Ok(self);
        }
        let (next, _) = rest.split_first().unwrap();
        match self.node_type {
            NodeType::File(_) => Err(AocError::IllegalState),
            NodeType::Directory(ref mut contents) => {
                contents.iter_mut()
                    .find(|node| node.name == *next)
                    .ok_or(AocError::IllegalState)
                    .and_then(|node| node.find_path_mut(rest))
            }
        }

    }

    fn flat_list(&self) -> Vec<FileSystemNode> {
        match self.node_type {
            NodeType::File(_) => vec![self.clone()],
            NodeType::Directory(ref contents) => {
                vec![self.clone()].into_iter()
                     .chain(contents.iter()
                        .flat_map(|node| node.flat_list()))
                     .collect()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
    #[error("No solution found")]
    NoSolution,
    #[error("Illegal state in interpreter")]
    IllegalState,
    #[error("No such directory {0}")]
    NoSuchDirectory(String), 
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
            FileSystemNode::new_file(&name,size)
        })
        .parse(input)
}

fn directory_parser(input: &str) -> IResult<&str, FileSystemNode> {
    tuple((tag("dir"), space1::<&str, _>, alpha1))
        .map (|(_, _, name)| {
            FileSystemNode::new_dir(name, vec![])
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

#[derive(Debug, PartialEq, Eq)]
pub struct Shell {
    root: FileSystemNode,
    path: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        let root = FileSystemNode::new_dir("/", vec![]);
        let path = vec![root.name.clone()];
        Shell { root, path }
    }

    pub fn get_root(&self) -> &FileSystemNode {
        &self.root
    }

    pub fn get_path(&self, path: &Vec<String>) -> Result<&FileSystemNode, AocError> {
        self.root.find_path(path)
    }

    pub fn get_path_mut(&mut self, path: &Vec<String>) -> Result<&mut FileSystemNode, AocError> {
        self.root.find_path_mut(path)
    }

    pub fn execute(&mut self, commands: Vec<Command>) -> Result<(), AocError> {
        for command in commands {
            self.execute_command(command)?;
        }
        Ok(())
    }

    pub fn flat_node_list(&self) -> Vec<FileSystemNode> {
        self.root.flat_list()
    }

    fn execute_command(&mut self, command: Command) -> Result<(), AocError> {
        match command {
            Command::CD(name) => self.cd(&name),
            Command::CDUP => self.cdup(),
            Command::CDROOT => self.cdroot(),
            Command::LS(contents) => self.ls(contents),
        }
    }

    fn cd(&mut self, name: &str) -> Result<(), AocError> {
        let cur_dir_contents = self.get_path(&self.path)
            .and_then(|node| match node.node_type {
                NodeType::Directory(ref contents) => Ok(contents), 
                NodeType::File(_) => Err(AocError::IllegalState),
            })?;

        let new_dir = cur_dir_contents.into_iter()
                 .filter(|node| node.is_dir())
                 .find(|node| node.name == name)
                 .ok_or(AocError::NoSuchDirectory(name.to_string()))?;
        self.path.push(new_dir.name.clone());
        Ok(())
    }

    fn cdup(&mut self) -> Result<(), AocError> {
        self.path.truncate(self.path.len() - 1);
        Ok(())
    }

    fn cdroot(&mut self) -> Result<(), AocError> {
        self.path.truncate(1);
        Ok(())
    }
    

    fn ls(&mut self, contents: Vec<FileSystemNode>) -> Result<(), AocError> {
        let path = self.path.clone();
        self.get_path_mut(&path)
            .and_then(|node| node.set_contents(contents))?;
        Ok(())
    }
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
            FileSystemNode::new_dir("a", vec![]),
            FileSystemNode::new_file("b.txt", 14848514),
            FileSystemNode::new_file("c.dat", 8504156),
            FileSystemNode::new_dir("d", vec![]),
        ];

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
            FileSystemNode::new_dir("a", vec![]),
            FileSystemNode::new_file("b.txt", 14848514),
            FileSystemNode::new_file("c.dat", 8504156),
            FileSystemNode::new_dir("d", vec![]),
        ]);

        assert_eq!(rest, "\n$ cd a");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_cd_command() {
        let input = "$ cd a
$ ls";
        let (rest, actual) = command(input).unwrap();
        let expected = Command::CD("a".to_string());

        assert_eq!(rest, "\n$ ls");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_cd_up_command() {
        let input = "$ cd ..
$ ls";
        let (rest, actual) = command(input).unwrap();
        let expected = Command::CDUP;

        assert_eq!(rest, "\n$ ls");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_cd_root_command() {
        let input = "$ cd /
$ ls";
        let (rest, actual) = command(input).unwrap();
        let expected = Command::CDROOT;

        assert_eq!(rest, "\n$ ls");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_new_shell() {
        let shell = Shell::new();

        assert_eq!(shell.path.len(), 1);
        assert_eq!(&shell.path[0], "/");
    }
    
    #[test]
    fn test_shell_ls() {
        let mut shell = Shell::new();
        let test_contents = vec![
            FileSystemNode::new_dir("a", vec![]),
            FileSystemNode::new_file("b.txt", 14848514),
            FileSystemNode::new_file("c.dat", 8504156),
            FileSystemNode::new_dir("d", vec![]),
        ];
        let command = Command::LS(test_contents.clone());

        shell.execute(vec![command]).unwrap();

        assert_eq!(shell.path.len(), 1);
        assert_eq!(&shell.path[0], "/");
    }

    #[test]
    fn test_shell_ls_deeper() {
        let mut shell = Shell::new();
        let expected_root = 
            FileSystemNode::new_dir("/", vec![
                FileSystemNode::new_dir("a", vec![
                    FileSystemNode::new_dir("d", vec![]),
                ])
            ]);
        let test_content1 = vec![
            FileSystemNode::new_dir("a", vec![]),
        ];
        let test_content2 = vec![
            FileSystemNode::new_dir("d", vec![]),
        ];

        shell.execute(vec![
            Command::LS(test_content1.clone()),
            Command::CD("a".to_string()),
            Command::LS(test_content2.clone()),
        ]).unwrap();

        assert_eq!(shell.path.len(), 2);
        assert_eq!(&shell.path[0], "/");
        assert_eq!(&shell.path[1], "a");
        assert_eq!(shell.get_root(), &expected_root);
    }


    #[test]
    fn test_shell_cd() {
        let mut shell = Shell::new();
        let test_contents = vec![
            FileSystemNode::new_dir("a", vec![]),
            FileSystemNode::new_file("b.txt", 14848514),
            FileSystemNode::new_file("c.dat", 8504156),
            FileSystemNode::new_dir("d", vec![]),
        ];
        let commands = vec![
            Command::LS(test_contents.clone()),
            Command::CD("a".to_string()),
        ];

        shell.execute(commands).unwrap();

        assert_eq!(shell.path.len(), 2);
        assert_eq!(&shell.path[1], "a");
    }
    
    #[test]
    fn test_shell_cd_up() {
        let mut shell = Shell::new();
        let test_contents = vec![
            FileSystemNode::new_dir("a", vec![]),
            FileSystemNode::new_file("b.txt", 14848514),
            FileSystemNode::new_file("c.dat", 8504156),
            FileSystemNode::new_dir("d", vec![]),
        ];
        let commands = vec![
            Command::LS(test_contents.clone()),
            Command::CD("a".to_string()),
            Command::CDUP,
        ];

        shell.execute(commands).unwrap();

        assert_eq!(shell.path.len(), 1);
        assert_eq!(&shell.path[0], "/");
    }
    
    #[test]
    fn test_shell_cd_root() {
        let mut shell = Shell::new();
        let test_contents = vec![
            FileSystemNode::new_dir("a", vec![]),
            FileSystemNode::new_file("b.txt", 14848514),
            FileSystemNode::new_file("c.dat", 8504156),
            FileSystemNode::new_dir("d", vec![]),
        ];
        let commands = vec![
            Command::LS(test_contents.clone()),
            Command::CD("a".to_string()),
            Command::CDROOT,
        ];

        shell.execute(commands).unwrap();

        assert_eq!(shell.path.len(), 1);
        assert_eq!(&shell.path[0], "/");
    }
    
    #[test]
    fn test_shell_flat_node_list() {
        let mut shell = Shell::new();
        let expected_list = vec![
            FileSystemNode::new_dir("/", vec![
                FileSystemNode::new_dir("a", vec![
                    FileSystemNode::new_dir("d", vec![]),
                ])
            ]),
            FileSystemNode::new_dir("a", vec![
                FileSystemNode::new_dir("d", vec![]),
            ]),
            FileSystemNode::new_dir("d", vec![]),
        ];
        let test_content1 = vec![
            FileSystemNode::new_dir("a", vec![]),
        ];
        let test_content2 = vec![
            FileSystemNode::new_dir("d", vec![]),
        ];

        shell.execute(vec![
            Command::LS(test_content1.clone()),
            Command::CD("a".to_string()),
            Command::LS(test_content2.clone()),
        ]).unwrap();

        assert_eq!(shell.flat_node_list(), expected_list);
    }


}
