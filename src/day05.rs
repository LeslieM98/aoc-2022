struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

struct Warehouse {
    stacks: Vec<Vec<char>>
}

impl Warehouse {

    pub fn parse_stacks(input: &Vec<String>) -> Vec<Vec<char>>{
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

        return stacks;
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
        let actual = Warehouse::parse_stacks(&input).len();
        assert_eq!(3, actual);
    }
}