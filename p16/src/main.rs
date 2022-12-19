use std::fs;
use std::collections::{HashMap};
use std::{thread, time::Duration};
use regex::Regex;

type Graph = HashMap<String, Vec<String>>;
type FlowRates = HashMap<String, usize>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let start = "AA".to_string();
    let (graph, flow_rates) = parse(file);
    println!("{:?}", graph);
    println!("===");
    println!("{:?}", flow_rates);
    0
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

fn parse(file: &'static str) -> (Graph, FlowRates) {
    let mut graph: Graph = HashMap::new();
    let mut flow_rates: FlowRates = HashMap::new();
    let contents = fs::read_to_string(file).unwrap();
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();

    for line in contents.split_terminator("\n") {
        let caps = re.captures(line).unwrap();
        let name = &caps[1];
        let flow_rate = &caps[2].parse::<usize>().unwrap();
        let kids = &caps[3];

        flow_rates.insert(name.to_string(), *flow_rate);
        for kid in kids.split(", ") {
            graph
                .entry(name.to_string())
                .and_modify(|mut v| v.push(kid.to_string()))
                .or_insert(vec![kid.to_string()]);
        }
    }

    (graph, flow_rates)
}

