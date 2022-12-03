pub fn solve_1(input: &Vec<String>) -> u128 {
    let grouped_values = convert_to_grouped_vector(input);
    let added_values = summarize_groups(grouped_values);

    find_biggest(added_values)
}

pub fn solve_2(input: &Vec<String>) -> u128 {
    let grouped_values = convert_to_grouped_vector(input);
    let mut added_values = summarize_groups(grouped_values);

    added_values.sort();

    added_values.get(added_values.len() - 1).unwrap() +
    added_values.get(added_values.len() - 2).unwrap() +
    added_values.get(added_values.len() - 3).unwrap()
}


pub fn summarize_groups(grouped_values: Box<Vec<Vec<u128>>>) -> Box<Vec<u128>> {
    Box::new(grouped_values.iter()
        .map(|x| add_values(x))
        .collect::<Vec<u128>>())
}

fn find_biggest(values: Box<Vec<u128>>) -> u128 {
    let mut biggest = 0;

    for value in *values {
        if value > biggest {
            biggest = value;
        }
    }
    biggest
}

fn add_values(values: &Vec<u128>) -> u128 {
    let mut ret = 0;
    for value in values {
        ret += value;
    }
    ret
}

fn convert_to_grouped_vector(input: &Vec<String>) -> Box<Vec<Vec<u128>>> {
    let mut result = Box::new(vec![vec![]]);
    let mut idx = 0;

    for individual_string in input {
        if individual_string.is_empty() {
            result.push(vec![]);
            idx = idx + 1;
            continue;
        }
        let value = individual_string.parse::<u128>().unwrap();
        result.get_mut(idx).expect("Error").push(value);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::util;
    use super::*;
    static INPUT_RAW: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    #[test]
    fn test_split_at_line_break () {
        let input: Vec<String> = util::into_lines_vec(&String::from(INPUT_RAW));
        let expected: Vec<Vec<u128>> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000]];
        assert_eq!(*convert_to_grouped_vector(&input), expected)
    }

    #[test]
    fn test_add_values() {
        let input = vec![1000, 2000, 3000];
        let expected: u128 = 6000;
        assert_eq!(add_values(&input), expected)
    }
    

    #[test]
    fn test_find_biggest() {
        let input = Box::new(vec![6000, 4000, 11000, 24000, 10000]);
        let actual = find_biggest(input);
        let expected: u128 = 24000;
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_solve_single_example_input() {
        let input = util::into_lines_vec(&String::from(INPUT_RAW));
        let actual = solve_1(&input);
        let expected: u128 = 24000;
        assert_eq!(actual, expected)
    }

    #[test]
    fn generate_solution_1() {
        let input = util::get_input(1);
        let actual = solve_1(&input);
        let expected: u128 = 69626;

        assert_eq!(actual, expected);
    }


    #[test]
    fn generate_solution_2() {
        let input = util::get_input(1);
        let actual = solve_2(&input);
        let expected: u128 = 206780;
        assert_eq!(actual, expected)
    }
}