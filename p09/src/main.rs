use std::fs;
use std::collections::HashMap;

type Directions = Vec<(char, isize)>;
type Spots = HashMap<String, u64>;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(input: &'static str) -> usize {
    let directions = parse(input);
    let (mut hx, mut hy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    let mut prev_dir = ' ';
    let mut spots: Spots = HashMap::new();

    spots.insert("00".to_string(), 1);

    for (dir, steps) in &directions {
        // println!("----- {}", dir);
        // These are all the positions H is in:
        for _ in 0..*steps {
            match dir {
                'L' => hx -= 1,
                'R' => hx += 1,
                'D' => hy -= 1,
                'U' => hy += 1,
                _ => panic!("BOOYA")
            };

            let x_diff = (hx - tx) as isize;
            let y_diff = (hy - ty) as isize;

            //match (x_diff, y_diff) {
            //    (2, 0) => tx += 1,
            //    (2, -1) => {
            //        ty -= 1;
            //        tx += 1
            //    },
            //    (-2, 0) => tx -= 1,
            //    (-2, 1) => {
            //        ty += 1;
            //        tx -= 1
            //    },
            //    (-2, -1) => {
            //        ty -= 1;
            //        tx -= 1
            //    },
            //    (0, 2) => ty += 1,
            //    (1, 2) => {
            //        tx += 1;
            //        ty += 1
            //    },
            //    (0, -2) => ty -= 1,
            //    (_, _) => ()
            //}

            let key = format!("{}{}", tx, ty);
            spots.entry(key).and_modify(|n| *n += 1).or_insert(1);
        };
        prev_dir = *dir;
    }

    spots.keys().len()
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 13);
}

fn part2(input: &'static str) -> u64 {
    let directions = parse(input);
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(input: &'static str) -> Directions {
	let file = fs::read_to_string(input).unwrap();

    file.split_terminator("\n").map(|line| {
        let chars: Vec<char> = line.chars().collect();

        (chars[0], (chars[2] as u8 - 48) as isize)
    }).collect()
}
