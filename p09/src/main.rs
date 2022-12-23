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
    walk(&directions, 2)
}

fn walk(directions: &Directions, knot_count: usize) -> usize {
    let mut knots = vec![(0,0); knot_count];
    let mut spots: Spots = HashSet::new();

    for (dir, steps) in directions {
        for _ in 0..*steps {
            // These are all the positions H is in:
            match dir {
                'L' => knots[0].0 -= 1,
                'R' => knots[0].0 += 1,
                'D' => knots[0].1 -= 1,
                'U' => knots[0].1 += 1,
                _ => panic!("BOOYA")
            };


            for i in 0..knot_count-1 {
                let h = knots[i];
                let t = &mut knots[i + 1];
                let dirs = get_dirs(h.0 - t.0, h.1 - t.1);
                for d in &dirs {
                    match d {
                        Dir::UP    => t.1 += 1,
                        Dir::DOWN  => t.1 -= 1,
                        Dir::LEFT  => t.0 -= 1,
                        Dir::RIGHT => t.0 += 1
                    }
                }

                if i + 1 == knot_count - 1 {
                    spots.insert(*t);
                }
            }

        };
    }

    spots.len()
}

fn get_dirs(x_diff: isize, y_diff: isize) -> Vec<Dir> {
    match (x_diff, y_diff) {
        (2, 0)              => vec![Dir::RIGHT],
        (-2, 0)             => vec![Dir::LEFT],
        (0, 2)              => vec![Dir::UP],
        (0, -2)             => vec![Dir::DOWN],
        (1, 2) | (2, 1)     => vec![Dir::UP, Dir::RIGHT],
        (1, -2) | (2, -1)   => vec![Dir::DOWN, Dir::RIGHT],
        (-1, 2) | (-2, 1)   => vec![Dir::UP, Dir::LEFT],
        (-1, -2) | (-2, -1) => vec![Dir::DOWN, Dir::LEFT],
        _                   => vec![]
    }
}

#[test]
fn test_walk() {
    let directions = vec![('R', 1), ('U', 2)];
    assert_eq!(walk(&directions, 2), 2);

    let directions = vec![('U', 1), ('R', 2)];
    assert_eq!(walk(&directions, 2), 2);

    let directions = vec![('D', 1), ('L', 2)];
    assert_eq!(walk(&directions, 2), 2);

    let directions = vec![('D', 2), ('L', 1)];
    assert_eq!(walk(&directions, 2), 2);

    let directions = parse("test_input2");
    assert_eq!(walk(&directions, 10), 36);
}

fn debug_grid(hx: isize, hy: isize, tx: isize, ty: isize) {
    let min_x = vec![hx, tx, 0].into_iter().min().unwrap() - 1;
    let max_x = vec![hx, tx, 0].into_iter().max().unwrap() + 1;
    let min_y = vec![hy, ty, 0].into_iter().min().unwrap() - 1;
    let max_y = vec![hy, ty, 0].into_iter().max().unwrap() + 1;

    for i in (min_y..=max_y).rev() {
        let mut row = String::new();
        for j in min_x..=max_x {
            let c = if i == hy && j == hx { 'H' }
                    else if i == ty && j == tx { 'T' }
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
        let (dir, num) = line.split_once(" ").unwrap();

        (dir.chars().nth(0).unwrap(), num.parse::<isize>().unwrap())
    }).collect()
}
