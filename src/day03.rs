pub fn solve_1(input: &Vec<String>) -> u32 {
    let mut sum = 0;
    for i in input {
        let rucksack = Rucksack::from(i.as_str());
        let common_item = rucksack.find_common_item();
        let priority = char_to_priority(common_item);
        sum += priority;
    }
    sum
}

pub fn solve_2(input: &Vec<String>) -> u32 {
    todo!()
}

struct Rucksack<'a> {
    left: &'a str,
    right: &'a str
}

fn char_to_priority(input: char) -> u32 {
    let numerical = input as u32;
    let ret;
    if input.is_ascii_uppercase() {
        let uppercase_start = 'A' as u32;
        ret = numerical - uppercase_start + 26;
    } else {
        let lowercase_start = 'a' as u32;
        ret = numerical - lowercase_start;
    };
    return ret + 1;
}
impl Rucksack<'_> {
    pub fn from (content: &str) -> Rucksack {
        Rucksack{ left: &content[0..content.len()/2], right: &content[content.len()/2..content.len()]}
    }

    pub fn find_common_item(&self) -> char {
        for i in self.left.chars() {
            if self.right.contains(i) {
                return i;
            }
        }
        '0'
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use crate::util;
    use super::*;
    static INPUT_RAW: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    fn get_input () -> Vec<String>{
        util::into_lines_vec(&String::from(INPUT_RAW))
    }

    #[test]
    fn generate_solution_1() {
        let input = util::get_input(3);
        let actual = solve_1(&input);
        assert_eq!(0, actual)
    }

    #[test]
    fn generate_solution_2() {
        let input = get_input();
        let actual = solve_2(&input);
        assert_eq!(7967, actual)
    }

    #[test]
    fn rucksack_representation() {
        let input = get_input();
        let first = input.get(0).unwrap();
        let rucksack = Rucksack::from(first.as_str());
        assert!(!rucksack.right.is_empty());
        assert_eq!(rucksack.right.len() + rucksack.left.len(), first.len());
        assert_eq!(rucksack.right.len(), rucksack.left.len())
    }

    #[test]
    fn find_priorities() {
        let input = get_input();
        let mut actual = vec![];
        let mut priorities = vec![];
        let mut sum = 0;
        for i in &input {
            let rucksack = Rucksack::from(i.as_str());
            let common_item = rucksack.find_common_item();
            let priority = char_to_priority(common_item);
            actual.push(common_item);
            priorities.push(priority);
            sum += priority;
        }

        assert_eq!(vec!['p', 'L', 'P', 'v', 't', 's'], actual);
        assert_eq!(vec![16, 38, 42, 22, 20, 19], priorities);
        assert_eq!(157, sum);
    }
}