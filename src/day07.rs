use std::{collections::HashMap, fmt::Display};

type Treeid = usize;

enum Storage {
    File(StorageFile),
    Directory(StorageDirectory)
}
struct StorageFile{
    size: u32,
    name: String
}

struct StorageDirectory{
    size: u32,
    name: String,
    children: Vec<Storage>
}

struct Tree {
    storage: Vec<Storage>
}

#[derive(Debug, PartialEq)]
enum Output<'a> {
    CDRoot,
    CDBack,
    CDSubdir(&'a str),
    LS,
    Directory(&'a str),
    File(&'a str, u32)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Path {
    path: Vec<String>
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl Output<'_> {
    fn lex(input: &str) -> Output {
        if !input.starts_with('$') {
            let space_pos = input.find(' ').unwrap();
            let name = &input[space_pos+1..input.len()];
            if input.as_bytes()[0].is_ascii_digit() {
                let size = input[0..space_pos].parse().unwrap();
                Output::File(name, size)
            } else {
                Output::Directory(name)
            }
        } else if input.eq("$ cd /") {
            Output::CDRoot
        } else if input.eq("$ cd ..") {
            Output::CDBack
        }else if input.eq("$ ls") {
            Output::LS
        } else {
            Output::CDSubdir(&input[5..input.len()])
        }
    }
}

impl Tree {
    fn parse(input: &Vec<String>) -> Vec<(Path, u32)> {
        let mut files: Vec<(Path, u32)> = vec![];
        let mut pwd = vec![String::from("/")];
        for line in input {
            let output = Output::lex(line);
            match output {
                Output::CDSubdir(x) => pwd.push(String::from(x)),
                Output::CDBack => {pwd.pop();},
                Output::CDRoot => pwd = vec![String::from("/")],
                Output::File(n, s) => {
                    let mut path = pwd.clone();
                    path.push(String::from(n));
                    let file = (Path{path}, s);
                    files.push(file);
                },
                _ => {}
            }
        }
        return files;
    }
}

impl Path {
    pub fn getParents(&self) -> Vec<Path> {
        let mut parents = vec![];
        let mut tmp = vec![];
        for directory in &self.path {
            tmp.push(directory.clone());
            parents.push(Path{path:tmp.clone()});
        }
        return parents;
    }
}

pub fn solve_1(input: &Vec<String>) -> u32 {
    let parsed_files = Tree::parse(input);
    let map = generate_parent_map(&parsed_files);
    let mut result = 0;
    for x in map.values() {
        if *x <= 100000 {
            result += x;
        }
    }
    return result;
}

fn generate_parent_map(parsed_files: &Vec<(Path, u32)>) -> HashMap<&Path, u32> {
    let mut map = HashMap::new();
    let mut counter_contained = 0;
    let mut counter_new = 0;
    for (path, size) in parsed_files {
        for parent in path.getParents() {
            if map.contains_key(&parent) {
                let old = map.get(&parent).unwrap();
                map.insert(path, old + size);
                counter_contained += 1;
            } else {
                map.insert(path, *size);
                counter_new += 1;
            }
        }
    }
    println!("counter_contained: {}", counter_contained);
    println!("counter_new: {}", counter_new);
    return map;
}

pub fn solve_2(input: &Vec<String>) -> u32 {
todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::*;
    use super::*;

    #[test]
    fn correct_outputs() {
        assert_eq!(Output::LS, Output::lex("$ ls"));
        assert_eq!(Output::CDRoot, Output::lex("$ cd /"));
        assert_eq!(Output::CDBack, Output::lex("$ cd .."));
        assert_eq!(Output::CDSubdir("a"), Output::lex("$ cd a"));
        assert_eq!(Output::Directory("a"), Output::lex("dir a"));
        assert_eq!(Output::File("f", 29116), Output::lex("29116 f"));
    }

    #[test]
    fn generate_solution_1_test_input() {
        let input = get_test_input(7);
        let actual = solve_1(&input);
        assert_eq!(95437, actual);
    }

    #[test]
    fn parse_tree() {
        let input = get_test_input(7);
        let actual = Tree::parse(&input);
        let expected_last_path: (Path, u32) = (Path{path:vec![String::from("/"), String::from("d"), String::from("k")]}, 7214296);
        assert_eq!(10, actual.len());
        assert_eq!(expected_last_path, *actual.last().unwrap());
    }

}