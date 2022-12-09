use std::collections::{HashMap, HashSet};
use crate::util::{Direction, Position, PositionSigned};

struct Rope {
    head: PositionSigned,
    tail: PositionSigned
}

impl Rope  {
    pub fn new(x: i32, y: i32) -> Rope {
        let head = PositionSigned::new(x, y);
        Rope{head: head.clone() , tail: head}
    }

    pub fn move_head(&mut self, direction: &Direction) {
        let x_delta: i32 = match direction {
            Direction::LEFT => -1,
            Direction::RIGHT => 1,
            _ => 0
        };

        let y_delta = match direction {
            Direction::UP => -1,
            Direction::DOWN => 1,
            _ => 0
        };

        self.head.x += x_delta;
        self.head.y += y_delta;

        self.move_tail();
    }

    fn move_tail(&mut self) {
        let delta = self.get_head_tail_delta();
        let mov_x = if delta.x > 1 {
            1
        } else if delta.y < -1 {
            -1
        } else {
            0
        };

        let mov_y = if delta.x > 1 {
            1
        } else if delta.x < -1 {
            -1
        } else {
            0
        };

        self.tail.x += mov_x;
        self.tail.y += mov_y;
    }

    pub fn get_head_tail_delta(&self) -> PositionSigned{
        let x_delta = self.head.x - self.tail.x;
        let y_delta = self.head.y - self.tail.y;
        PositionSigned::new(x_delta, y_delta)
    }
}

pub fn parse_input(input: &String) -> (Direction, i32) {
    let d = input.as_bytes()[0] as char;
    let num: i32 = input[2..input.len()].parse().unwrap();
    match d {
        'R' => (Direction::RIGHT, num),
        'D' => (Direction::DOWN, num),
        'L' => (Direction::LEFT, num),
        'U' => (Direction::UP, num),
        _ => panic!()
    }
}


pub fn patterns(input: &Vec<String>) -> HashSet<PositionSigned> {
    let mut tail_positions = HashSet::new();
    let mut rope = Rope::new(0,0);
    tail_positions.insert(rope.tail.clone());
    for line in input{
        let (direction, amount) = parse_input(line);
        for _ in 0..amount {
            rope.move_head(&direction);
            tail_positions.insert(rope.tail.clone());
        }
    }
    tail_positions
}

pub fn solve_1(input: &Vec<String>) -> u32 {
    let tail_positions = patterns(input);
    return tail_positions.len() as u32;
}

pub fn solve_2(input: &Vec<String>) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::util::{get_input, get_test_input};
    use super::*;

    #[test]
    fn generate_solution_1() {
        let input = get_input(9);
        let actual = solve_1(&input);
        assert_eq!(0, actual);
    }

    #[test]
    fn generate_solution_1_test_input() {
        let input = get_test_input(9);
        let actual = solve_1(&input);

        assert_eq!(13, actual);
    }

    #[test]
    fn parsing() {
        assert_eq!((Direction::UP, 124), parse_input(&String::from("U 124")));
        assert_eq!((Direction::DOWN, 124), parse_input(&String::from("D 124")));
        assert_eq!((Direction::LEFT, 124), parse_input(&String::from("L 124")));
        assert_eq!((Direction::RIGHT, 124), parse_input(&String::from("R 124")));

        assert_eq!((Direction::UP, 1), parse_input(&String::from("U 1")));
        assert_eq!((Direction::DOWN, 1), parse_input(&String::from("D 1")));
        assert_eq!((Direction::LEFT, 1), parse_input(&String::from("L 1")));
        assert_eq!((Direction::RIGHT, 1), parse_input(&String::from("R 1")));
    }
}