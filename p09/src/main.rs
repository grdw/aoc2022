use std::fs;
use std::cmp;
use std::collections::HashMap;

type Directions = Vec<(char, isize)>;
type Spots = HashMap<String, u64>;

#[derive(Debug, PartialEq)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UPLEFT,
    UPRIGHT,
    DOWNLEFT,
    DOWNRIGHT,
    IDLE
}

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(input: &'static str) -> usize {
    let directions = parse(input);
    let (mut hx, mut hy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    let mut spots: Spots = HashMap::new();

    //spots.insert("00".to_string(), 1);

    for (dir, steps) in &directions {
        println!("========= {} {}", dir, steps);
        for _ in 0..*steps {
            // These are all the positions H is in:
            match dir {
                'L' => hx -= 1,
                'R' => hx += 1,
                'D' => hy -= 1,
                'U' => hy += 1,
                _ => panic!("BOOYA")
            };

            let x_diff = (hx - tx) as isize;
            let y_diff = (hy - ty) as isize;

            let d = match (x_diff, y_diff) {
                (-2, 0) => Dir::LEFT,
                (-2, 1) => Dir::UPLEFT,
                (-2, -1) => Dir::DOWNLEFT,
                (2, 0) => Dir::RIGHT,
                (2, 1) => Dir::UPRIGHT,
                (2, -1) => Dir::DOWNRIGHT,
                (0, -2) => Dir::DOWN,
                (1, -2) => Dir::DOWNRIGHT,
                (-1, -2) => Dir::DOWNLEFT,
                (0, 2) => Dir::UP,
                (1, 2) => Dir::UPRIGHT,
                (-1, 2) => Dir::UPLEFT,
                (_, _) => Dir::IDLE
            };

            if d != Dir::IDLE {
                print!("✨ {:?} @", d);
                //println!("{:?} H: ({},{}) T: ({},{})", d, hx, hy, tx, ty);
            }

            match d {
                Dir::UP => ty += 1,
                Dir::DOWN => ty -= 1,
                Dir::LEFT => tx -= 1,
                Dir::RIGHT => tx += 1,
                Dir::UPLEFT => {
                    tx -= 1;
                    ty += 1;
                },
                Dir::UPRIGHT => {
                    tx += 1;
                    ty += 1;
                },
                Dir::DOWNLEFT => {
                    tx -= 1;
                    ty -= 1;
                },
                Dir::DOWNRIGHT => {
                    tx += 1;
                    ty -= 1;
                },
                _ => {},
            }

            println!("--------- {} {}", x_diff, y_diff);
            debug_grid(hx, hy, tx, ty);

            let key = format!("{}-{}", tx, ty);
            spots.entry(key).and_modify(|n| *n += 1).or_insert(1);
        };
    }

    spots.keys().len()
}

fn debug_grid(hx: isize, hy: isize, tx: isize, ty: isize) {
    let min_x = vec![hx, tx, 0].into_iter().min().unwrap();
    let max_x = vec![hx, tx, 0].into_iter().max().unwrap();
    let min_y = vec![hy, ty, 0].into_iter().min().unwrap();
    let max_y = vec![hy, ty, 0].into_iter().max().unwrap();

    for i in ((min_y -2)..=max_y + 1).rev() {
        let mut row = String::new();
        for j in (min_x - 2)..=max_x + 1 {
            let c = if i == hy && j == hx { 'H' }
                    else if i == ty && j == tx { 'T' }
                    else { '.' };

            row.push(c);
        }
        println!("{}", row)
    }
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
