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
    let mut dir = '>';
    let mut start = Point {
        x: map[0].x,
        y: map[0].y,
        wall: false
    };

    for (steps, rotation) in &instr {
        walk(&map, &mut start, *steps, dir);

        dir = match (rotation, dir) {
            ('R', '>') | ('L', '<') => 'v',
            ('R', 'v') | ('L', '^') => '<',
            ('R', '<') | ('L', '>') => '^',
            ('R', '^') | ('L', 'v') => '>',
            (_, _)                  => dir // stay the same
        }
    }

    let facing_value = match dir {
        '>' => 0,
        'v' => 1,
        '<' => 2,
        '^' => 3,
        _ => panic!("Invalid direction")
    };

    println!("{:?} {}", start, facing_value);
    (1000 * (start.y + 1)) + (4 * (start.x + 1)) + facing_value
}

fn walk(map: &MonkeyMap, start: &mut Point, steps: u8, dir: char) {
    let n_min_y = map.iter().filter(|p| p.x == start.x).map(|p| p.y).min().unwrap();
    let n_max_y = map.iter().filter(|p| p.x == start.x).map(|p| p.y).max().unwrap();
    let n_min_x = map.iter().filter(|p| p.y == start.y).map(|p| p.x).min().unwrap();
    let n_max_x = map.iter().filter(|p| p.y == start.y).map(|p| p.x).max().unwrap();
    let height = (n_max_y + 1) - n_min_y;
    let width = (n_max_x + 1) - n_min_x;

    for _ in 0..steps {
        let mut next_wall = false;
        match dir {
            'v' => {
                let next = n_min_y + (start.y + 1) % height;

                next_wall = map
                    .iter()
                    .find(|p| p.x == start.x && p.y == next && p.wall)
                    .is_some();

                if next_wall {
                    break;
                }

                //println!("v {:?}", start);
                start.y = next;
            },
            '<' => {
                let next = if start.x == 0 {
                    n_min_x + n_max_x % width
                } else {
                    n_min_x + (start.x - 1) % width
                };

                next_wall = map
                    .iter()
                    .find(|p| p.x == next && p.y == start.y && p.wall)
                    .is_some();

                if next_wall {
                    break;
                }
                //println!("< {:?}", start);
                start.x = next;
            },
            '^' => {
                let next = if start.y == 0 {
                    n_min_y + n_max_y % height
                } else {
                    n_min_y + (start.y - 1) % height
                };

                next_wall = map
                    .iter()
                    .find(|p| p.x == start.x && p.y == next && p.wall)
                    .is_some();

                if next_wall {
                    break;
                }
                //println!("^ {:?}", start);
                start.y = next;
            },
            '>' => {
                let next = n_min_x + (start.x + 1) % width;

                next_wall = map
                    .iter()
                    .find(|p| p.x == next && p.y == start.y && p.wall)
                    .is_some();

                if next_wall {
                    break;
                }

                start.x = next;
                //println!("> {:?}", start);
            },
            _ => panic!("BOOM!")
        }

    }
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 6032);
    assert_eq!(part1("test_input2"), 4046);
    assert_eq!(part1("test_input3"), 9057);
    assert_eq!(part1("test_input4"), 12043);
    assert_eq!(part1("test_input5"), 5006);
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
