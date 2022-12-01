use std::fs;

fn main() {
    println!("Part 1: {}", part1("input"));
}

fn part1(file: &'static str) -> u32 {
    let angels = fs::read_to_string(file).unwrap();
    let mut max = 0;

    for angel in angels.split("\n\n") {
        let total: u32 = angel
            .split_terminator("\n")
            .map(|n| n.parse::<u32>().unwrap())
            .sum();

        if total > max {
            max = total
        }
    }
    max
}

#[test]
fn test_input() {
    assert_eq!(part1("test_input"), 24000)
}
