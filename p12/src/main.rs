use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type HeightMap = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Edge(usize);
type Edges = Vec<Vec<Edge>>;

fn main() {
    println!("Part 1: {:?}", part1("input"));
    println!("Part 2: {:?}", part2("input"));
}

fn part1(input: &'static str) -> Option<usize> {
    let height_map = parse(input);
    let width = height_map[0].len();
    let height = height_map.len();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..height {
        for x in 0..width {
            let current = height_map[y][x];

            if current == 'S' {
                start = (y, x);
            }

            if current == 'E' {
                end = (y, x);
            }
        }
    }

    let start_id = (start.0 * width) + start.1;
    let goal_id = (end.0 * width) + end.1;
    let edges = get_edges(&height_map);

    dijkstra(&edges, start_id, goal_id)
}

fn get_edges(height_map: &HeightMap) -> Edges{
    let width = height_map[0].len();
    let height = height_map.len();
    let directions: Vec<(isize, isize)> = vec![
        (-1, 0), (0, -1), (1, 0), (0, 1)
    ];
    let mut edges: Edges = vec![vec![]; height * width];

    for y in 0..height {
        for x in 0..width {
            let current = height_map[y][x];
            let ix = x as isize;
            let iy = y as isize;

            let id = (y * width) + x;
            for (dy, dx) in &directions {
                let ty = (iy + dy) as usize;
                let tx = (ix + dx) as usize;
                if let Some(row) = height_map.get(ty) {
                    if let Some(cell) = row.get(tx) {
                        let n = score_char(current);
                        let m = score_char(*cell);

                        if m - n < 2 {
                            let other_id = (ty * width) + tx;
                            edges[id].push(Edge(other_id));
                        }
                    }
                }
            }
        }
    }

    edges
}

fn dijkstra(edges: &Edges, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..edges.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return Some(cost); }
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

fn score_char(n: char) -> i8 {
    let mut nc = n;
    if nc == 'S' {
        nc = 'a';
    } else if nc == 'E' {
        nc = 'z';
    }

    (nc as u8) as i8
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), Some(31));
}

fn part2(input: &'static str) -> Option<usize> {
    let height_map = parse(input);
    let width = height_map[0].len();
    let height = height_map.len();
    let mut starts = vec![];
    let mut routes = vec![];
    let mut end = (0, 0);

    for y in 0..height {
        for x in 0..width {
            let current = height_map[y][x];

            if current == 'S' || current == 'a' {
                starts.push((y, x));
            }

            if current == 'E' {
                end = (y, x);
            }
        }
    }

    for (sy, sx) in starts {
        let start_id = (sy * width) + sx;
        let goal_id = (end.0 * width) + end.1;
        let edges = get_edges(&height_map);

        routes.push(dijkstra(&edges, start_id, goal_id));
    }

    routes.iter().map(|r| r.unwrap_or(9999)).min()
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), Some(29));
}

fn parse(input: &'static str) -> HeightMap {
	let file = fs::read_to_string(input).unwrap();

    file.split_terminator("\n").map(|line| {
        line.chars().collect()
    }).collect()
}
