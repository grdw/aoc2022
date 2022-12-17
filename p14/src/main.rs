use std::fs;
use std::{thread, time::Duration};

const SAND_X: i16 = 500;
const SAND_Y: i16 = 0;

#[derive(Debug)]
enum PointType {
    Rock,
    Sand
}

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
    point_type: PointType
}

impl Point {
    fn rock(x: i16, y: i16) -> Point {
        Point { x: x, y: y, point_type: PointType::Rock }
    }

    fn sand(x: i16, y: i16) -> Point {
        Point { x: x, y: y, point_type: PointType::Sand }
    }
}

type Points = Vec<Point>;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let mut points = parse_paths(file);
    let mut sand_count = 0;
    let mut max_y = 0;

    for p in &points[0..points.len()-1] {
        if p.y > max_y { max_y = p.y }
    }

    while is_air(&points, SAND_X, SAND_Y) && !is_void(&points, max_y) {
        let y = lowest_y(&points);
        let sand_point = Point::sand(SAND_X, SAND_Y + y);
        points.push(sand_point);

        let mut i = 1;
        let mut j = 0;
        let l = points.len() - 1;

        loop {
            if is_air(&points, SAND_X + j, SAND_Y + i) {
                let p_m = points.get_mut(l).unwrap();
                p_m.y = SAND_Y + i;
                p_m.x = SAND_X + j;
                i += 1;
            } else if is_air(&points, SAND_X + j - 1, SAND_Y + i) {
                j -= 1;
            } else if is_air(&points, SAND_X + j + 1, SAND_Y + i) {
                j += 1;
            } else {
                break;
            }

            if is_void(&points, max_y) {
                break;
            }
        }
        //debug(&points);
        sand_count += 1;
    }

    sand_count - 1
}

fn lowest_y(points: &Points) -> i16 {
    points
        .iter()
        .filter(|&n| n.x == SAND_X)
        .map(|n| n.y)
        .min()
        .unwrap() - 1
}

fn is_air(points: &Points, x: i16, y: i16) -> bool {
    !points.iter().any(|n| n.x == x && n.y == y)
}

fn is_void(points: &Points, max_y: i16) -> bool {
    let curr_sand = &points[points.len() - 1];

    max_y < curr_sand.y
}

fn debug(points: &Points) {
    let mut max_x = 0;
    let mut min_x = i16::MAX;
    let mut max_y = 0;
    let min_y = 0;

    for p in points {
        if p.x > max_x { max_x = p.x }
        if p.x < min_x { min_x = p.x }
        if p.y > max_y { max_y = p.y }
    }

    // This is the grid
    let mut grid: Vec<Vec<char>> = vec![];

    for _ in min_y..=max_y {
        let mut sub_grid = vec![];
        for _ in min_x..=max_x {
            sub_grid.push('.');
        }
        grid.push(sub_grid);
    }

    for p in points {
        let c = match p.point_type {
            PointType::Rock => '#',
            PointType::Sand => '+'
        };
        grid[p.y as usize][(p.x - min_x) as usize] = c;
    }

    println!("");
    for g in &grid {
        println!("{}", g.into_iter().collect::<String>())
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 24);
}

fn parse_paths(file: &'static str) -> Points {
    let file = fs::read_to_string(file).unwrap();
    let mut points: Points = vec![];

    for path in file.split_terminator("\n") {
        let paths: Vec<&str> = path.split(" -> ").collect();

        for i in 0..paths.len()-1 {
            let coord_start = paths[i];
            let coord_end = paths[i + 1];
            let (fx, fy) = coord_start.split_once(",").unwrap();
            let (ex, ey) = coord_end.split_once(",").unwrap();

            let pfx = fx.parse::<i16>().unwrap();
            let pfy = fy.parse::<i16>().unwrap();
            let pex = ex.parse::<i16>().unwrap();
            let pey = ey.parse::<i16>().unwrap();

            let (rsx, rex) = if pfx <= pex {
                (pfx, pex)
            } else {
                (pex, pfx)
            };

            let (rsy, rey) = if pfy <= pey {
                (pfy, pey)
            } else {
                (pey, pfy)
            };

            for y in rsy..=rey {
                for x in rsx..=rex {
                    points.push(Point::rock(x, y));
                }
            }
        }
    }
    points
}

fn part2(file: &'static str) -> usize {
    let mut points = parse_paths(file);
    let mut sand_count = 0;
    let mut max_y = 0;

    for p in &points[0..points.len()-1] {
        if p.y > max_y { max_y = p.y }
    }

    max_y += 2;

    while is_air_f(&points, SAND_X, SAND_Y, max_y) {
        let y = lowest_y(&points);
        let sand_point = Point::sand(SAND_X, SAND_Y + y);
        points.push(sand_point);

        let mut i = 1;
        let mut j = 0;
        let l = points.len() - 1;

        loop {
            if is_air_f(&points, SAND_X + j, SAND_Y + i, max_y) {
                let p_m = points.get_mut(l).unwrap();
                p_m.y = SAND_Y + i;
                p_m.x = SAND_X + j;
                i += 1;
            } else if is_air_f(&points, SAND_X + j - 1, SAND_Y + i, max_y) {
                j -= 1;
            } else if is_air_f(&points, SAND_X + j + 1, SAND_Y + i, max_y) {
                j += 1;
            } else {
                break;
            }
        }
        //debug(&points);
        sand_count += 1;
    }

    sand_count
}

fn is_air_f(points: &Points, x: i16, y: i16, max_y: i16) -> bool {
    if y == max_y { return false }
    !points.iter().any(|n| n.x == x && n.y == y)
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 93);
}