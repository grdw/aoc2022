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
            let val = "RLTB"
                .chars()
                .map(|n| calculate_score(&trees, x, y, n))
                .product();

            if val > scenic_score {
                scenic_score = val
            }
        }
    }

    scenic_score
}

fn calculate_score(trees: &Trees, x: usize, y: usize, d: char) -> usize {
    let mut score = 0;

    let bound = match d {
        'L' => x,
        'R' => (trees.len() - x - 1),
        'T' => (trees.len() - y - 1),
        'B' => y,
        _ => panic!("Invalid direction")
    };

    for i in 1..=bound {
        let check = match d {
            'L' => trees[y][x - i],
            'R' => trees[y][x + i],
            'T' => trees[y + i][x],
            'B' => trees[y - i][x],
            _ => panic!("Invalid direction!")
        };

        score += 1;

        if trees[y][x] <= check {
            break;
        }
    }

    score
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
