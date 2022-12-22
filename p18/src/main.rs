use std::fs;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let coords = parse(file);
    let mut exposed = 0;

    for i in 0..coords.len() {
        let mut visible_sides = 6;

        let (cx, cy, cz) = coords[i];

        for j in 0..coords.len() {
            if i == j { continue }

            let (nx, ny, nz) = coords[j];

            if ((nx - cx).abs() == 1 && ny == cy && nz == cz) ||
               (nx == cx && (ny - cy).abs() == 1 && nz == cz) ||
               (nx == cx && ny == cy && (nz - cz).abs() == 1) {
                visible_sides -= 1;
            }
        }

        exposed += visible_sides;
    }

    exposed
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 64);
}

fn part2(file: &'static str) -> usize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(file: &'static str) -> Vec<(isize, isize, isize)> {
    let contents = fs::read_to_string(file).unwrap();
    contents.split_terminator("\n").map(|line| {
        let nums: Vec<isize> = line
            .split(",")
            .map(|n| n.parse::<isize>().unwrap())
            .collect();

        (nums[0], nums[1], nums[2])
    }).collect()
}
