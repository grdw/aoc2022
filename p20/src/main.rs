use std::fs;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> i32 {
    let numbers = parse(file);
    let mut fork = numbers.clone();
    let positions = vec![1000, 2000, 3000];
    let len = numbers.len() as i32;

    for i in 0..numbers.len() {
        let n = numbers[i];
        let j = fork.iter().position(|&m| n == m).unwrap();
        let mut r = (j as i32 + n).rem_euclid(len - 1) as usize;

        // If I need to wrap back around
        if r == 0 {
            r = (len - 1) as usize
        }

        let del_n = fork.remove(j);
        fork.insert(r as usize, del_n);
    }


    let offset = fork.iter().position(|&m| m == 0).unwrap();
    println!("{}", offset);

    positions
        .iter()
        .map(|n| fork[(n + offset) % fork.len()])
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 3);
}

fn part2(file: &'static str) -> usize {
    let numbers = parse(file);
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(file: &'static str) -> Vec<i32> {
    let text = fs::read_to_string(file).unwrap();
    text
        .split_terminator("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}
