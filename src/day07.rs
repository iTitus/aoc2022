use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum FileType {
    File { size: usize },
    Dir,
}

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    file_type: FileType,
    children: Vec<Node>,
}

impl Node {
    pub fn new_file(name: impl Into<String>, size: usize) -> Node {
        Node {
            name: name.into(),
            file_type: FileType::File { size },
            children: vec![],
        }
    }

    pub fn new_dir(name: impl Into<String>) -> Node {
        Node {
            name: name.into(),
            file_type: FileType::Dir,
            children: vec![],
        }
    }

    pub fn add_child<S: AsRef<str>>(&mut self, path: &[S], node: Node) -> Result<(), ()> {
        if !self.is_dir() {
            return Err(());
        }

        if path.is_empty() {
            if self.children.iter().any(|c| c.name == node.name) {
                return Err(());
            }

            self.children.push(node);
            Ok(())
        } else {
            let child_path = path[0].as_ref();
            match self
                .children
                .iter_mut()
                .filter(|c| c.name == child_path)
                .exactly_one()
            {
                Ok(child) => child.add_child(&path[1..], node),
                Err(_) => Err(()),
            }
        }
    }

    pub fn size(&self) -> usize {
        match self.file_type {
            FileType::File { size } => size,
            FileType::Dir => self.children.iter().map(|c| c.size()).sum(),
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.file_type, FileType::Dir)
    }
}

#[derive(Debug, Clone)]
struct CommandInfo {
    args: Vec<String>,
    output: Vec<String>,
}

#[derive(Debug, Clone)]
enum Command {
    Cd(CommandInfo),
    Ls(CommandInfo),
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Node {
    let commands = input
        .lines()
        .map(|l| l.trim())
        .peekable()
        .batching(|it| match it.next() {
            Some(l1) => {
                if !l1.starts_with('$') {
                    panic!("expected command starting with '$' but got '{l1}'");
                }

                let mut iter = l1[2..].split_whitespace();
                let cmd = iter.next().unwrap();
                let args = iter.map(|s| s.to_string()).collect_vec();
                let output = it
                    .peeking_take_while(|l| !l.starts_with('$'))
                    .map(|l| l.to_string())
                    .collect_vec();

                Some(match cmd {
                    "cd" => Command::Cd(CommandInfo { args, output }),
                    "ls" => Command::Ls(CommandInfo { args, output }),
                    _ => panic!("unknown command {cmd}"),
                })
            }
            None => None,
        })
        .collect_vec();

    let first_cmd = &commands[0];
    if !matches!(first_cmd, Command::Cd(CommandInfo {args, output:_}) if args.len() == 1 && args[0] == "/")
    {
        panic!("expected 'cd /' in first command but got {first_cmd:?}")
    }

    let mut root = Node::new_dir("");
    let mut current_path: Vec<&str> = vec![];
    for cmd in &commands[1..] {
        match cmd {
            Command::Cd(CommandInfo { args, output: _ }) => {
                if args.len() != 1 {
                    panic!("expected ! arg for cd, got {args:?}");
                }

                let arg = args[0].as_str();
                match arg {
                    "/" => current_path.clear(),
                    ".." => {
                        current_path.pop().unwrap();
                    }
                    name if !name.starts_with('/') => current_path.push(name),
                    _ => panic!("unknown arg for cd: '{arg}'"),
                };
            }
            Command::Ls(CommandInfo { args: _, output }) => {
                for entry in output {
                    let (prefix, name) = entry.split_once(' ').unwrap();
                    match prefix {
                        "dir" => root.add_child(&current_path, Node::new_dir(name)).unwrap(),
                        size_str => {
                            let size = usize::from_str(size_str).unwrap();
                            root.add_child(&current_path, Node::new_file(name, size))
                                .unwrap()
                        }
                    }
                }
            }
        }
    }

    root
}

#[aoc(day7, part1)]
pub fn part1(root: &Node) -> usize {
    let mut sum = 0usize;
    let mut q = vec![root];
    while let Some(n) = q.pop() {
        if n.is_dir() {
            let size = n.size();
            if size <= 100_000 {
                sum += size;
            }

            n.children.iter().for_each(|c| q.push(c));
        }
    }

    sum
}

#[aoc(day7, part2)]
pub fn part2(root: &Node) -> usize {
    let capacity = 70000000usize;
    let required = 30000000usize;
    let used = root.size();
    let free = capacity - used;

    if free >= required {
        return 0;
    }

    let mut node_to_free = usize::MAX;
    let mut q = vec![root];
    while let Some(n) = q.pop() {
        if n.is_dir() {
            let size = n.size();
            if free + size >= required {
                if size < node_to_free {
                    node_to_free = size;
                }

                n.children.iter().for_each(|c| q.push(c));
            }
        }
    }

    node_to_free
}
