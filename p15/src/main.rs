use std::fs;
use std::cmp;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum PointType {
    Sensor,
    Beacon
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    t: PointType
}

impl Point {
    fn sensor(x: i32, y: i32) -> Point {
        Point { x: x, y: y, t: PointType::Sensor }
    }

    fn beacon(x: i32, y: i32) -> Point {
        Point { x: x, y: y, t: PointType::Beacon }
    }
}

type PointPairs = Vec<(Point, Point)>;

fn main() {
    println!("P1: {}", part1("input", 2_000_000));
    println!("P2: {}", part2("input", 4_000_000));
}

fn part1(file: &'static str, pos: usize) -> usize {
    let pairs = parse(file);
    count_bonqs(&pairs, pos)
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input", 10), 26);
}

// Fucking nightmare code
fn count_bonqs(pairs: &PointPairs, pos: usize) -> usize {
    let mut lines = vec![];

    for (sensor, beacon) in pairs {
        let sy = sensor.y;
        let sx = sensor.x;
        let by = beacon.y;
        let bx = beacon.x;

        let dist = (bx - sx).abs() + (by - sy).abs();
        let top = 0..=dist;
        let bottom = (0..=dist-1).rev();

        let mut ty = -(dist as isize);

        for i in top.chain(bottom) {
            let cy = ty + (sy as isize);
            ty += 1;

            if cy != (pos as isize) {
                continue
            }

            let start = -i + sx;
            let end = i + sx;

            line_squash(&mut lines, (start, end));
        }

        merge(&mut lines);
    }

    (lines[0].0..lines[0].1).len()
}

fn line_squash(lines: &mut Vec<(i32, i32)>, point: (i32, i32)) {
    let len = lines.len();

    for i in 0..len {
        let mut l = lines.get_mut(i).unwrap();

        if is_wrapped(&point, l) {
            return;
        } else if is_wrapped(l, &point) {
            l.0 = point.0;
            l.1 = point.1;
            return;
        } else if is_start(point.0, l) {
            l.1 = point.1;
            return;
        } else if is_start(point.1, l) {
            l.0 = point.0;
            return;
        }
    }

    lines.push(point);
}

fn is_wrapped(p1: &(i32, i32), p2: &(i32, i32)) -> bool {
    p1.0 >= p2.0 && p1.1 <= p2.1
}

fn is_start(x: i32, p: &(i32, i32)) -> bool {
    x <= p.1 && x >= p.0
}

fn merge(lines: &mut Vec<(i32, i32)>) {
    lines.sort();

    let len = lines.len();

    for i in 0..len {
        for j in (i + 1)..len {
            let left = lines[i];
            let right = lines[j];

            if is_start(left.1, &right) {
                lines[i].1 = right.1;
                // Fucking ugly hack, but it works :')
                lines[j] = (0, 0);
            }
        }
    }

    lines.retain(|(a, b)| !(*a == 0 && *b == 0));
}

#[test]
fn test_line_squash_merge() {
    let mut lines = vec![(-1, 2), (7, 10)];
    merge(&mut lines);
    assert_eq!(lines.len(), 2);

    let mut lines = vec![(-1, 7), (7, 10)];
    merge(&mut lines);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], (-1, 10));

    let mut lines = vec![(7, 10), (0, 7)];
    merge(&mut lines);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], (0, 10));

    let mut lines = vec![(1,2), (3,4), (2,3)];
    merge(&mut lines);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], (1,4));

    let mut lines = vec![(1,2), (3,4), (2,3), (4,9)];
    merge(&mut lines);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], (1,9));

    let mut lines = vec![(1,2), (3,4), (2,3), (8,9)];
    merge(&mut lines);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], (1,4));
    assert_eq!(lines[1], (8,9));

    let mut lines = vec![(1,2), (3,4), (2,3), (8,9), (9,12)];
    merge(&mut lines);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], (1,4));
    assert_eq!(lines[1], (8,12));
}

#[test]
fn test_line_squash() {
    // Regular insert
    let mut lines = vec![(-1, 5)];
    line_squash(&mut lines, (12, 15));
    assert_eq!(lines.len(), 2);

    // Squash on the left-side
    let mut lines = vec![(-1, 5)];
    line_squash(&mut lines, (5, 10));
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], (-1, 10));

    // Squash on the left-side
    let mut lines = vec![(1, 5)];
    line_squash(&mut lines, (3, 10));
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], (1, 10));

    // Squash on the right-side
    let mut lines = vec![(3, 10)];
    line_squash(&mut lines, (1, 3));
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], (1, 10));

    // Squash on the right-side
    let mut lines = vec![(3, 10)];
    line_squash(&mut lines, (1, 5));
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], (1, 10));

    // Swallowed whole
    let mut lines = vec![(0, 10)];
    line_squash(&mut lines, (1, 5));
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], (0, 10));
}

fn part2(file: &'static str, max: isize) -> i32 {
    let pairs = parse(file);
    tuning_frequency(&pairs, max)
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input", 20), 56000011);
}

fn tuning_frequency(pairs: &PointPairs, max: isize) -> i32 {
    let mut m: HashMap<isize, Vec<(i32, i32)>> = HashMap::new();
    let mut tuning_frequency = 0;

    for (sensor, beacon) in pairs {
        let sy = sensor.y;
        let sx = sensor.x;
        let by = beacon.y;
        let bx = beacon.x;

        let dist = (bx - sx).abs() + (by - sy).abs();
        let top = 0..=dist;
        let bottom = (0..=dist-1).rev();

        let mut ty = -(dist as isize);

        for i in top.chain(bottom) {
            let cy = ty + (sy as isize);

            ty += 1;

            if cy < 0 || cy > max {
                continue
            }

            let cxs = cmp::max(-i + sx, 0);
            let cxe = cmp::min(i + sx, max as i32) + 1;

            m.entry(cy)
                .and_modify(|mut v| {
                    line_squash(&mut v, (cxs, cxe));
                    merge(&mut v)
                })
                .or_insert(vec![(cxs, cxe)]);
        }
    }

    m.retain(|_, val| val[0] != (0, (max as i32) + 1));

    for (y, x_lines) in m {
        tuning_frequency = x_lines[0].1 * 4_000_000 + (y as i32)
    }
    tuning_frequency
}

// A little helper method that draws pixel diamonds
// Thanks guy at this SO question: https://stackoverflow.com/questions/69681723/rhombus-with-letters-java
fn draw_diamond(size: usize) {
    for i in 0..=size {
        let num_spaces = size - i;
        for _ in 0..num_spaces {
            print!(".");
        }

        for _ in (0..=i).rev() {
            print!("#");
        }

        for _ in 1..=i {
            print!("#");
        }
        println!("");
    }

    for i in (0..=size-1).rev() {
        let num_spaces = size - i;
        for _ in 0..num_spaces {
            print!(".");
        }

        for _ in (0..=i).rev() {
            print!("#");
        }

        for _ in 1..=i {
            print!("#");
        }
        println!("");
    }
}

fn parse(file: &'static str) -> PointPairs {
    let contents = fs::read_to_string(file).unwrap();
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    contents.split_terminator("\n").map(|line| {
        let caps = re.captures(line).unwrap();
        let coords: Vec<i32> = (1..=4)
            .map(|n| caps[n].parse::<i32>().unwrap())
            .collect();

        (Point::sensor(coords[0], coords[1]),
         Point::beacon(coords[2], coords[3]))

    }).collect()
}

