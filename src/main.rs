pub mod util;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

fn main() {
    let input = util::get_input(5);
    let solution = day05::solve_1(&input);
    println!("Solution: {}", solution);
}
