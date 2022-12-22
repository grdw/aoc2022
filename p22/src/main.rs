use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    wall: bool
}

type Instructions = Vec<(u8, char)>;
type MonkeyMap = Vec<Point>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let (map, instr) = parse(file);
    println!("{:?}", map);
    println!("{:?}", instr);
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 1);
}

fn part2(file: &'static str) -> usize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(file: &'static str) -> (MonkeyMap, Instructions) {
    let text = fs::read_to_string(file).unwrap();

    let mut dir_map = vec![];
    let mut parsed_instructions = vec![];
    let (map, mut instructions) = text.split_once("\n\n").unwrap();
    for (y, line) in map.split("\n").enumerate() {
        for (x, p) in line.chars().enumerate() {
            if p == ' ' { continue }

            dir_map.push(
                Point { y: y, x: x, wall: p == '#' }
            );
        }
    }


    let mut number = String::new();
    for c in instructions.chars() {
        match c {
            '0'..='9' => number.push(c),
            'R' | 'L' | '\n' => {
                let parse_number = number.parse::<u8>().unwrap();
                parsed_instructions.push((parse_number, c));
                number = String::from("");
            },
            _ => panic!("Invalid char!")
        }
    }

    (dir_map, parsed_instructions)
}
