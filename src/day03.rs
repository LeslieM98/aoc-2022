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
    let groups = group_elves(&input);
    let mut sum = 0;
    for group in groups {
        let priority = char_to_priority(group.get_badge());
        sum += priority;
    }
    return sum;
}

struct Rucksack<'a> {
    content: &'a str
}

impl Rucksack<'_> {
    pub fn from (content: &str) -> Rucksack {
        Rucksack{ content }
    }

    pub fn find_common_item(&self) -> char {
        for i in self.left().chars() {
            if self.right().contains(i) {
                return i;
            }
        }
        todo!()
    }

    pub fn left(&self) -> &str {
        return &self.content[0 .. self.content.len()/2];
    }

    pub fn right(&self) -> &str {
        return &self.content[self.content.len()/2 .. self.content.len()];
    }

    pub fn contains(&self, item: char) -> bool {
        self.content.contains(item)
    }
}

struct Group<'a> {
    rucksacks: [Rucksack<'a>; 3]
}

impl Group<'_> {
    pub fn from(rucksacks: [Rucksack; 3]) -> Group {
        Group{rucksacks}
    }

    pub fn sort_rucksacks(&self) -> (&Rucksack, &Rucksack, &Rucksack) {
        if self.rucksacks[0].content.len() <= self.rucksacks[1].content.len()
            && self.rucksacks[0].content.len() <= self.rucksacks[2].content.len() {
                return (&self.rucksacks[0],
                        &self.rucksacks[1],
                        &self.rucksacks[2]);
        }
        if self.rucksacks[1].content.len() <= self.rucksacks[0].content.len()
            && self.rucksacks[1].content.len() <= self.rucksacks[2].content.len() {
            return (&self.rucksacks[1],
                    &self.rucksacks[0],
                    &self.rucksacks[2]);
        }
        if self.rucksacks[2].content.len() <= self.rucksacks[1].content.len()
            && self.rucksacks[2].content.len() <= self.rucksacks[2].content.len() {
            return (&self.rucksacks[2],
                    &self.rucksacks[1],
                    &self.rucksacks[0]);
        }
        panic!()
    }

    pub fn get_badge(&self) -> char {
        let (shortest, second, third) = self.sort_rucksacks();

        for i in shortest.content.chars(){
            if second.contains(i) && third.contains(i) {
                return i;
            }
        }
        panic!()
    }
}

fn group_elves(input: &Vec<String>) -> Vec<Group>{
    let mut groups = vec![];
    for i in 0..input.len()/3{
        let group = Group::from([
            Rucksack::from(input.get(i*3).unwrap().as_str()),
            Rucksack::from(input.get(i*3+1).unwrap().as_str()),
            Rucksack::from(input.get(i*3+2).unwrap().as_str())
        ]);
        groups.push(group);
    }
    return groups;
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




#[cfg(test)]
mod tests {
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
        assert_eq!(7967, actual)
    }

    #[test]
    fn generate_solution_2() {
        let input = util::get_input(3);
        let actual = solve_2(&input);
        assert_eq!(2716, actual)
    }

    #[test]
    fn rucksack_representation() {
        let input = get_input();
        let first = input.get(0).unwrap();
        let rucksack = Rucksack::from(first.as_str());
        assert!(!rucksack.right().is_empty());
        assert_eq!(rucksack.right().len() + rucksack.left().len(), first.len());
        assert_eq!(rucksack.right().len(), rucksack.left().len())
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

    #[test]
    fn grouping() {
        let input = get_input();
        let groups = group_elves(&input);
        assert_eq!(2, groups.len())
    }

    #[test]
    fn generate_solution_2_testinput() {
        let input = get_input();
        let groups = group_elves(&input);
        let mut badges = vec![];
        let mut priorities = vec![];
        let mut sum = 0;
        for group in groups {
            let badge = group.get_badge();
            let priority = char_to_priority(badge);
            priorities.push(priority);
            badges.push(badge);
            sum += priority;
        }

        assert_eq!(vec!['r', 'Z'], badges);
        assert_eq!(vec![18, 52], priorities);
        assert_eq!(70, sum);
    }
}