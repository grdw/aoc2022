use std::fs;
use std::collections::{HashSet, HashMap, VecDeque};
use std::{thread, time::Duration};
use regex::Regex;

const TIME: usize = 30;

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
    let mut total_flow_rates: Vec<u32> = vec![];

    println!("{:?}", graph);
    println!("{:?}", flow_rates);
    println!("{:?}", valve_states);
    dfs(
        &graph,
        &flow_rates,
        &mut valve_states,
        start,
        &mut total_flow_rates,
        0,
        4
    );

    println!("{:?}", total_flow_rates);
    0
}

fn dfs(
    graph: &Routes,
    flow_rates: &FlowRates,
    valve_states: &mut ValveStates,
    mx: Move,
    total_flow_rates: &mut Vec<u32>,
    total_flow_rate: u32,
    time: usize
) {

    println!("{:?} {}", mx, time);
    if time <= 0 {
        if total_flow_rate > 0 {
            total_flow_rates.push(total_flow_rate);
        }
        // Add total flow rate to list
        return
    }

    //thread::sleep(Duration::from_millis(1000));

    match mx {
        Move::Navigate(node_name) => {
            let children = graph.get(&node_name).unwrap();
            let read_valves = valve_states.clone(); // Because, fuck you Rust!
            let valve_state = read_valves.get(&node_name).unwrap();
            let flow_rate = flow_rates.get(&node_name).unwrap();

            let mut t_opens: u32 = 0;
            for (valve, state) in &read_valves {
                if !state { continue };

                let rate = flow_rates.get(valve).unwrap();
                println!("{:?} {}", rate, valve);
                t_opens += rate;
            }

            println!("{:?} {}", total_flow_rate, t_opens);


            for kid in children {
                dfs(
                    graph,
                    flow_rates,
                    valve_states,
                    Move::Navigate(kid.to_string()),
                    total_flow_rates,
                    total_flow_rate,
                    time - 1
                );
            }

            if !valve_state && *flow_rate > 0 {
                dfs(
                    graph,
                    flow_rates,
                    valve_states,
                    Move::OpenValve(node_name),
                    total_flow_rates,
                    total_flow_rate,
                    time - 1
                );
            }
        },
        Move::OpenValve(node_name) => {
            let children = graph.get(&node_name).unwrap();
            let valve_state = valve_states.get_mut(&node_name).unwrap();
            let flow_rate = flow_rates.get(&node_name).unwrap();
            *valve_state = true;
            println!("{}", "OPEN");

            for kid in children {
                dfs(
                    graph,
                    flow_rates,
                    valve_states,
                    Move::Navigate(kid.to_string()),
                    total_flow_rates,
                    total_flow_rate + flow_rate,
                    time - 1
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

