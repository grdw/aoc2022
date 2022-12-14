use std::fs;

type Boxes = Vec<String>;
type Instructions = Vec<(usize, usize, usize)>;

fn main() {
    println!("Part 1: {}", parse_and_execute("input", exec_instructions_9000));
    println!("Part 2: {}", parse_and_execute("input", exec_instructions_9001));
}

fn parse_and_execute(file: &'static str, exec: fn(&mut Boxes, Instructions)) -> String {
    let (mut boxes, instructions) = parse_input(file);
    exec(&mut boxes, instructions);
    boxes
        .iter()
        .map(|n| n.chars().nth(0).unwrap())
        .collect::<String>()
}

fn exec_instructions_9000(boxes: &mut Boxes, instructions: Instructions) {
    for (n_moves, from, to) in instructions {
        for _ in 0..n_moves {
            let n = boxes[from].remove(0);
            boxes[to].insert(0, n);
        }
    }
}

#[test]
fn test_part1() {
    assert_eq!(
        parse_and_execute("test_input", exec_instructions_9000),
        String::from("CMZ")
    )
}

fn exec_instructions_9001(boxes: &mut Boxes, instructions: Instructions) {
    for (n, from, to) in instructions {
        let f = &mut boxes[from];
        let m = String::from(f.get(0..n).unwrap());
        f.replace_range(0..n, "");

        let t = &mut boxes[to];
        t.insert_str(0, &m);
    }
}

#[test]
fn test_part2() {
    assert_eq!(
        parse_and_execute("test_input", exec_instructions_9001),
        String::from("MCD")
    )
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
        for (i, c) in boxes_strs[i].chars().enumerate() {
            if i % 4 != 1 || c == ' ' {
                continue
            }

            let ind = i / 4;
            let cur_box = boxes.get_mut(ind);

            match cur_box {
                Some(x) => x.push(c),
                None => {
                    boxes.resize(ind, String::from(""));
                    boxes.insert(ind, String::from(c))
                }
            }
        }
    }

    boxes
}

fn parse_instructions(inst_string: &str) -> Instructions {
    inst_string.split_terminator("\n").map(|inst| {
        let mut result = vec![];
        for (i, n) in inst.split(" ").enumerate() {
            if i % 2 == 1 {
                result.push(n.parse::<usize>().unwrap());
            }
        }

        (result[0], result[1] - 1, result[2] - 1)
    }).collect()
}
