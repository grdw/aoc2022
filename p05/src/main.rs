use std::fs;

type Boxes = Vec<String>;
type Instructions = Vec<(usize, usize, usize)>;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> String {
    let (mut boxes, instructions) = parse_input(file);
    execute_instructions(&mut boxes, instructions);
    boxes
        .iter()
        .map(|n| n.chars().nth(0).unwrap())
        .collect::<String>()
}

fn execute_instructions(boxes: &mut Boxes, instructions: Instructions) {
    for (n_moves, from, to) in instructions {
        for _ in 0..n_moves {
            if let Some(f) = boxes.get_mut(from) {
                let n = f.remove(0);

                if let Some(t) = boxes.get_mut(to) {
                  t.insert(0, n);
                }
            }
        }
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), String::from("CMZ"))
}

fn part2(file: &'static str) -> String {
    let (mut boxes, instructions) = parse_input(file);
    execute_instructions_9001(&mut boxes, instructions);
    boxes
        .iter()
        .map(|n| n.chars().nth(0).unwrap())
        .collect::<String>()
}

fn execute_instructions_9001(
    boxes: &mut Boxes,
    instructions: Instructions
) {
    for (n, from, to) in instructions {
        if let Some(f) = boxes.get_mut(from) {
            let m = String::from(f.get(0..n).unwrap());
            f.replace_range(0..n, "");

            if let Some(t) = boxes.get_mut(to) {
                t.insert_str(0, &m);
            }
        }
    }
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), String::from("MCD"))
}

fn parse_input(file: &'static str) -> (Boxes, Instructions) {
    let content = fs::read_to_string(file).unwrap();
    let boxes_instr: Vec<&str> = content.split("\n\n").collect();
    let boxes = parse_boxes(boxes_instr[0]);
    let instructions = parse_instructions(boxes_instr[1]);

    (boxes, instructions)
}

fn parse_boxes(box_string: &str) -> Boxes {
    let boxes_strs: Vec<&str> = box_string.split("\n").collect();
    let mut boxes: Boxes = vec![];

    for i in 0..(boxes_strs.len() - 1) {
        for (i, l) in boxes_strs[i].chars().enumerate() {
            if i % 4 != 1 {
                continue
            }

            let ind = i / 4;
            let cur_box = boxes.get_mut(ind);

            match cur_box {
                Some(x) => x.push(l),
                None => boxes.push(String::from(l))
            }
        }
    }

    boxes.iter().map(|n| String::from(n.trim())).collect()
}

fn parse_instructions(inst_string: &str) -> Instructions {
    let mut instructions: Instructions = vec![];

    for inst in inst_string.split_terminator("\n") {
        let mut result = vec![];
        for (i, n) in inst.split(" ").enumerate() {
            if i % 2 == 1 {
                result.push(n.parse::<usize>().unwrap());
            }
        }

        instructions.push((result[0], result[1] - 1, result[2] - 1));
    }

    instructions
}
