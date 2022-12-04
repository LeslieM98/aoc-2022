struct Pair<'a> {
    content: &'a str
}

impl Pair<'_> {
    pub fn new(content: &str) -> Pair {
        Pair{content}
    }

    fn extract_range(input: &str) -> (usize, usize) {
        let split: Vec<&str> = input.split('-').collect();
        let left = split.get(0).unwrap().parse();
        let right = split.get(1).unwrap().parse();
        return (left.unwrap(), right.unwrap());
    }

    fn get_elves(&self) -> Vec<&str> {
        self.content.split(',').collect()
    }
    pub fn first(&self) -> (usize, usize) {
        let elves = self.get_elves();
        let first_elf = elves.get(0).unwrap();
        return Self::extract_range(first_elf);
    }

    pub fn second(&self) -> (usize, usize) {
        let elves = self.get_elves();
        let second_elf = elves.get(1).unwrap();
        return Self::extract_range(second_elf);
    }

    pub fn one_contains_the_other(&self) -> bool {
        let first_elf = self.first();
        let second_elf = self.second();
        if first_elf.0 <= second_elf.0 && first_elf.1 >= second_elf.1 {
            return true;
        }
        if second_elf.0 <= first_elf.0 && second_elf.1 >= first_elf.1 {
            return true;
        }
        return false;
    }

    pub fn overlaps(&self) -> bool {
        let first_elf = self.first();
        let second_elf = self.second();

        if first_elf.0 <= second_elf.0 && first_elf.1 >= second_elf.0 {
            return true;
        }
        if first_elf.1 >= second_elf.1 && first_elf.0 <= second_elf.0 {
            return true;
        }
        if second_elf.0 <= first_elf.0 && second_elf.1 >= first_elf.0 {
            return true;
        }
        if second_elf.1 >= first_elf.1 && second_elf.0 <= first_elf.0 {
            return true;
        }
        return false;
    }
}

pub fn solve_1(input: &Vec<String>) -> u32{
    let mut sum = 0;
    for line in input {
        let pair = Pair::new(line.as_str());
        if pair.one_contains_the_other() {
            sum += 1;
        }
    }
    sum
}

pub fn solve_2(input: &Vec<String>) -> u32{
    let mut sum = 0;
    for line in input {
        let pair = Pair::new(line.as_str());
        if pair.overlaps() {
            sum += 1;
        }
    }
    sum
}


#[cfg(test)]
mod tests {
    use crate::util;
    use super::*;

    const INPUT_RAW: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    fn get_input () -> Vec<String>{
        util::into_lines_vec(&String::from(INPUT_RAW))
    }
    
    #[test]
    fn generate_solution_1_test_input() {
        let inputs = get_input();
        let mut pairs = vec![];
        let mut sum = 0;
        for input in &inputs {
            let pair = Pair::new(input.as_str());
            if pair.one_contains_the_other() {
                sum += 1;
            }
            pairs.push(pair);
        }

        assert_eq!(2, sum)
    }

    #[test]
    fn generate_solution_2_test_input() {
        let inputs = get_input();
        let mut pairs = vec![];
        let mut sum = 0;
        for input in &inputs {
            let pair = Pair::new(input.as_str());
            if pair.overlaps() {
                sum += 1;
            }
            pairs.push(pair);
        }

        assert_eq!(4, sum)
    }

    #[test]
    fn generate_solution_1() {
        let input = util::get_input(4);
        let actual = solve_1(&input);
        assert_eq!(588, actual)
    }

    #[test]
    fn generate_solution_2() {
        let input = util::get_input(4);
        let actual = solve_2(&input);
        assert_eq!(911, actual)
    }
}
