use std::fs;

const WIDTH: isize = 40;

type Instructions = Vec<String>;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: \n{}", part2("input"));
}

fn part1(input: &'static str) -> isize {
    let instructions: Instructions = parse_instructions(input);

    calculate_signal_strength(&instructions, 20) +
    calculate_signal_strength(&instructions, 60) +
    calculate_signal_strength(&instructions, 100) +
    calculate_signal_strength(&instructions, 140) +
    calculate_signal_strength(&instructions, 180) +
    calculate_signal_strength(&instructions, 220)
}

fn parse_instructions(input: &'static str) -> Instructions {
	let file_text = fs::read_to_string(input).unwrap();
    let mut instructions: Instructions = vec![];

    for f in file_text.split_terminator("\n") {
        instructions.push(f.to_string());
    }

    instructions
}

fn calculate_signal_strength(inst: &Instructions, max: isize) -> isize {
    let mut cycle = 0;
    let mut signal_strength = 0;
    let mut x = 1;

    for instr in inst {
        if instr.starts_with("addx") {
            tick(&mut cycle, &mut signal_strength, x, max);
            tick(&mut cycle, &mut signal_strength, x, max);
            let (_, val) = instr.split_once(" ").unwrap();
            let isize_val = val.parse::<isize>().unwrap();
            x += isize_val;
        } else if instr.starts_with("noop") {
            tick(&mut cycle, &mut signal_strength, x, max);
        } else {
            panic!("Invalid command")
        }
    }

    signal_strength * max
}

fn tick(cycle: &mut isize, signal_strength: &mut isize, x: isize, max: isize) {
    *cycle += 1;

    if *cycle == max {
        *signal_strength += x;
    }
}

#[test]
fn test_calculate_signal_strength() {
	let instructions = parse_instructions("test_input2");

    assert_eq!(calculate_signal_strength(&instructions, 20), 420);
    assert_eq!(calculate_signal_strength(&instructions, 60), 1140);
    assert_eq!(calculate_signal_strength(&instructions, 100), 1800);
    assert_eq!(calculate_signal_strength(&instructions, 140), 2940);
    assert_eq!(calculate_signal_strength(&instructions, 180), 2880);
    assert_eq!(calculate_signal_strength(&instructions, 220), 3960);
}

#[test]
fn test_calculate_signal_strength_easy() {
	let instructions = parse_instructions("test_input");

    assert_eq!(calculate_signal_strength(&instructions, 1), 1);
    assert_eq!(calculate_signal_strength(&instructions, 2), 2);
    assert_eq!(calculate_signal_strength(&instructions, 3), 3);
    assert_eq!(calculate_signal_strength(&instructions, 4), 16);
    assert_eq!(calculate_signal_strength(&instructions, 5), 20);
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input2"), 13140);
}

fn part2(input: &'static str) -> String {
    let instructions: Instructions = parse_instructions(input);
    let mut drawing = String::new();
    let mut cycle = 0;
    let mut x = 1;

    for instr in instructions {
        if instr.starts_with("addx") {
            draw(&mut cycle, x, &mut drawing);
            draw(&mut cycle, x, &mut drawing);
            let (_, val) = instr.split_once(" ").unwrap();
            let isize_val = val.parse::<isize>().unwrap();
            x += isize_val;
        } else if instr.starts_with("noop") {
            draw(&mut cycle, x, &mut drawing);
        } else {
            panic!("Invalid command")
        }
    }

    drawing
}

fn draw(cycle: &mut isize, x: isize, drawing: &mut String) {
    *cycle += 1;

    let sprite = *cycle % WIDTH;
    let c = if sprite <= (x + 2) && sprite >= x {
        '⬜'
    } else {
        '⬛'
    };
    drawing.push(c);
    if *cycle % WIDTH == 0 {
        drawing.push('\n')
    }
}

#[test]
fn test_part2() {
    println!("{}", part2("test_input2"));
}
