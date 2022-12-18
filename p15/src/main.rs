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
    //let grid = to_grid(&pairs);
    draw_diamond(1);
    draw_diamond(2);
    draw_diamond(3);
    draw_diamond(4);
    draw_diamond(5);
    //debug(&grid);
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

fn debug(grid: &Grid) {
    println!("");
    for line in grid {
        println!("{}", line.into_iter().collect::<String>());
    }
}

fn to_grid(pairs: &PointPairs) -> Grid {
    let mut grid: Grid = vec![];
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

    let h = grid.len();
    let w = grid[0].len();
    for (sensor, beacon) in pairs {
        let sy = (sensor.y - min_y) as usize;
        let sx = (sensor.x - min_x) as usize;
        let by = (beacon.y - min_y) as usize;
        let bx = (beacon.x - min_x) as usize;

        let dist: usize = (
            (beacon.x - sensor.x).abs() +
            (beacon.y - sensor.y).abs()
        ) as usize;

        println!("\n\n==== DEBUG START");
        println!("-------- DISTANCE {:?}", dist);
        let mut n = 1;
        loop {
            // 2 is arbitary right now but it could by any N
            if n > dist {
                break;
            }

            println!("-----");
            println!("{}", n);

            //if sy + n < h  {
            //    grid[sy + n][sx] = '#'
            //}

            //if sy >= n {
            //    grid[sy - n][sx] = '#'
            //}

            //if sx + n < w {
            //    grid[sy][sx + n] = '#'
            //}

            //if sx >= n {
            //    grid[sy][sx - n] = '#'
            //}

            for p in 0..n {
                let tx = 1;
                let ty = 1 + p;
                println!("{} {}", tx, ty);
                println!("-{} {}", tx, ty);
                println!("{} -{}", tx, ty);
                println!("-{} -{}", tx, ty);
            }
            //for m in 1..=n {
            //    println!("-{}, {}", m, m);
            //    println!("{}, -{}", m, m);
            //    println!("{}, {}", m, m);
            //    println!("-{}, -{}", m, m);
            //    //if sy >= n {
            //    //    grid[sy - m][sx - m] = '#';
            //    //    grid[sy + m][sx - m] = '#';
            //    //    grid[sy + m][sx + m] = '#';
            //    //    grid[sy - m][sx + m] = '#';
            //    //}
            //}
            //for tx in 0..=i {
            //    //println!("COORD: {}", i, j);
            //}
            //(sensor.x - 1, sensor.y)
            //(sensor.x + 1, sensor.y)
            //(sensor.x, sensor.y)
            //(sensor.x, sensor.y)
            //
            n += 1;

        }

        grid[sy][sx] = 'S';
        grid[by][bx] = 'B';

        println!("S: ({},{}), B: ({},{})", sensor.x, sensor.y, beacon.x, beacon.y);
        println!("Delta: ({},{})", sensor.x - beacon.x, sensor.y - beacon.y);

    }

    grid
}

fn draw_diamond(size: usize) {
    //let g_size = 10;

    // Makes a simple 10 by 10 grid:
    //let mut v: Grid = vec![];
    //for h in 0..=g_size {
    //    let mut sub = vec![];
    //    for w in 0..=g_size {
    //        sub.push('.');
    //    }
    //    v.push(sub);
    //}

    // Draws the S in the middle;
    //v[g_size / 2][g_size / 2] = 'S';

    //for k in 0..size {
    //    let l = ((size + 1) / 2) - k - 1;
    //    println!("{}", l);
    //    if l > 0 {
    //        v[k][l] = '#';
    //    }
    //}

    for i in 0..=size {
        let num_spaces = (size + 1) - i - 1;
        for n in 0..num_spaces {
            print!(" ");
        }

        for k in (0..=i).rev() {
            print!("#");
        }

        for j in 1..=i {
            print!("#");
        }
        println!("");
    }

    for i in (0..=size-1).rev() {
        let num_spaces = (size + 1) - i - 1;
        for n in 0..num_spaces {
            print!(" ");
        }

        for k in (0..=i).rev() {
            print!("#");
        }

        for j in 1..=i {
            print!("#");
        }
        println!("");
    }

    //debug(&v);
}
