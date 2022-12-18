use std::fs;
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
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str, pos: usize) -> usize {
    let pairs = parse(file);
    debug(&pairs);
    0
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

fn debug(pairs: &PointPairs) {
    let mut grid: Vec<Vec<char>> = vec![];
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

    for y in min_y..=max_y {
        let mut s = vec![];

        for x in min_x..=max_x {
            s.push('.');
        }

        grid.push(s);
    }

    for (sensor, beacon) in pairs {
        let sy = (sensor.y - min_y) as usize;
        let sx = (sensor.x - min_x) as usize;
        let by = (beacon.y - min_y) as usize;
        let bx = (beacon.x - min_x) as usize;

        grid[sy][sx] = 'S';
        grid[by][bx] = 'B';
    }

    for line in grid {
        println!("{}", line.into_iter().collect::<String>());
    }
}
