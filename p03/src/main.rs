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
    let result = parse_rucksacks_part2(file);
    calculate_string_value(result)
}

fn parse_rucksacks_part2(file: &'static str) -> String {
    let contents = fs::read_to_string(file).unwrap();
    let mut result = String::new();
    let mut count = 0;
    let mut last_index = 0;
    let mut groups: Vec<String> = vec![];

    for (i, _) in contents.match_indices("\n") {
        count += 1;

        if count % 3 == 0 {
            let s = &contents[last_index..i];
            groups.push(String::from(s));
            last_index = i + 1;
        }
    }

    for group in &groups {
        let m: Vec<&str> = group.split("\n").collect();
        let mut unique: Vec<char> = m[0].chars().collect();

        for rugsack in &m {
            unique.retain(|&c| {
                rugsack.chars().any(|d| d == c)
            });
        }

        result.push(unique[0]);
    }

    result
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 70)
}

fn calculate_string_value(s: String) -> u64 {
    s.chars().map(|n| {
        let start = n as u8;

        if n.is_uppercase() {
            (start - ('A' as u8) + 27) as u64
        } else {
            (start - ('a' as u8) + 1) as u64
        }
    }).sum()
}
