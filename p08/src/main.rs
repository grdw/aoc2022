use std::fs;

type Trees = Vec<Vec<u8>>;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let trees = parse(file);
    let grid_size = trees.len();
    let mut vis = (grid_size - 1) * 4;

    for y in 1..grid_size-1 {
        for x in 1..grid_size-1 {
            let around = vec![
                trees[y][x + 1..grid_size].to_vec(),
                trees[y][0..x].to_vec(),
                trees[0..y].iter().map(|m| m[x]).collect::<Vec<u8>>(),
                trees[y + 1..grid_size].iter().map(|m| m[x]).collect::<Vec<u8>>()
            ];

            if around.iter().any(|n| n.iter().all(|&m| m < trees[y][x])) {
                vis += 1
            }
        }
    }

    vis
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 21);
}

fn part2(file: &'static str) -> usize {
    let _trees = parse(file);
    0
}

fn parse(file: &'static str) -> Trees {
    let trees_string = fs::read_to_string(file).unwrap();
    let mut trees: Trees = vec![];
    for row in trees_string.split_terminator("\n") {
        let mut row_vec = vec![];
        for tree in row.chars() {
            row_vec.push(tree as u8 - 48);
        }
        trees.push(row_vec);
    }
    trees
}
