use std::fs;

const DIRECTIONS: &'static str = "RLTB";

type Trees = Vec<Vec<u8>>;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let trees = parse(file);
    let vis = (trees.len() - 1) * 4;
    let sum = score_trees(&trees)
        .iter()
        .filter(|&scores| scores.iter().any(|(a, b)| a == b))
        .count();

    vis + sum
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 21);
}

fn part2(file: &'static str) -> usize {
    let trees = parse(file);

    score_trees(&trees)
        .iter()
        .map(|scores| scores.iter().map(|(s, _)| s ).product())
        .max()
        .unwrap()
}

fn score_trees(trees: &Trees) -> Vec<Vec<(usize, usize)>> {
    let grid_size = trees.len();
    let mut result = vec![];

    for y in 1..grid_size-1 {
        for x in 1..grid_size-1 {
            let inter = DIRECTIONS
                .chars()
                .map(|d| score(&trees, x, y, d))
                .collect();

            result.push(inter);
        }
    }

    result
}

fn score(trees: &Trees, x: usize, y: usize, d: char) -> (usize, usize) {
    let grid_size = trees.len();
    let mut i = 1;

    loop {
        let (check, max) = match d {
            'L' => (trees[y][x - i], x),
            'R' => (trees[y][x + i], grid_size - x - 1),
            'T' => (trees[y + i][x], grid_size - y - 1),
            'B' => (trees[y - i][x], y),
            _ => panic!("Invalid direction!")
        };

        i += 1;

        if i > max || trees[y][x] <= check {
            break (i - 1, max - 1);
        }
    }
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 8);
}

fn parse(file: &'static str) -> Trees {
    let trees_string = fs::read_to_string(file).unwrap();

    trees_string
        .split_terminator("\n")
        .map(|row| row.chars().map(|tree| tree as u8 - 48).collect())
        .collect()
}
