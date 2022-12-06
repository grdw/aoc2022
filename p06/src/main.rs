use std::fs;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    parse_and_answer(file, 4)
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 7);
    assert_eq!(part1("test_input2"), 5);
    assert_eq!(part1("test_input3"), 6);
    assert_eq!(part1("test_input4"), 10);
    assert_eq!(part1("test_input5"), 11)
}

fn part2(file: &'static str) -> usize {
    parse_and_answer(file, 14)
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 19);
    assert_eq!(part2("test_input2"), 23);
    assert_eq!(part2("test_input3"), 23);
    assert_eq!(part2("test_input4"), 29);
    assert_eq!(part2("test_input5"), 26)
}

fn parse_and_answer(file: &'static str, l: usize) -> usize {
	let stream = fs::read_to_string(file).unwrap();
    let mut answer = 0;

	for i in 0..(stream.len() - l) {
        let slice = &stream[i..i+l];
        let mut n: Vec<char> = slice.chars().collect();
        n.sort();
        n.dedup();

        if n.len() == l {
            answer = i + l;
            break;
        }
	}

    answer
}
