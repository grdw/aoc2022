use std::fs;
use std::collections::HashMap;

struct Node {
    open: bool,
    flow_rate: usize
}

type Graph = HashMap<Node, Node>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let graph: Graph = parse(file);
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

fn parse(file: &'static str) -> Graph {
    let mut graph: Graph = HashMap::new();
    graph
}

