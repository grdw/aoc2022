use std::fs;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> u64 {
    let doubles = parse_rucksacks(file);
    doubles.chars().map(|n| {
        if n.is_uppercase() {
            ((n as u8) - ('A' as u8) + 27) as u64
        } else {
            ((n as u8) - ('a' as u8) + 1) as u64
        }
    }).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 157)
}

fn part2(file: &'static str) -> u64 {
    0
}

#[test]
fn test_part2() {

}

fn parse_rucksacks(file: &'static str) -> String {
    let mut result = String::new();
    let contents = fs::read_to_string(file).unwrap();
    for line in contents.split_terminator("\n") {
        let half = line.len() / 2;
        let comp1 = String::from(&line[0..half]);

        for c in line[half..].chars() {
            if comp1.contains(c) {
                result.push(c);
                break;
            }
        }
    }
    result
}
