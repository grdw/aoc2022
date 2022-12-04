use std::fs;
use std::ops::RangeInclusive;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let mut areas = parse_areas(file);
    areas.retain(|(left, right)| {
        (left.start() >= right.start() && left.end() <= right.end()) ||
        (right.start() >= left.start() && right.end() <= left.end())
    });
    areas.len()
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 2)
}

fn part2(file: &'static str) -> usize {
    let mut areas = parse_areas(file);
    areas.retain(|(left, right)| {
        left.contains(&right.start()) ||
            right.contains(&left.start())
    });
    areas.len()
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 4)
}

fn parse_areas(file: &'static str) -> Vec<(
    RangeInclusive<u8>,
    RangeInclusive<u8>
)> {
    let file = fs::read_to_string(file).unwrap();

    file.split_terminator("\n").map(|line| {
        let mut split = line.split(",");

        (parse_range(split.next()), parse_range(split.next()))
    }).collect()
}

fn parse_range(area: Option<&str>) -> RangeInclusive<u8> {
    let range_value: Vec<u8> = area
        .unwrap()
        .split("-")
        .map(|c| c.parse::<u8>().unwrap() )
        .collect();

    range_value[0]..=range_value[1]
}
