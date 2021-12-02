const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn numbers(input: &str) -> Vec<i32> {
    let numbers = input
        .trim()
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    numbers
}

fn solve_problem1(input: &str) -> usize {
    numbers(input).windows(2).filter(|w| w[0] < w[1]).count()
}

fn solve_problem2(input: &str) -> usize {
    numbers(input)
        .windows(3)
        .map(|w| -> i32 { w.iter().sum() })
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}
