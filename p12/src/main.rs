use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type HeightMap = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Node(usize, usize);

#[derive(Debug, Clone)]
struct Edge(usize, usize);
type Edges = Vec<Vec<Edge>>;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(input: &'static str) -> usize {
    let height_map = parse(input);
    let width = height_map[0].len();
    let height = height_map.len();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..height {
        for x in 0..width {
            if height_map[y][x] == 'S' {
                start = (y, x);
            }

            if height_map[y][x] == 'E' {
                end = (y, x);
            }
        }
    }

    // Next up get out all the possible "edges", as in, from where
    // to where can I go. All the weights are '1' of all the nodes
    // then just do a little Dijkstra and you should be good.
    // See https://github.com/grdw/aoc2021/blob/main/problem_0015/src/main.rs

    println!("{:?} {:?}", start, end);
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 31);
}

fn part2(input: &'static str) -> u64 {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(input: &'static str) -> HeightMap {
	let file = fs::read_to_string(input).unwrap();

    file.split_terminator("\n").map(|line| {
        line.chars().collect()
    }).collect()
}
