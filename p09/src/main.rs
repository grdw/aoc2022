use std::fs;
use std::cmp;
use std::collections::HashSet;

type Directions = Vec<(char, isize)>;
type Spots = HashSet<(isize, isize)>;

#[derive(Debug, PartialEq)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(input: &'static str) -> usize {
    let directions = parse(input);
    walk(&directions)
}

fn walk(directions: &Directions) -> usize {
    let (mut hx, mut hy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    let mut spots: Spots = HashSet::new();

    println!("======== INITIAL STATE:");
    debug_grid(hx, hy, tx, ty);
    for (dir, steps) in directions {
        for _ in 0..*steps {
            // These are all the positions H is in:
            match dir {
                'L' => hx -= 1,
                'R' => hx += 1,
                'D' => hy -= 1,
                'U' => hy += 1,
                _ => panic!("BOOYA")
            };


            let x_diff = hx - tx;
            let y_diff = hy - ty;

            let dirs = match (x_diff, y_diff) {
                (2, 0)              => vec![Dir::RIGHT],
                (-2, 0)             => vec![Dir::LEFT],
                (0, 2)              => vec![Dir::UP],
                (0, -2)             => vec![Dir::DOWN],
                (1, 2) | (2, 1)     => vec![Dir::UP, Dir::RIGHT],
                (1, -2) | (2, -1)   => vec![Dir::DOWN, Dir::RIGHT],
                (-1, 2) | (-2, 1)   => vec![Dir::UP, Dir::LEFT],
                (-1, -2) | (-2, -1) => vec![Dir::DOWN, Dir::LEFT],
                _                   => vec![]
            };

            for d in &dirs {
                match d {
                    Dir::UP    => ty += 1,
                    Dir::DOWN  => ty -= 1,
                    Dir::LEFT  => tx -= 1,
                    Dir::RIGHT => tx += 1
                }
            }

            println!("{} {} {} {}", hx, hy, tx, ty);
            debug_grid(hx, hy, tx, ty);
        };
    }

    spots.len()

}

fn debug_grid(hx: isize, hy: isize, tx: isize, ty: isize) {
    let min_x = vec![hx, tx].into_iter().min().unwrap();
    let max_x = vec![hx, tx].into_iter().max().unwrap() + 4;
    let min_y = vec![hy, ty].into_iter().min().unwrap();
    let max_y = vec![hy, ty].into_iter().max().unwrap() + 4;

    for i in min_y..=max_y {
        let mut row = String::new();
        for j in min_x..=max_x {
            let c = if i == (max_y - hy) && j == hx { 'H' }
                    else if i == (max_y - ty) && j == tx { 'T' }
                    else { '.' };

            row.push(c);
        }
        println!("{}", row)
    }

    println!("");
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
