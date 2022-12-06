pub fn solve_1(input: &Vec<String>) -> u32 {
    find_marker(input.first().unwrap().as_str(), 4)
}

pub fn solve_2(input: &Vec<String>) -> u32 {
    find_marker(input.first().unwrap().as_str(), 14)
}

fn find_marker(message: &str, marker_length: usize) -> u32 {
    for i in 0..message.len() - marker_length {
        let curr = &message[i..i+marker_length];
        if only_individual_chars(curr) {
            return (i+marker_length) as u32;
        }
    }
    panic!();
}

fn only_individual_chars(input: &str) -> bool {
    let mut v = vec![];
    for c in input.chars() {
        if v.contains(&c) {
            return false;
        }
        v.push(c);
    }
    return true;
}



#[cfg(test)]
mod tests {
    use crate::util::get_input;
    use super::*;

    #[test]
    fn generate_solution_part1() {
        let input = get_input(6);
        let actual = solve_1(&input);
        assert_eq!(1658, actual)
    }

    #[test]
    fn generate_solution_part2() {
        let input = get_input(6);
        let actual = solve_2(&input);
        assert_eq!(2260, actual)
    }

    #[test]
    fn generate_solution_part1_test_input() {
        let input = vec![String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")];
        let actual = solve_1(&input);
        assert_eq!(5, actual)
    }
}