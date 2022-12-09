use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Position{
    pub x:usize,
    pub y:usize
}

impl Position {
    pub fn new(x: usize, y:usize) -> Position {
        Position{x, y}
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PositionSigned {
    pub x: i32,
    pub y: i32
}

impl PositionSigned {
    pub fn new(x: i32, y: i32) -> PositionSigned{
        PositionSigned {x,y}
    }

    pub fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn build_input_path(file_name: &str, day: i8) -> String {
    let base_path = "./input/";
    let mut file = String::from(base_path);
    file.push_str(file_name);
    let mut day_str = day.to_string();
    if day_str.len() < 2 {
        day_str.insert(0, '0');
    }
    file.push_str(day_str.as_str());
    file.push_str(".txt");
    return file;
}

pub fn into_lines_vec(input: &String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn get_input(day: i8) -> Vec<String> {
    let file = build_input_path("day", day);
    let content = fs::read_to_string(file)
        .expect("Could not find File");

    let splitted = into_lines_vec(&content);

    return splitted;
}

pub fn get_test_input(day: i8) -> Vec<String> {
    let file = build_input_path("test", day);
    let content = fs::read_to_string(file)
        .expect("Could not find File");

    let splitted = into_lines_vec(&content);

    return splitted;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_input_path() {
        assert_eq!(String::from("./input/day01.txt"), build_input_path("day", 1));
        assert_eq!(String::from("./input/day22.txt"), build_input_path("day", 22));
        assert_eq!(String::from("./input/test01.txt"), build_input_path("test", 1));
        assert_eq!(String::from("./input/test22.txt"), build_input_path("test", 22));
    }

    #[test]
    fn test_get_input() {
        let content = get_input(1);
        assert!(content.len() > 10);
    }
}