use std::fs;
use std::collections::{HashSet, HashMap, VecDeque};
use std::{thread, time::Duration};
use regex::Regex;

type Routes = HashMap<String, Vec<String>>;
type FlowRates = HashMap<String, u32>;
type ValveStates = HashMap<String, bool>;

#[derive(Debug)]
enum Move {
    OpenValve(String),
    Navigate(String),
    Idle
}

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let (graph, flow_rates, mut valve_states) = parse(file);
    let start = Move::Navigate("AA".to_string());
    let mut possible_moves: Vec<Vec<Move>> = vec![];

    println!("{:?}", graph);
    println!("{:?}", flow_rates);
    println!("{:?}", valve_states);
    dfs(
        &graph,
        &flow_rates,
        &mut valve_states,
        start,
        &mut possible_moves,
        0
    );
    0
}

fn dfs(
    graph: &Routes,
    flow_rates: &FlowRates,
    valve_states: &mut ValveStates,
    mx: Move,
    possible_moves: &mut Vec<Vec<Move>>,
    time: usize
) {

    if time >= 4 {
        return
    }

    println!("{:?} {}", mx, time);
    //thread::sleep(Duration::from_millis(1000));

    match mx {
        Move::Navigate(node_name) => {
            let children = graph.get(&node_name).unwrap();
            let read_valves = valve_states.clone(); // Because, fuck you Rust!
            let valve_state = read_valves.get(&node_name).unwrap();
            let flow_rate = flow_rates.get(&node_name).unwrap();

            for kid in children {
                dfs(
                    graph,
                    flow_rates,
                    valve_states,
                    Move::Navigate(kid.to_string()),
                    possible_moves,
                    time + 1
                );
            }

            if !valve_state && *flow_rate > 0 {
                dfs(graph, flow_rates, valve_states, Move::OpenValve(node_name), possible_moves, time + 1);
            }
        },
        Move::OpenValve(node_name) => {
            let children = graph.get(&node_name).unwrap();
            let valve_state = valve_states.get_mut(&node_name).unwrap();
            *valve_state = true;

            for kid in children {
                dfs(
                    graph,
                    flow_rates,
                    valve_states,
                    Move::Navigate(kid.to_string()),
                    possible_moves,
                    time + 1
                );
            }

        },
        _ => println!("PAIN!")
    };
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

fn parse(file: &'static str) -> (Routes, FlowRates, ValveStates) {
    let contents = fs::read_to_string(file).unwrap();
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();

    let mut routes: Routes = HashMap::new();
    let mut flow_rates: FlowRates = HashMap::new();
    let mut valve_states: ValveStates = HashMap::new();

    for line in contents.split_terminator("\n") {
        let caps = re.captures(line).unwrap();
        let name = &caps[1];
        let flow_rate = caps[2].parse::<u32>().unwrap();
        let kids = &caps[3];

        routes.insert(name.to_string(), vec![]);
        valve_states.insert(name.to_string(), false);
        flow_rates.insert(name.to_string(), flow_rate);

        for kid in kids.split(", ") {
            routes
                .entry(name.to_string())
                .and_modify(|v| v.push(kid.to_string()));
        }
    }

    (routes, flow_rates, valve_states)
}

