use std::fs;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashSet};

type Coords = Vec<(isize, isize, isize)>;
type HCoords = HashSet<(isize, isize, isize)>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let coords = parse(file);
    visible_sides(&coords)
}

fn visible_sides(coords: &HCoords) -> usize {
    let mut exposed = 0;

    for (cx, cy, cz) in coords.iter() {
        let mut visible_sides = 6;

        for (nx, ny, nz) in coords.iter() {
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
    let (
        mut min_x,
        mut max_x,
        mut min_y,
        mut max_y,
        mut min_z,
        mut max_z
    ) = (
        isize::MAX,
        isize::MIN,
        isize::MAX,
        isize::MIN,
        isize::MAX,
        isize::MIN
    );

    let coords = parse(file);
    let points = coords.into_iter().inspect(|&(x, y, z)| {
            min_x = min(min_x, x);
            max_x = max(max_x, x);
            min_y = min(min_y, y);
            max_y = max(max_y, y);
            min_z = min(min_z, z);
            max_z = max(max_z, z);
        })
        .collect::<BTreeSet<_>>();
    let mut queue = vec![(min_x - 1, min_y - 1, min_z - 1)];
    let mut outside = queue.iter().copied().collect::<BTreeSet<_>>();

    while let Some(point) = queue.pop() {
        for (x, y, z) in neighbors(point) {
            if (min_x - 1..=max_x + 1).contains(&x)
                && (min_y - 1..=max_y + 1).contains(&y)
                && (min_z - 1..=max_z + 1).contains(&z)
                && !points.contains(&(x, y, z))
                && outside.insert((x, y, z))
            {
                queue.push((x, y, z));
            }
        }
    }
    points
        .iter()
        .copied()
        .flat_map(neighbors)
        .filter(|point| outside.contains(point))
        .count()
}

fn neighbors((x, y, z): (isize, isize, isize)) -> Coords {
    vec![
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}


#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 58);
}

fn parse(file: &'static str) -> HCoords {
    let contents = fs::read_to_string(file).unwrap();
    let mut set = HashSet::new();
    for line in contents.split_terminator("\n") {
        let nums: Vec<isize> = line
            .split(",")
            .map(|n| n.parse::<isize>().unwrap())
            .collect();

        set.insert((nums[0], nums[1], nums[2]));
    }
    set
}
