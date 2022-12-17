use std::fs;

#[derive(Debug)]
enum PointType {
    Rock,
    Air,
    Sand,
    SandStale
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

    fn air(x: i16, y: i16) -> Point {
        Point { x: x, y: y, point_type: PointType::Air }
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
    debug(&points);

    let sand_x = 500;
    let sand_y = 0;

    let mut sand_count = 0;
    let transposes = vec![(0, 0), (-1, 1), (1, 1)];

    while is_air(&points, sand_x, sand_y) {
        let mut sand_point = Point::sand(sand_x, sand_y);
        points.push(sand_point);
        debug(&points);

        let mut i = 1;
        let mut j = 0;
        let l = points.len() - 1;
        let (t_x, t_y) = transposes[sand_count % transposes.len()];

        loop {
            if is_air(&points, sand_x + j, sand_y + i) {
                let p_m = points.get_mut(l).unwrap();
                p_m.y = sand_y + i;
                p_m.x = sand_x + j;
                i += 1;
            } else {
                j += t_x;
            }

            if !is_air(&points, sand_x + j, sand_y + i) &&
               !is_air(&points, sand_x + j + 1, sand_y + i) &&
               !is_air(&points, sand_x + j - 1, sand_y + i) {

                let p_m = points.get_mut(l).unwrap();
                p_m.point_type = PointType::SandStale;
                break;
            }

            debug(&points);
        }

        debug(&points);

        sand_count += 1;

        if sand_count == 5 {
            break;
        }
    }

    0
}

fn is_air(points: &Points, x: i16, y: i16) -> bool {
    !points.iter().any(|n| n.x == x && n.y == y)
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

    for y in min_y..=max_y {
        let mut sub_grid = vec![];
        for x in min_x..=max_x {
            sub_grid.push('.');
        }
        grid.push(sub_grid);
    }

    for p in points {
        let c = match p.point_type {
            PointType::Rock => '#',
            PointType::Sand => '+',
            PointType::SandStale => 'o',
            _ => ' '
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
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 24);
}
