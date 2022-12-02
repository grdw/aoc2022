use std::fs;

const ROCK: u64 = 1;
const PAPER: u64 = 2;
const SCISSORS: u64 = 3;
const WIN: u64 = 3;
const LOSE: u64 = 1;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> u64 {
    parse_file(file).into_iter().map(|(elf, me)| {
        if (me == PAPER && elf == ROCK) ||
           (me == SCISSORS && elf == PAPER) ||
           (me == ROCK && elf == SCISSORS) {
            me + 6
        } else if me == elf {
            me + 3
        } else {
            me
        }
    }).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 45)
}

fn part2(file: &'static str) -> u64 {
    parse_file(file).into_iter().map(|(elf, me)| {
        match (me, elf) {
            (LOSE, ROCK)     => SCISSORS,
            (LOSE, PAPER)    => ROCK,
            (LOSE, SCISSORS) => PAPER,
            (WIN, ROCK)      => PAPER + 6,
            (WIN, PAPER)     => SCISSORS + 6,
            (WIN, SCISSORS)  => ROCK + 6,
            (_, _) => elf + 3
        }
    }).sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 45)
}

fn parse_file(file: &'static str) -> Vec<(u64, u64)> {
    let file = fs::read_to_string(file).unwrap();

    file.split_terminator("\n").map(|line| {
        let elf = line.chars().nth(0).unwrap() as u8 - ('A' as u8) + 1;
        let me = line.chars().nth(2).unwrap() as u8 - ('X' as u8) + 1;

        (elf as u64, me as u64)
    }).collect()
}

