use std::fs;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> u64 {
    let doubles = parse_rucksacks(file);

    calculate_string_value(doubles)
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

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 157)
}

fn part2(file: &'static str) -> u64 {
    let result = parse_badges(file);

    calculate_string_value(result)
}

fn parse_badges(file: &'static str) -> String {
    let contents = fs::read_to_string(file).unwrap();
    let mut result = String::new();
    let mut f_line: Vec<char> = vec![];

    for (i, line) in contents.split("\n").enumerate() {
        if i % 3 == 0 {
            if let Some(d) = f_line.get(0) {
                result.push(*d);
            }

            f_line = line.chars().collect();
        } else {
            f_line.retain(|&c| line.chars().any(|d| d == c));
        }
    }

    result
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 70)
}

fn calculate_string_value(s: String) -> u64 {
    s.chars().map(|n| {
        let (start, offset) = if n.is_uppercase() {
            ('A', 27)
        } else {
            ('a', 1)
        };

        ((n as u8) - (start as u8) + offset) as u64
    }).sum()
}
