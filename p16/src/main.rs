use std::fs;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use regex::Regex;

#[derive(Debug, Clone)]
struct Edge(usize, usize);

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve {
    id: usize,
    name: String,
    flow_rate: usize
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    time: usize,
    flow_rate: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on times.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.time.cmp(&self.time)
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

fn dijkstra(edges: &Edges, start: usize, goal: usize) -> Option<State> {
    let mut dist: Vec<_> = (0..edges.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { time: 0, flow_rate: 0, position: start });

    while let Some(State { time, flow_rate, position }) = heap.pop() {
        if position == goal { return Some( State { time, flow_rate, position }); }
        if time > dist[position] { continue; }

        for edge in &edges[position] {
            let next = State {
                time: time + 1,
                flow_rate: edge.1 * (30 - time - 1),
                position: edge.0
            };

            if next.time < dist[next.position] {
                heap.push(next);

                dist[next.position] = next.time;
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

    sort_valves(&mut valves, &edges, current);
    debug(&valves);

    println!("START AT 'AA");
    while let Some(valve) = valves.pop() {
        println!("GO TO '{}", valve.name.to_string());
        let state = dijkstra(&edges, current, valve.id).unwrap();
        let travel_time = state.time + 1;
        if minutes < travel_time {
            break;
        }
        minutes -= travel_time;
        total_flow_rate += valve.flow_rate * minutes;
        current = valve.id;
        sort_valves(&mut valves, &edges, current);
        debug(&valves);
    }
    total_flow_rate
}

fn debug(valves: &Valves) {
    println!("{:?}", valves.iter().map(|n| &n.name).collect::<Vec<&String>>());
    println!("");
}

fn sort_valves(valves: &mut Valves, edges: &Edges, current: usize) {
    valves.sort_by(|v, ov| {
        let d_v = dijkstra(&edges, current, v.id).unwrap();
        let d_ov = dijkstra(&edges, current, ov.id).unwrap();

        println!("{} {:?} | {} {:?}", v.name, d_v, ov.name, d_ov);
        d_v.flow_rate.cmp(&d_ov.flow_rate).then_with(|| d_ov.time.cmp(&d_v.time))
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
            edges[id].push(Edge(v.id, v.flow_rate));
        }
    }

    (valves, edges)
}
