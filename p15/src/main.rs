use std::fs;
use std::collections::HashSet;
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

type Grid = Vec<Vec<char>>;

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
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str, pos: usize) -> usize {
    let pairs = parse(file);
    count_bonqs(&pairs, pos)
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input", 10), 26);
}

fn part2(file: &'static str) -> usize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 26);
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

fn debug(grid: &Grid) {
    println!("");
    for line in grid {
        println!("{}", line.into_iter().collect::<String>());
    }
}

fn count_bonqs(pairs: &PointPairs, pos: usize) -> usize {
    let mut bonq_counts: HashSet<(isize, isize)> = HashSet::new();
    let mut min_y = i32::MAX;
    let mut max_y = 0;
    let mut min_x = i32::MAX;
    let mut max_x = 0;

    for (sensor, beacon) in pairs {
        if sensor.x > max_x { max_x = sensor.x }
        if sensor.x < min_x { min_x = sensor.x }
        if sensor.y > max_y { max_y = sensor.y }
        if sensor.y < min_y { min_y = sensor.y }
        if beacon.x > max_x { max_x = beacon.x }
        if beacon.x < min_x { min_x = beacon.x }
        if beacon.y > max_y { max_y = beacon.y }
        if beacon.y < min_y { min_y = beacon.y }
    }

    let h = (max_y - min_y) as usize;
    let w = (max_x - min_x) as usize + 1;

    println!("{} {}", w, h);

    for (sensor, beacon) in pairs {
        let sy = (sensor.y - min_y) as usize;
        let sx = (sensor.x - min_x) as usize;
        let by = (beacon.y - min_y) as usize;
        let bx = (beacon.x - min_x) as usize;

        let dist =
            (beacon.x - sensor.x).abs() +
            (beacon.y - sensor.y).abs();

        let mut ty = -(dist as isize);
        let top = 0..=dist;
        let bottom = (0..=dist-1).rev();

        println!("AAAAAAAA {}", dist);
        for i in top.chain(bottom) {
            let range = -i..=i;
            let cy = ty + (sy as isize);
            ty += 1;

            if cy != (pos as isize) {
                continue
            }

            for n in range {
                let cx = (sx as isize) + (n as isize);

                // If it hits a scanner
                if cx == sx as isize && cy == sy as isize {
                    continue
                };

                // If it hits a beacon
                if cx == bx as isize && cy == by as isize {
                    continue
                };

                if cy == (pos as isize) {
                    bonq_counts.insert((cx, cy));
                }
            }
        }
    }

    bonq_counts.len()
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
