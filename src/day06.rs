fn solve_1(input: &Vec<String>) -> u32 {
    let data = input.first().unwrap().as_str();
    for i in 0..data.len() - 4 {
        let curr = &data[i..i+4];
        if only_individual_chars(curr) {
            return (i+4) as u32;
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

fn solve_2() -> u32 {
    todo!()
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
    fn generate_solution_part1_test_input() {
        let input = vec![String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")];
        let actual = solve_1(&input);
        assert_eq!(5, actual)
    }
}