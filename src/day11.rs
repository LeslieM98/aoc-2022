type StressLevel = u128;
type Condition = Box<dyn Fn(StressLevel) -> bool>;
type Operation = Box<dyn Fn(StressLevel) -> StressLevel>;

struct Monkey {
    items: Vec<StressLevel>,
    operation: Operation,
    condition: StressLevel,
    monkey_true: usize,
    monkey_false: usize,
    inspected_items: usize
}

impl Monkey{
    pub fn parse(input: &[String]) -> Monkey {
        let condition = Self::parse_condition(&input[3]);
        Monkey{
            items: Self::parse_items(&input[1]),
            operation: Self::parse_operation(&input[2], condition),
            condition,
            monkey_true: Self::parse_monkey_true(&input[4]),
            monkey_false: Self::parse_monkey_false(&input[5]),
            inspected_items: 0
        }
    }

    fn parse_items(input: &String) -> Vec<StressLevel> {
        let mut ret = vec![];
        let numbers = &input[18..];
        for n in numbers.split(", ").into_iter() {
            let parsed = n.parse().expect("Could not parse number");
            ret.push(parsed);
        }
        ret
    }

    fn parse_operation(input: &String, condition: StressLevel) -> Operation {
        let operator = input.as_bytes()[23] as char;
        let operand = &input[25..];
        if operand.starts_with('o') {
            Box::new(move |x| x * x)
        } else {
            let parsed_operand: StressLevel = operand.parse().expect("Could not parse number");
            match operator {
                '+' =>  Box::new(move |x| x + parsed_operand),
                '*' =>  Box::new(move |x| x * parsed_operand),
                _ => panic!()
            }
        }
    }

    fn parse_condition(input: &String) -> StressLevel {
        let relevant_part = &input[21..];
        let parsed: StressLevel = relevant_part.parse().expect("Could not parse number");
        parsed
    }

    fn parse_monkey_true(input: &String) -> usize {
        let relevant_part = &input[29..];
        relevant_part.parse().expect("Could not parse number")
    }

    fn parse_monkey_false(input: &String) -> usize {
        let relevant_part = &input[30..];
        relevant_part.parse().expect("Could not parse number")
    }

    fn operate(&mut self, stress_divider: StressLevel, modulo: StressLevel) -> (StressLevel, usize) {
        let mut item = self.items.remove(0);
        item = (self.operation)(item % modulo);
        item = item / stress_divider;
        let monkey = if item % self.condition == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        };
        (item, monkey)
    }

    pub fn take_turn(&mut self, stress_divider: StressLevel, modulo: StressLevel) -> Vec<(StressLevel, usize)> {
        let mut ret = vec![];
        while !self.items.is_empty() {
            ret.push(self.operate(stress_divider, modulo));
        }
        self.inspected_items += ret.len();
        ret
    }

    fn push(&mut self, item: StressLevel) {
        self.items.push(item);
    }
}

fn take_round(monkeys: &mut Vec<Monkey>, stress_divider: StressLevel, modulo: StressLevel) {
    for curr_idx in 0..monkeys.len(){
        let monkey = monkeys.get_mut(curr_idx).expect("Something weird happened");
        let turn = monkey.take_turn(stress_divider, modulo);
        for (item, next_monkey_idx) in turn {
            let next_monkey = monkeys.get_mut(next_monkey_idx).expect("Something weird happened");
            next_monkey.items.push(item);
        }
    }
}


fn solve_1(input: &Vec<String>) -> u128 {
    let rounds = 20;
    let stress_divider = 3;

    take_rounds(input, rounds, stress_divider)
}

fn solve_2(input: &Vec<String>) -> u128 {
    let rounds = 10000;
    let stress_divider = 1;

    take_rounds(input, rounds, stress_divider)
}

fn take_rounds(input: &Vec<String>, rounds: u32, stress_divider: StressLevel) -> u128{
    let mut parsed = vec![];
    let chunks = input.chunks(7);
    let mut modulo = 1;
    for s in chunks {
        let p = Monkey::parse(s);
        modulo *= p.condition;
        parsed.push(p);
    }
    for _ in 0..rounds {
        take_round(&mut parsed, stress_divider, modulo);
    }

    let mut most: u128 = 0;
    let mut second: u128 = 0;
    for monkey in parsed {
        let inspected : u128 = monkey.inspected_items as u128;
        if inspected > most {
            second = most;
            most = inspected;
        } else if inspected < most && inspected > second {
            second = inspected;
        }
    }
    return most * second;
}

#[cfg(test)]
mod tests {
    use crate::util::{get_input, get_test_input};
    use super::*;

    #[test]
    fn correct_parsing_test_input() {
        let input = get_test_input(11);
        let mut parsed = vec![];
        let chunks = input.chunks(7);
        for s in chunks {
            let p = Monkey::parse(s);
            parsed.push(p);
        }

        let subject = parsed.get(0).expect("Error parsing");
        assert_eq!(vec![79, 98], subject.items);
        assert_eq!(2, subject.monkey_true);
        assert_eq!(3, subject.monkey_false);
        assert_eq!(19*2, (subject.operation)(2));
        assert_eq!(23, subject.condition);
    }

    #[test]
    fn correct_single_round_test_input() {
        let input = get_test_input(11);
        let mut parsed = vec![];
        let chunks = input.chunks(7);
        let mut modulo = 1;
        for s in chunks {
            let p = Monkey::parse(s);
            modulo *= p.condition;
            parsed.push(p);
        }

        take_round(&mut parsed, 3, modulo);
        assert_eq!(vec![20, 23, 27, 26], parsed[0].items);
        assert_eq!(vec![2080, 25, 167, 207, 401, 1046], parsed[1].items);
        assert!(parsed[2].items.is_empty());
        assert!(parsed[3].items.is_empty());
    }

    #[test]
    fn correct_20_rounds_test_input() {
        let input = get_test_input(11);
        let mut parsed = vec![];
        let chunks = input.chunks(7);
        let mut modulo = 1;
        for s in chunks {
            let p = Monkey::parse(s);
            modulo *= p.condition;
            parsed.push(p);
        }
        for _ in 0..20 {
            take_round(&mut parsed, 3, modulo);
        }
        assert_eq!(vec![10, 12, 14, 26, 34], parsed[0].items);
        assert_eq!(vec![245, 93, 53, 199, 115], parsed[1].items);
        assert!(parsed[2].items.is_empty());
        assert!(parsed[3].items.is_empty());

        assert_eq!(101, parsed[0].inspected_items);
        assert_eq!(95, parsed[1].inspected_items);
        assert_eq!(7, parsed[2].inspected_items);
        assert_eq!(105, parsed[3].inspected_items);
    }
    
    #[test]
    fn generate_solution_1_test_input() {
        let input = get_test_input(11);
        let actual = solve_1(&input);
        assert_eq!(10605, actual);
    }

    #[test]
    fn generate_solution_1 () {
        let input = get_input(11);
        let actual = solve_1(&input);
        assert_eq!(50172, actual);
    }

    #[test]
    fn generate_solution_2_test_input() {
        let input = get_test_input(11);
        let actual = solve_2(&input);
        assert_eq!(2713310158, actual);
    }

    #[test]
    fn generate_solution_2 () {
        let input = get_input(11);
        let actual = solve_2(&input);
        assert_eq!(11614682178, actual);
    }
}