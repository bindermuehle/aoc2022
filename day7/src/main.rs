use std::{cell::RefCell, collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    IResult,
};

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    children: HashMap<String, NodeHandle>,
    parent: Option<NodeHandle>,
    size: u64,
}
impl Node {
    fn from_name(name: String) -> Self {
        Self {
            name,
            parent: None,
            children: HashMap::new(),
            size: 0,
        }
    }
    fn get_size(&self) -> u64 {
        if self.size > 0 {
            return self.size;
        }
        self.children
            .values()
            .map(|child| child.borrow().get_size())
            .sum()
    }
}

#[derive(Debug, PartialEq)]

enum Command {
    Cd(String),
    Ls(Vec<Node>),
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("cd ")(input)?;
    let (input, path) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?; // consume the newline
    Ok((input, Command::Cd(path.to_string())))
}

fn parse_ls_command(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("ls\n")(input)?;
    Ok((input, ()))
}

fn parse_ls_file(input: &str) -> IResult<&str, Node> {
    let (input, size) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, name) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?; // consume the newline

    Ok((
        input,
        Node {
            name: name.to_string(),
            size: size.parse().unwrap(),
            children: HashMap::new(),
            parent: None,
        },
    ))
}
fn parse_ls_dir(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?; // consume the newline
    Ok((input, Node::from_name(name.to_string())))
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = parse_ls_command(input)?;
    let (input, result) = nom::multi::many0(alt((parse_ls_dir, parse_ls_file)))(input)?;
    Ok((input, Command::Ls(result)))
}
fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    alt((parse_cd, parse_ls))(input)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    nom::multi::many0(parse_command)(input)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let root = Rc::new(RefCell::new(Node::from_name("/".to_string())));
    let mut current = root.clone();
    let (_, commands) = parse_commands(&input).unwrap();
    for command in commands {
        match command {
            Command::Cd(path) => match path.as_str() {
                "/" => (), // do nothing
                ".." => {
                    let parent = current.borrow().parent.clone().unwrap();
                    current = parent;
                }
                _ => {
                    let node = current.borrow().children.get(&path).unwrap().clone();
                    current = node;
                }
            },
            Command::Ls(results) => {
                for mut result in results {
                    result.parent = Some(current.clone());
                    current
                        .borrow_mut()
                        .children
                        .insert(result.name.clone(), Rc::new(RefCell::new(result)));
                }
            }
        }
    }
    let mut folders = get_folders(root);
    folders.sort_by(|(_, a), (_, b)| a.cmp(b));
    let (_, size) = folders.iter().find(|(_, size)| *size > 8381165).unwrap();
    println!("{}", size);
}

fn get_folders(node: NodeHandle) -> Vec<(String, u64)> {
    if node.borrow().size > 0 {
        return vec![];
    }
    let mut sizes = vec![];
    sizes.push((node.borrow().name.clone(), node.borrow().get_size()));
    for child in node.borrow().children.values() {
        let mut child_size = get_folders(child.clone());
        sizes.append(&mut child_size);
    }
    return sizes;
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{parse_cd, parse_ls, parse_ls_command, parse_ls_dir, parse_ls_file, Command};

    #[test]
    fn test_cd() {
        assert_eq!(parse_cd("cd /\n"), Ok(("", Command::Cd("/".to_string()))))
    }
    #[test]
    fn test_ls_command() {
        assert_eq!(parse_ls_command("ls\n"), Ok(("", ())));
    }
    #[test]
    fn test_parse_file() {
        assert_eq!(
            parse_ls_file("1234 file.txt\n"),
            Ok((
                "",
                crate::Node {
                    name: "file.txt".to_string(),
                    size: 1234,
                    children: HashMap::new(),
                    parent: None,
                }
            ))
        )
    }
    #[test]
    fn test_parse_dir() {
        assert_eq!(
            parse_ls_dir("dir dir1\n"),
            Ok(("", crate::Node::from_name("dir1".to_string())))
        )
    }
    #[test]
    fn test_ls() {
        assert_eq!(
            parse_ls("ls\ndir dir1\n1234 file.txt\n"),
            Ok((
                "",
                Command::Ls(vec![
                    crate::Node::from_name("dir1".to_string()),
                    (crate::Node {
                        name: "file.txt".to_string(),
                        size: 1234,
                        children: HashMap::new(),
                        parent: None,
                    })
                ])
            ))
        )
    }
    #[test]
    fn test_parse_commands() {
        assert_eq!(
            crate::parse_commands("$ cd /\n$ ls\ndir dir1\n1234 file.txt\n"),
            Ok((
                "",
                vec![
                    Command::Cd("/".to_string()),
                    Command::Ls(vec![
                        crate::Node::from_name("dir1".to_string()),
                        crate::Node {
                            name: "file.txt".to_string(),
                            size: 1234,
                            children: HashMap::new(),
                            parent: None,
                        }
                    ])
                ]
            ))
        )
    }
}
