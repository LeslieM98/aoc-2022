use crate::day02::Outcome::{DRAW, LOSS, WIN};

#[derive(PartialEq, Debug)]
enum Outcome{
    WIN(u8),
    DRAW(u8),
    LOSS(u8)
}

impl Outcome {
    pub fn get_total_score(&self) -> u8 {
        match self {
            WIN(x) => x + 6,
            DRAW(x) => x + 3,
            LOSS(x) => *x
        }
    }
}


pub fn solve_1(input: &Vec<String>) -> u32 {
    let tuples = generate_tuples(input);
    let outcomes: Vec<u8> = tuples.into_iter()
        .map(|x| calculate_score(x.0, translate_encryption(x.1)))
        .map(|x| x.get_total_score())
        .collect();

    let mut result: u32 = 0;
    for outcome in outcomes {
        result += outcome as u32;
    }

    result
}

pub fn solve_2(input: &Vec<String>) -> u32 {
    let tuples = generate_tuples(input);
    let outcomes: Vec<u8> = tuples.into_iter()
        .map(|x| calculate_score(x.0, predict((x.0, x.1))))
        .map(|x| x.get_total_score())
        .collect();

    let mut result: u32 = 0;
    for outcome in outcomes {
        result += outcome as u32;
    }

    result
}

pub fn predict(input: (char, char)) -> char {
    match input.1 {
        'X' => make_loss(input.0),
        'Y' => make_draw(input.0),
        'Z' => make_win(input.0),
        _ => panic!("incorrect value")
    }
}

fn make_draw(input: char) -> char {
    input
}

fn make_win(input: char) -> char {
    match input {
        'A' => 'B',
        'B' => 'C',
        'C' => 'A',
        _ => panic!("incorrect value")
    }
}

fn make_loss(input: char) -> char {
    match input {
        'A' => 'C',
        'B' => 'A',
        'C' => 'B',
        _ => panic!("incorrect value")
    }
}


fn generate_tuples(input: &Vec<String>) -> Vec<(char, char)> {
    input.into_iter()
        .filter(|s|s.len() == 3)
        .map(|s| (s.as_bytes()[0] as char, s.as_bytes()[2] as char)).collect()
}

fn translate_encryption(input: char) -> char {
    (input as u8 - 23 as u8) as char
}

fn calculate_score(left: char, right: char) -> Outcome {
    let shape_score = match right {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0
    };
    if left == right {
        DRAW(shape_score)
    } else {
        match left {
            'A' => match right {
                'B' => WIN(shape_score),
                'C' => LOSS(shape_score),
                _ => panic!("incorrect value")
            },
            'B' => match right {
                'A' => LOSS(shape_score),
                'C' => WIN(shape_score),
                _ => panic!("incorrect value")
            },
            'C' => match right {
                'A' => WIN(shape_score),
                'B' => LOSS(shape_score),
                _ => panic!("incorrect value")
            },
            _ => panic!("incorrect value")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util;
    use super::*;
    static INPUT_RAW: &str = "A Y\nB X\nC Z";

    #[test]
    fn solve_1_test_input() {
        let input = util::into_lines_vec(&String::from(INPUT_RAW));
        let actual = solve_1(&input);
        assert_eq!(15, actual);
    }
    
    #[test]
    fn test_calculate_score() {
        let input1 = ('A', 'Y');
        let input2 = ('B', 'X');
        let input3 = ('C', 'Z');

        assert_eq!(WIN(2), calculate_score(input1.0, translate_encryption(input1.1)));
        assert_eq!(8, calculate_score(input1.0, translate_encryption(input1.1)).get_total_score());

        assert_eq!(LOSS(1), calculate_score(input2.0, translate_encryption(input2.1)));
        assert_eq!(1, calculate_score(input2.0, translate_encryption(input2.1)).get_total_score());

        assert_eq!(DRAW(3), calculate_score(input3.0, translate_encryption(input3.1)));
        assert_eq!(6, calculate_score(input3.0, translate_encryption(input3.1)).get_total_score());
    }

     #[test]
     fn generate_solution_1() {
         let input = util::get_input(2);
         let actual = solve_1(&input);
         assert_eq!(13809, actual)
     }

    #[test]
    fn generate_solution_2() {
        let input = util::get_input(2);
        let actual = solve_2(&input);
        assert_eq!(12316, actual)
    }
}