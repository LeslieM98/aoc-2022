use std::collections::HashSet;
use std::hash::Hash;

pub fn solve_1(input: &Vec<String>) -> u32 {
    let forest = Forest::parse(input);
    let mut walker = forest.create_walker(0,0);
    let mut visible: HashSet<Position> = HashSet::new();
    let mut b = true;
    while b {
        visible.extend(walker.look_down().iter());
        b = walker.walk_right();
    }

    b = true;
    while b {
        visible.extend(walker.look_left().iter());
        b = walker.walk_down();
    }

    b = true;
    while b {
        visible.extend(walker.look_up().iter());
        b = walker.walk_left();
    }

    b = true;
    while b {
        visible.extend(walker.look_right().iter());
        b = walker.walk_up();
    }
    visible.len() as u32
}

pub fn solve_2(input: &Vec<String>) -> u32 {
    todo!();
}

type Tree = u8;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Position{
    x:usize,
    y:usize
}

impl Position {
    pub fn new(x: usize, y:usize) -> Position {
        Position{x, y}
    }
}

struct Forest {
    trees: Vec<Vec<Tree>>
}

struct Walker<'a> {
    position: Position,
    forest: &'a Forest
}

impl Forest {
    pub fn parse(input: &Vec<String>) -> Forest{
        let mut trees = vec![];
        for line in input {
            if line.is_empty(){
                continue;
            }
            let mut row = vec![];
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
            trees.push(row);
        }
        return Forest {trees};
    }

    pub fn create_walker(&self, x: usize, y:usize) -> Walker {
        let position = Position{x,y};
        Walker{ position, forest: self}
    }
}

impl Walker<'_> {
    pub fn walk_up(&mut self) -> bool {
        if self.position.y == 0 {
            false
        } else {
            self.position.y -= 1;
            true
        }
    }

    pub fn walk_down(&mut self) -> bool {
        let next = self.position.y + 1;
        let len = self.forest.trees.len();
        if len == next {
            false
        } else {
            self.position.y = next;
            true
        }
    }

    pub fn walk_left(&mut self) -> bool {
        if self.position.x == 0 {
            false
        } else {
            self.position.x -= 1;
            true
        }
    }

    pub fn walk_right(&mut self) -> bool {
        let next = self.position.x + 1;
        let len = self.forest.trees.get(0).unwrap().len();
        if len == next {
            false
        } else {
            self.position.x = next;
            true
        }
    }

    pub fn pos(&self) -> Position {
        self.position
    }

    pub fn look_down(&self) -> HashSet<Position> {
        self.look(Self::walk_down)
    }

    pub fn look_up(&self) -> HashSet<Position> {
        self.look(Self::walk_up)
    }

    pub fn look_left(&self) -> HashSet<Position> {
        self.look(Self::walk_left)
    }

    pub fn look_right(&self) -> HashSet<Position> {
        self.look(Self::walk_right)
    }

    fn look(&self, walk_direction: fn(&mut Self) -> bool) -> HashSet<Position> {
        let mut visible = HashSet::new();
        let mut walker = self.clone();

        let mut highest = walker.get();
        visible.insert(walker.pos());
        let mut b = walk_direction(&mut walker);
        while b {
            let curr = walker.get();
            if highest < curr {
                visible.insert(walker.pos());
                highest = curr;
            }
            b = walk_direction(&mut walker);
        }

        return visible;
    }

    pub fn get(&self) -> Tree {
        *self.forest.trees.get(self.position.y).unwrap().get(self.position.x).unwrap()
    }
}

impl Clone for Walker<'_> {
    fn clone(&self) -> Self {
        Walker{position: self.position, forest: self.forest}
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{get_input, get_test_input};
    use super::*;

    #[test]
    fn generate_solution_1() {
        let input = get_input(8);
        let actual = solve_1(&input);
        assert_eq!(0, actual);
    }

    #[test]
    fn generate_solution_2() {
        let input = get_input(8);
        let actual = solve_2(&input);
        assert_eq!(0, actual);
    }

    #[test]
    fn generate_solution_1_test_input() {
        let input = get_test_input(8);
        let actual = solve_1(&input);
        assert_eq!(21, actual);
    }

    #[test]
    fn generate_solution_2_test_input() {
        let input = get_test_input(8);
        let actual = solve_2(&input);
        assert_eq!(0, actual);
    }

    #[test]
    fn correct_parsing() {
        let input = get_test_input(8);
        let actual = Forest::parse(&input);
        let expected: Vec<Vec<u8>> = vec![vec![3,0,3,7,3], vec![2,5,5,1,2], vec![6,5,3,3,2], vec![3,3,5,4,9], vec![3,5,3,9,0]];
        assert_eq!(expected, actual.trees);
    }

    #[test]
    fn correct_walking() {
        let input = get_test_input(8);
        let forest = Forest::parse(&input);
        let mut walker = forest.create_walker(0,0);

        let mut walked_trees = vec![];
        let mut b = true;
        while b {
            walked_trees.push(walker.get());
            b = walker.walk_right();
        }
        assert_eq!(vec![3,0,3,7,3], walked_trees);

        walked_trees = vec![];
        b = true;
        while b {
            walked_trees.push(walker.get());
            b = walker.walk_down();
        }
        assert_eq!(vec![3,2,2,9,0], walked_trees);

        walked_trees = vec![];
        b = true;
        while b {
            walked_trees.push(walker.get());
            b = walker.walk_left();
        }
        assert_eq!(vec![0,9,3,5,3], walked_trees);

        walked_trees = vec![];
        b = true;
        while b {
            walked_trees.push(walker.get());
            b = walker.walk_up();
        }
        assert_eq!(vec![3,3,6,2,3], walked_trees);
    }

    #[test]
    fn correct_looking() {
        let input = get_test_input(8);
        let forest = Forest::parse(&input);
        let mut walker = forest.create_walker(1,0);
        let mut visible = walker.look_down();
        assert_eq!(2, visible.len());
        assert!(visible.contains(&Position::new(1,0)));
        assert!(visible.contains(&Position::new(1,1)));


        walker = forest.create_walker(2,4);
        visible = walker.look_up();
        assert_eq!(2, visible.len());
        assert!(visible.contains(&Position::new(2,4)));
        assert!(visible.contains(&Position::new(2,3)));

        walker = forest.create_walker(4,2);
        visible = walker.look_left();
        assert_eq!(2, visible.len());
        assert!(visible.contains(&Position::new(4,2)));
        assert!(visible.contains(&Position::new(3,2)));

        walker = forest.create_walker(0,1);
        visible = walker.look_right();
        assert_eq!(2, visible.len());
        assert!(visible.contains(&Position::new(0,1)));
        assert!(visible.contains(&Position::new(1,1)));
    }
    
    #[test]
    fn contains_inner_trees_test_input() {
        let input = get_test_input(8);
        let forest = Forest::parse(&input);
        let mut visible: HashSet<Position> = HashSet::new();

        visible.extend(forest.create_walker(1,0).look_down().iter());
        visible.extend(forest.create_walker(2,0).look_down().iter());
        visible.extend(forest.create_walker(4,2).look_left().iter());
        visible.extend(forest.create_walker(0,1).look_down().iter());

        visible.remove(&Position::new(1,0));
        visible.remove(&Position::new(2,0));
        visible.remove(&Position::new(4,2));
        visible.remove(&Position::new(0,1));

        assert_eq!(5, visible.len());
        assert!(visible.contains(&Position::new(1,1)));
        assert!(visible.contains(&Position::new(2,1)));
        assert!(visible.contains(&Position::new(1,2)));
        assert!(visible.contains(&Position::new(3,2)));
    }
}
