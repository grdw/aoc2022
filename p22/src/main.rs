use std::fs;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
    face: usize,
    wall: bool
}

type Instructions = Vec<(u8, char)>;
type MonkeyMap = Vec<Point>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> isize {
    traverse_path(file, walk)
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 6032);
}

fn traverse_path(
    file: &'static str,
    walk: fn(map: &MonkeyMap, start: &mut Point, steps: u8, dir: char))
    -> isize
{
    let (map, instr) = parse(file);
    let mut dir = '>';
    let mut start = Point {
        x: map[0].x,
        y: map[0].y,
        face: map[0].face,
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

    (1000 * (start.y + 1)) + (4 * (start.x + 1)) + facing_value

}

fn walk(map: &MonkeyMap, start: &mut Point, steps: u8, dir: char) {
    let n_min_y = map.iter().filter(|p| p.x == start.x).map(|p| p.y).min().unwrap();
    let n_max_y = map.iter().filter(|p| p.x == start.x).map(|p| p.y).max().unwrap();
    let n_min_x = map.iter().filter(|p| p.y == start.y).map(|p| p.x).min().unwrap();
    let n_max_x = map.iter().filter(|p| p.y == start.y).map(|p| p.x).max().unwrap();
    let height = n_max_y;
    let width = n_max_x;

    for _ in 0..steps {
        let (mut nx, mut ny): (isize, isize) = match dir {
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => panic!("BOOM!")
        };

        if start.x + nx >= 0 {
            nx += start.x
        } else if start.x + nx >= n_max_x {
            nx += n_min_x
        } else if start.x + nx <= n_min_x {
            nx += n_max_x
        }

        if start.y + ny >= 0 {
            ny += start.y
        } else if start.y + ny >= n_max_y {
            ny += n_min_y
        } else if start.y + ny <= n_min_y {
            ny += n_max_y
        }

        nx %= width;
        ny %= height;

        let is_wall = map
            .iter()
            .find(|p| p.x == nx && p.y == ny && p.wall)
            .is_some();

        if is_wall {
            break;
        }

        start.x = nx;
        start.y = ny;

    }
}

fn walk_cube(map: &MonkeyMap, start: &mut Point, steps: u8, dir: char) {
    let n_min_y = map.iter().filter(|p| p.x == start.x).map(|p| p.y).min().unwrap();
    let n_max_y = map.iter().filter(|p| p.x == start.x).map(|p| p.y).max().unwrap();
    let n_min_x = map.iter().filter(|p| p.y == start.y).map(|p| p.x).min().unwrap();
    let n_max_x = map.iter().filter(|p| p.y == start.y).map(|p| p.x).max().unwrap();
    let height = n_max_y;
    let width = n_max_x;

    for _ in 0..steps {
        let (mut nx, mut ny): (isize, isize) = match dir {
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => panic!("BOOM!")
        };

        if start.x + nx >= 0 {
            nx = (start.x + nx) % width;
        } else if start.x + nx >= n_max_x {
            nx = (n_min_x + nx) % width;
        } else if start.x + nx <= n_min_x {
            nx = (n_max_x + nx) % width;
        }

        if start.y + ny >= 0 {
            ny = (start.y + ny) % height;
        } else if start.y + ny >= n_max_y {
            ny = (n_min_y + ny) % height;
        } else if start.y + ny <= n_min_y {
            ny = (n_max_y + ny) % height;
        }

        let is_wall = map
            .iter()
            .find(|p| p.x == nx && p.y == ny && p.wall)
            .is_some();

        if is_wall {
            break;
        }

        start.x = nx;
        start.y = ny;

    }
}

fn part2(file: &'static str) -> isize {
    traverse_path(file, walk_cube)
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 5031);
}

fn parse(file: &'static str) -> (MonkeyMap, Instructions) {
    let text = fs::read_to_string(file).unwrap();

    let mut dir_map = vec![];
    let mut parsed_instructions = vec![];
    let (map, instructions) = text.split_once("\n\n").unwrap();

    let lines: Vec<&str> = map.split("\n").collect();
    let cube_edge_size = lines[0].trim().len();
    let mut c = 0;
    //println!("{}", cube_edge_size);
    for (y, line) in lines.iter().enumerate() {
        for (x, p) in line.chars().enumerate() {
            if p == ' ' { continue }

            dir_map.push(
                Point {
                    y: y as isize,
                    x: x as isize,
                    face: c / cube_edge_size.pow(2),
                    wall: p == '#'
                }
            );

            c += 1;
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
