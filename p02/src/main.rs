use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", 0);
}

fn part1(file: &'static str) -> u64 {
    let points = parse_file(file);
    points.iter().sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 45)
}

fn parse_file(file: &'static str) -> Vec<u64> {
    let file = fs::read_to_string(file).unwrap();

    file.split_terminator("\n").map(|line| {
        let elf = (line.chars().nth(0).unwrap() as u8 - ('A' as u8) + 1) as u64;
        let me = (line.chars().nth(2).unwrap() as u8 - ('X' as u8) + 1) as u64;

        if (me == 2 && elf == 1) || (me == 3 && elf == 2) || (me == 1 && elf == 3) {
            me + 6
        } else if me == elf {
            me + 3
        } else {
            me + 0
        }
    }).collect()
}
