use std::fs;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let coords = parse(file);
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 64);
}

fn part2(file: &'static str) -> usize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(file: &'static str) -> Vec<(usize, usize, usize)> {
    let contents = fs::read_to_string(file).unwrap();
    contents.split_terminator("\n").map(|line| {
        let nums: Vec<usize> = line
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        (nums[0], nums[1], nums[2])
    }).collect()
}
