use std::fs;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> u32 {
    let totals = parse_angels(file);
    totals[0]
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 24000)
}

fn part2(file: &'static str) -> u32 {
    let totals = parse_angels(file);
    totals[0..3].iter().sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 45000)
}

fn parse_angels(file: &'static str) -> Vec<u32> {
    let angels = fs::read_to_string(file).unwrap();
    let mut totals: Vec<u32> = angels
        .split("\n\n")
        .map(|angel|{
            angel
                .split_terminator("\n")
                .map(|n| n.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    totals.sort_by(|a, b| b.cmp(a));
    totals
}
