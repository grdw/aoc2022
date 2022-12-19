use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashSet, HashMap, VecDeque};
use std::{thread, time::Duration};
use regex::Regex;

#[derive(Debug)]
struct Node {
    open: bool,
    flow_rate: usize,
    key: String,
    children: Vec<RNode>
}

type RNode = Rc<RefCell<Node>>;
type Graph = Vec<RNode>;

#[derive(Debug)]
enum Move {
    OpenValve(usize, usize),
    Navigate(usize),
    Idle
}

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let parsed = parse(file);
    let mut graph = Box::new(parsed);
    let start = Move::Navigate(0);
    let mut possible_moves: Vec<Vec<Move>> = vec![];

    println!("{:?}", graph);
    //dfs(
    //    graph,
    //    start,
    //    &mut possible_moves,
    //    0
    //);
    0
}

//fn dfs(
//    graph: Box<Graph>,
//    mx: Move,
//    possible_moves: &mut Vec<Vec<Move>>,
//    time: usize
//) {
//
//    if time >= 30 {
//        return
//    }
//
//    println!("{:?} {}", mx, time);
//    thread::sleep(Duration::from_millis(1000));
//    //let m = possible_moves.get(time);
//
//    match mx {
//        Move::Navigate(gi) => {
//            let node = graph[gi];
//
//            if !node.open && node.flow_rate > 0 {
//              dfs(
//                  graph,
//                  Move::OpenValve(gi, node.flow_rate),
//                  possible_moves,
//                  time + 1
//              )
//            }
//
//            for ci in &node.children {
//                dfs(
//                    graph,
//                    Move::Navigate(*ci),
//                    possible_moves,
//                    time + 1
//                )
//            }
//
//        },
//        Move::OpenValve(gi, _) => {
//            let mut node = graph[gi];
//            node.open = true;
//            //let node = graph_b.get_mut(gi).unwrap();
//            //node.open = true;
//
//            //for ci in &node.children {
//                //dfs(
//                //    graph,
//                //    Move::Navigate(*ci),
//                //    possible_moves,
//                //    time + 1
//                //)
//            //}
//        },
//        _ => println!("Say what?")
//    }
//}

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
    let contents = fs::read_to_string(file).unwrap();
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();

    let mut map = HashMap::new();

    for line in contents.split_terminator("\n") {
        let caps = re.captures(line).unwrap();
        let name = caps[1].to_string();
        let flow_rate = &caps[2].parse::<usize>().unwrap();
        let rc = Rc::new(
            RefCell::new(
                Node {
                    open: false,
                    flow_rate: *flow_rate,
                    key: name.to_string(),
                    children: vec![]
                }
            )
        );

        map.insert(name, rc);
    }

    for line in contents.split_terminator("\n") {
        let caps = re.captures(line).unwrap();
        let name = &caps[1];
        let kids = &caps[3];
        let mut children = vec![];
        let mut current = map.get(name).unwrap().borrow_mut();

        for kid in kids.split(", ") {
            let n = map.get(kid).unwrap();

            children.push(n.clone());
        }

        current.children = children;
    }

    map.values().collect()
}

