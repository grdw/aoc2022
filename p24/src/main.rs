use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Point {
    id: usize,
    x: isize,
    y: isize,
    p_type: char
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Edge(usize, usize);
type Edges = Vec<Vec<Edge>>;
type Grid = Vec<Vec<Point>>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let mut minutes = 0;
    let mut basin = parse(file);
    let (start_x, end_x, start_y, end_y) = maxes(&basin);
    let width = end_x;
    let height = end_y;

    //let mut start = Point {
    //    id: 0,
    //    x: start_x + 1,
    //    y: start_y,
    //    p_type: 'S'
    //};

    //let end = Point {
    //    id: ((end_x * end_y) + 1) as usize,
    //    x: end_x - 1,
    //    y: end_y,
    //    p_type: 'E'
    //};

    //let edges = get_edges(&basin, width, height);
    //debug_blizzards(&basin, &start);
    loop {
        minutes += 1;
        move_blizzards(&mut basin, height, width);
        let val = dijkstra(&basin, 1, ((end_x * end_y) - 1) as usize);
        println!("{:?}", val);
        //move_me(&mut start, &basin);

        //println!("{}", minutes);
        //debug_blizzards(&basin, &start);

        if minutes == 14 {
            break;
        }

        //if start.x == end_x && start.y == end_y {
        //    break;
        //}
    }

    minutes
}

fn maxes(grid: &Grid) -> (isize, isize, isize, isize) {
    let mi_x = 0;
    let mx_x = grid[0].len() as isize;
    let mi_y = 0;
    let mx_y = grid.len() as isize;

    (mi_x, mx_x, mi_y, mx_y)
}

fn move_blizzards(grid: &mut Grid, height: isize, width: isize) {
    for row in grid {
        for p in row {
            match p.p_type {
                'v' => p.y += 1,
                '>' => p.x += 1,
                '^' => p.y -= 1,
                '<' => p.x -= 1,
                _   => continue
            }

            if p.x == 0 {
                p.x = width - 1
            } else if p.x == width {
                p.x = 1
            }

            if p.y == 0 {
                p.y = height - 1
            } else if p.y == height {
                p.y = 1
            }
        }
    }
}

fn debug_blizzards(grid: &Grid, me: &Point) {
    let (min_x, max_x, min_y, max_y) = maxes(grid);

    for y in min_y..=max_y {
        let row = &grid[y as usize];
        let mut s = String::new();
        for x in min_x..=max_x {
            let f = row.iter().find(|p| p.x == x && p.y == y);

            match f {
                Some(p) => s.push(p.p_type),
                None => {
                    if me.x == x && me.y == y {
                        s.push('X');
                    } else {
                        s.push('.');
                    }
                }
            }
        }
        println!("{}", s);
    }
}

fn get_edges(grid: &Grid) -> Edges{
    let width = grid[0].len();
    let height = grid.len();
    let directions: Vec<(isize, isize)> = vec![
        (-1, 0), (0, -1), (1, 0), (0, 1)
    ];
    let mut edges: Edges = vec![vec![]; height * width];

    for y in 0..height {
        let row = &grid[y];
        for x in 0..width {
            let current = row.iter().find(|p| p.x == x as isize);

            if let Some(p) = current {
                if p.p_type == '#' {
                    continue
                }
            }

            let id = (y * width) + x;
            for (dy, dx) in &directions {
                let ty = (y as isize + dy) as usize;
                let tx = (x as isize + dx) as usize;

                if let Some(row) = grid.get(ty) {
                    let brother = row
                        .iter()
                        .find(|p| p.x == tx as isize);

                    if let Some(b) = brother {
                        if b.p_type == '#' {
                            continue
                        }
                    }

                    let b_id = (ty * width) + tx;
                    edges[id].push(Edge(b_id, 1));
                }
            }
        }
    }

    edges
}

fn dijkstra(basin: &Grid, start: usize, end: usize) -> Option<usize> {
    let mut edges: Edges = get_edges(basin);
    let mut dist: Vec<_> = (0..edges.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end { return Some(cost); }
        if cost > dist[position] { continue; }

        for edge in &edges[position] {
            let next = State { cost: cost + 1, position: edge.0 };

            if next.cost < dist[next.position] {
                heap.push(next);

                dist[next.position] = next.cost;
            }
        }
    }

    None
}

#[test]
fn test_part1() {
    //assert_eq!(part1("test_input"), 1);
    assert_eq!(part1("test_input2"), 1);
}

fn part2(file: &'static str) -> isize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(file: &'static str) -> Grid {
    let mut grid = vec![];
    let content = fs::read_to_string(file).unwrap();
    let mut id = 1;

    for (y, line) in content.split_terminator("\n").enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                row.push(
                    Point {
                        id: id,
                        x: x as isize,
                        y: y as isize,
                        p_type: c
                    }
                );
            }

            id += 1
        }

        grid.push(row);
    }

    grid
}
