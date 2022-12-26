use std::fs;
use std::collections::{HashMap, BinaryHeap, VecDeque};
use std::cmp::Ordering;
use regex::Regex;

const TIME: usize = 30;

#[derive(Debug, Clone)]
struct Edge(usize);

#[derive(Debug, Eq, PartialEq)]
struct Valve {
    id: usize,
    name: String,
    flow_rate: usize
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

type Edges = Vec<Vec<Edge>>;
type Valves = Vec<Valve>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
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

fn part1(file: &'static str) -> usize {
    let (mut valves, edges) = parse(file);
    let mut current = 0;
    let mut total_flow_rate = 0;
    let mut minutes = 30;

    valves.retain(|v| v.flow_rate > 0);
    sort_valves(&mut valves, &edges, current, minutes);

    while let Some(valve) = valves.pop() {
        let time = dijkstra(&edges, current, valve.id).unwrap() + 1;
        total_flow_rate += (valve.flow_rate * (minutes - time));

        minutes -= time;
        println!("{} {}", current, valve.id);
        println!("{:?} Travel time: {} Time left: {} name: {}", total_flow_rate, time, minutes, valve.name);

        current = valve.id;
        sort_valves(&mut valves, &edges, current, minutes);
    }
    println!("{:?}", minutes);
    total_flow_rate
}

fn sort_valves(valves: &mut Valves, edges: &Edges, current: usize, total_time_left: usize) {
    valves.sort_by(|left, right| {
        let t_left = dijkstra(&edges, current, left.id).unwrap();
        let t_right = dijkstra(&edges, current, right.id).unwrap();
        let l_edge_count = edges[left.id].len();
        let r_edge_count = edges[right.id].len();

        let l = left.flow_rate * (total_time_left - t_left);
        let r = right.flow_rate * (total_time_left - t_right);
        //println!("L: {} ({}) with R: {} ({})", left.name, l, right.name, r);
        l.cmp(&r).then_with(|| t_right.cmp(&t_left)).then_with(|| r_edge_count.cmp(&l_edge_count))
    });
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 1651)
}

fn part2(file: &'static str) -> usize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1)
}

fn parse(file: &'static str) -> (Valves, Edges) {
    let contents = fs::read_to_string(file).unwrap();
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();

    let mut valves: Valves = vec![];
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();

    for (id, line) in contents.split_terminator("\n").enumerate() {
        let caps = re.captures(line).unwrap();
        let name = &caps[1];
        let flow_rate = caps[2].parse::<usize>().unwrap();
        let kids = &caps[3];

        let valve = Valve {
            id: id,
            name: name.to_string(),
            flow_rate: flow_rate,
        };
        valves.push(valve);

        for kid in kids.split(", ") {
            map
                .entry(id)
                .and_modify(|v| v.push(kid.to_string()))
                .or_insert(vec![kid.to_string()]);
        }
    }

    let mut edges: Edges = vec![vec![]; valves.len()];
    for (id, kids) in map {
        for kid in kids {
            let v = valves.iter().find(|v| v.name == kid).unwrap();
            edges[id].push(Edge(v.id));
        }
    }

    (valves, edges)
}
