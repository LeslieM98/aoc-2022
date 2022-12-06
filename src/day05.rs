#[derive(PartialEq, Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn parse(input: &String) -> Instruction {
        let mut amount_str = String::new();
        let mut from_str = String::new();
        let mut to_str = String::new();
        let mut status = 0;
        for char in input.chars() {
            if status == 0 {
                if char == ' ' {
                    status = 1;
                }
            } else if status == 1 {
                if char == ' ' {
                    status = 2;
                } else {
                    amount_str.push(char);
                }
            } else if status == 2 {
                if char == ' ' {
                    status = 3;
                }
            } else if status == 3 {
                if char == ' ' {
                    status = 4;
                } else {
                    from_str.push(char);
                }
            }else if status == 4 {
                if char == ' ' {
                    status = 5;
                }
            }else if status == 5 {
                if char == '\n' {
                    status = 6;
                } else {
                    to_str.push(char);
                }
            }
        }

        Instruction{
            to: to_str.parse().unwrap(),
            from: from_str.parse().unwrap(),
            amount: amount_str.parse().unwrap()
        }
    }

    pub fn execute(&self, stacks: &mut Vec<Vec<char>>) {
        for _ in 0..self.amount {
            let from = stacks.get_mut(self.from - 1).unwrap();
            let tmp = from.pop().unwrap();
            let to = stacks.get_mut(self.to - 1).unwrap();
            to.push(tmp);
        }
    }
}

#[derive(PartialEq, Debug)]
struct Warehouse {
    stacks: Vec<Vec<char>>,
}

impl Warehouse {

    pub fn get_horizontal_index_from_position(position: usize) -> usize {
        if position == 0{
            return 1;
        }
        return 1 + position * 4;
    }

    pub fn parse(input: &Vec<String>) -> String{
        let mut stacks = vec![];
        let mut end_idx = 0;
        for line_idx in 0..input.len() {
            let line = &input.get(line_idx).unwrap();
            if **line == String::from("") {
                end_idx = line_idx;
                break;
            }
        }

        for i in 1..input[end_idx-1].len() {
            let bytes = input[end_idx-1].as_bytes();
            if (bytes[i] as char).is_ascii_digit() {
                stacks.push(vec![]);
            }
        }

        for line_idx in (0..end_idx-1).rev() {
            let line = input.get(line_idx).unwrap();
            for i in 0..stacks.len() {
                let position = Warehouse::get_horizontal_index_from_position(i);
                if position >= line.len() {
                    break;
                }
                let char = line.as_bytes()[position] as char;
                if !char.is_whitespace() {
                    stacks.get_mut(i).unwrap().push(char);
                }
            }
        }

        for line_idx in end_idx+1..input.len() {
            let line = input.get(line_idx).unwrap();
            let instruction = Instruction::parse(line);
            instruction.execute(&mut stacks);
        }

        let mut solution = String::new();
        for v in &mut stacks {
            let tmp = v.pop().unwrap();
            solution.push(tmp);
        }
        return solution;
    }
}



#[cfg(test)]
mod tests {
    use crate::util;
    use super::*;

    const INPUT_RAW: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3\n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
    fn get_input () -> Vec<String>{
        util::into_lines_vec(&String::from(INPUT_RAW))
    }

    #[test]
    fn correct_stack_count() {
        let input = get_input();
        let actual = Warehouse::parse(&input).len();
        assert_eq!(3, actual);
    }

    #[test]
    fn correct_stack_position() {
        assert_eq!(1, Warehouse::get_horizontal_index_from_position(0));
        assert_eq!(5, Warehouse::get_horizontal_index_from_position(1));
        assert_eq!(9, Warehouse::get_horizontal_index_from_position(2));
    }
    
    #[test]
    fn test_correct_instruction_parsing() {
        let input = String::from("move 23 from 5 to 8\n");
        let actual = Instruction::parse(&input);
        assert_eq!(Instruction{from: 5, to: 8, amount: 23}, actual);
    }

    #[test]
    fn generate_solution_1_test_input() {
        let input = get_input();
        let actual = Warehouse::parse(&input);
        assert_eq!(String::from("CMZ"), actual);
    }

    #[test]
    fn generate_solution_1() {
        let input = util::get_input(5);
        let actual = Warehouse::parse(&input);
        assert_eq!(String::new(), actual);
    }
}