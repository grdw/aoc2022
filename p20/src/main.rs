use std::fs;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let nums = parse_file(file);
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 3);
}

fn part2(file: &'static str) -> usize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse_file(file: &'static str) -> Vec<i32> {
    let contents = fs::read_to_string(file).unwrap();
    contents
        .split_terminator("\n")
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}
