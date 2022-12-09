pub fn solve_1(input: &Vec<String>) -> u32 {
    find_marker(input.first().unwrap().as_str(), 4)
}

pub fn solve_2(input: &Vec<String>) -> u32 {
    find_marker(input.first().unwrap().as_str(), 14)
}

fn find_marker(message: &str, marker_length: usize) -> u32 {
    let mut i = 0;
    while i < message.len() - marker_length {
        let curr = &message[i..i+marker_length];
        let marker = only_individual_chars(curr);
        match marker {
            MessageMarker::Marker => return (i+marker_length) as u32,
            MessageMarker::Skip(x) => i += x
        }
    }
    panic!();
}

#[derive(Debug, PartialEq)]
enum MessageMarker {
    Marker,
    Skip(usize)
}

fn only_individual_chars(input: &str) -> MessageMarker {
    let mut v = vec![];
    for c in input.chars() {
        let skip = v.iter().position(|x| *x == c);
        match skip {
            Some(x) => return MessageMarker::Skip(x + 1),
            None => v.push(c)
        }
    }
    return MessageMarker::Marker;
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

    #[test]
    fn test_skip() {
        let actual = only_individual_chars("abab");
        assert_eq!(MessageMarker::Skip(1), actual);
    }
}