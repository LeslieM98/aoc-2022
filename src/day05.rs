struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

struct Warehouse {
    stacks: Vec<Vec<char>>
}

impl Warehouse {

    pub fn get_horizontal_index_from_position(position: usize) -> usize {
        if position == 0{
            return 1;
        }
        return 1 + position * 4;
    }

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

        for line_idx in end_idx .. 0 {
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

    #[test]
    fn correct_stack_init() {
        let input = get_input();
        let actual = Warehouse::parse_stacks(&input);
        assert_eq!(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']], actual)
    }

    #[test]
    fn correct_stack_position() {
        assert_eq!(1, Warehouse::get_horizontal_index_from_position(0));
        assert_eq!(5, Warehouse::get_horizontal_index_from_position(1));
        assert_eq!(9, Warehouse::get_horizontal_index_from_position(2));
    }
}