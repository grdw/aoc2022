use std::fs;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> u64 {
    let points = parse_file(file);
    points.into_iter().map(|(elf, me)| {
        if (me == 2 && elf == 1) || (me == 3 && elf == 2) || (me == 1 && elf == 3) {
            me + 6
        } else if me == elf {
            me + 3
        } else {
            me + 0
        }
    }).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 45)
}

fn part2(file: &'static str) -> u64 {
    let points = parse_file(file);
    points.into_iter().map(|(elf, me)| {
        if me == 2 {
            elf + 3
        } else if me == 1 {
            if elf == 1 { 3 }
            else if elf == 2 { 1 }
            else { 2 }
        } else {
            if elf == 1 { 8 }
            else if elf == 2 { 9 }
            else { 7 }
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

