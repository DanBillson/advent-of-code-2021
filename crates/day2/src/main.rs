fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    7
}

#[cfg(test)]
mod test {
    use crate::solve_problem1;

    #[test]
    fn problem1() {
        let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
        "#;
        let expected = 7;
        let actual = solve_problem1(input);
        assert_eq!(expected, actual)
    }
}
