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
    let trees = parse(file);
    let grid_size = trees.len();
    let mut scenic_score = 0;

    for y in 1..grid_size-1 {
        for x in 1..grid_size-1 {
            let mut left_score = 0;
            let mut right_score = 0;
            let mut top_score = 0;
            let mut bottom_score = 0;

            for r in (x + 1)..grid_size {
                let right = trees[y][r];
                right_score += 1;

                if trees[y][x] <= right {
                    break;
                }
            }

            for l in (0..x).rev() {
                let left = trees[y][l];
                left_score += 1;

                if trees[y][x] <= left {
                    break;
                }
            }

            for b in (0..y).rev() {
                let bottom = trees[b][x];
                bottom_score += 1;

                if trees[y][x] <= bottom {
                    break;
                }
            }

            for t in (y + 1)..grid_size {
                let top = trees[t][x];
                top_score += 1;

                if trees[y][x] <= top {
                    break;
                }
            }

            let val = left_score * right_score * top_score * bottom_score;

            if val > scenic_score {
                scenic_score = val
            }
        }
    }

    scenic_score
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
