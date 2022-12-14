use std::fs;
use std::collections::HashSet;

const SAND_X: i16 = 500;
const SAND_Y: i16 = 0;

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16
}

impl Point {
    fn new(x: i16, y: i16) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Debug)]
struct Line { start: Point, end: Point }

impl Line {
    pub fn fits(&self, x: i16, y: i16) -> bool {
        (self.start.x..=self.end.x).contains(&x) &&
        (self.start.y..=self.end.y).contains(&y)
    }
}

type Points = HashSet<(i16, i16)>;
type Lines = Vec<Line>;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn highest_y(lines: &Lines) -> i16 {
    let mut max_y = 0;

    for l in lines {
        if l.end.y > max_y {
            max_y = l.end.y
        }
    }

    max_y
}

fn part1(file: &'static str) -> usize {
    let lines = parse_lines(file);

    drop_sand(&lines, is_air_lines_v) - 1
}

fn is_air_lines_v(
    lines: &Lines,
    points: &Points,
    x: i16,
    y: i16,
    ny: i16,
    max_y: i16) -> bool {

    let not_void = max_y >= (ny - 1);

    not_void && is_air_lines(lines, points, x, y)
}

fn is_air_lines(lines: &Lines, points: &Points, x: i16, y: i16) -> bool {
    !(
        points.contains(&(x, y)) ||
        lines.iter().any(|l| l.fits(x, y))
    )
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 24);
}

fn part2(file: &'static str) -> usize {
    let lines = parse_lines(file);

    drop_sand(&lines, is_air_lines_f)
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 93);
}

fn is_air_lines_f(
    lines: &Lines,
    points: &Points,
    x: i16,
    y: i16,
    _ny: i16,
    max_y: i16) -> bool {

    if y == max_y { return false }

    is_air_lines(lines, points, x, y)
}

fn drop_sand(
    lines: &Lines,
    is_air: fn(&Lines, &Points, i16, i16, i16, i16) -> bool) -> usize {

    let mut prev_y = 0;
    let mut sand_count = 0;
    let mut points = HashSet::new();
    let max_y = highest_y(&lines) + 2;

    while is_air(&lines, &points, SAND_X, SAND_Y, prev_y, max_y) {
        let mut sand_point = Point::new(SAND_X, SAND_Y);
        let mut i = 1;
        let mut j = 0;

        loop {
            if is_air(&lines, &points, SAND_X + j, SAND_Y + i, SAND_Y + i, max_y) {
                sand_point.y = SAND_Y + i;
                sand_point.x = SAND_X + j;
                i += 1;
            } else if is_air(&lines, &points, SAND_X + j - 1, SAND_Y + i, SAND_Y + i, max_y) {
                j -= 1;
            } else if is_air(&lines, &points, SAND_X + j + 1, SAND_Y + i, SAND_Y + i, max_y) {
                j += 1;
            } else {
                break;
            }
        }

        points.insert((sand_point.x, sand_point.y));
        prev_y = SAND_Y + i;
        sand_count += 1;
    }

    sand_count
}

fn parse_lines(file: &'static str) -> Lines {
    let file = fs::read_to_string(file).unwrap();
    let mut lines: Lines = vec![];

    for path in file.split_terminator("\n") {
        let paths: Vec<&str> = path.split(" -> ").collect();

        for i in 0..paths.len()-1 {
            let (pfx, pfy) = to_coords(paths[i]);
            let (pex, pey) = to_coords(paths[i + 1]);
            let (rsx, rex) = max_t(pfx, pex);
            let (rsy, rey) = max_t(pfy, pey);

            lines.push(
                Line {
                    start: Point {
                        x: rsx,
                        y: rsy
                    },

                    end: Point {
                        x: rex,
                        y: rey
                    }
                }
            );
        }
    }
    lines
}

fn max_t(x1: i16, x2: i16) -> (i16, i16) {
    if x1 <= x2 {
        (x1, x2)
    } else {
        (x2, x1)
    }
}

fn to_coords(coord: &str) -> (i16, i16) {
    let (x, y) = coord.split_once(",").unwrap();
    let px = x.parse::<i16>().unwrap();
    let py = y.parse::<i16>().unwrap();

    (px, py)
}
