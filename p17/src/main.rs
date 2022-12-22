use std::fs;

const CHAMBER_WIDTH: usize = 7;
const ROCKS: [&'static str; 5] = [
    "####",

    ".#.\n\
     ###\n\
     .#.",

    "###\n\
     ..#\n\
     ..#",

     "#\n\
      #\n\
      #\n\
      #",

     "##\n\
      ##"
];

type Coords = Vec<(usize, usize)>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P1: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    tetris_simulator(file, 2022)
}

fn tetris_simulator(file: &'static str, max: usize) -> usize {
    let wind = parse_wind(file);
    let mut jet_count = 0;
    let mut rock_coords: Vec<Coords> = vec![
        to_coords("#######", 0, 0)
    ];

    for i in 0..max {
        tetris_drop(i, &wind, &mut jet_count, &mut rock_coords);
        delete_coords(&mut rock_coords);
    }

    highest_y(&rock_coords)
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 3068);
}

fn part2(file: &'static str) -> usize {
    let rock_index = 1_000_000_000_000;
    let (block_index, tower_height, highest_map) = tetris_height_diff(file, 15_000);
    println!("{:?} {:?}", block_index, tower_height);
    let (next_block_index, next_tower_height) = tetris_next_height_diff(file, highest_map, block_index);
    println!("{:?} {:?}", next_block_index, next_tower_height);
    let diff_index = next_block_index - block_index;
    let diff_height = next_tower_height - tower_height;
    let l = rock_index - block_index; // height = tower_height

    // Start from the start height
    let mut total_height = tower_height;
    let (div, m) = ((l / diff_index), (l % diff_index));
    total_height += div * diff_height;
    // divide whatever you have leftover by the diff_index
    let rest = tetris_simulator(file, block_index + m) - tower_height;
    total_height + rest
}

#[test]
fn test_part2() {
    //assert_eq!(part2("test_input"), 1514285714288);
    assert_eq!(part2("input") / 100_000_000_000, 15);
}

fn tetris_height_diff(file: &'static str, max: usize) -> (usize, usize, Vec<usize>) {
    let wind = parse_wind(file);

    let mut jet_count = 0;
    let mut rock_coords: Vec<Coords> = vec![
        to_coords("#######", 0, 0)
    ];

    let mut tower_height = 0;
    let mut max_height_index = 0;
    let mut max_height_diff_map = vec![];
    let mut patterns = vec![];

    for i in 0..max {
        tetris_drop(i, &wind, &mut jet_count, &mut rock_coords);

        let height_map = delete_coords(&mut rock_coords);
        let max = height_difference(&height_map);
        patterns.push(max.clone());

        for j in 0..patterns.len() - 1 {
            if patterns[j] == max {
                max_height_index = i;
                max_height_diff_map = max;
                tower_height = highest_y(&rock_coords);
                break
            }
        }
    }

    (max_height_index, tower_height, max_height_diff_map)
}

fn tetris_next_height_diff(file: &'static str, map: Vec<usize>, skip: usize) -> (usize, usize) {
    let wind = parse_wind(file);

    let mut jet_count = 0;
    let mut rock_coords: Vec<Coords> = vec![
        to_coords("#######", 0, 0)
    ];

    let mut i = 0;
    let mut max_height = 0;
    let mut next_height_index = 0;

    loop {
        tetris_drop(i, &wind, &mut jet_count, &mut rock_coords);

        let height_map = delete_coords(&mut rock_coords);
        let max = height_difference(&height_map);

        if i > skip && max == map {
            next_height_index = i;
            max_height = highest_y(&rock_coords);
            break;
        }

        i += 1;
    }

    (next_height_index, max_height)
}

#[test]
fn test_tetris_height_diff_map() {
    assert_eq!(
        tetris_height_diff("test_input", 200),
        (199, 308, vec![28, 0, 0, 6, 3, 3, 1])
    );
    assert_eq!(
        tetris_height_diff("input", 100),
        (92, 146, vec![2, 2, 0, 3, 1, 0, 1])
    );
}

fn height_difference(height_map: &Vec<usize>) -> Vec<usize> {
    let max = height_map.iter().max().unwrap();

    height_map
        .iter()
        .map(|n| max - n)
        .collect::<Vec<usize>>()
}

fn tetris_drop(i: usize, wind: &String, jet_count: &mut usize, rock_coords: &mut Vec<Coords>) {
    let rock = ROCKS[i % ROCKS.len()];
    let y_offset = highest_y(&rock_coords) + 4;
    let insert_rock_coords = to_coords(rock, y_offset, 2);
    rock_coords.push(insert_rock_coords);

    loop {
        let jet = wind
            .chars()
            .nth(*jet_count % wind.len())
            .unwrap();

        *jet_count += 1;

        // The wind should push the latest rock to whichever
        // direction
        if can_push_wind_right(&rock_coords, jet) {
            push_wind_right(rock_coords.last_mut().unwrap());
        } else if can_push_wind_left(&rock_coords, jet) {
            push_wind_left(rock_coords.last_mut().unwrap());
        }

        let can_fall = can_fall(&rock_coords);
        // ... and then you should tumble
        // but only if it fits
        if can_fall {
            fall_rock(rock_coords.last_mut().unwrap());
        }

        if !can_fall {
            break;
        }
    }
}

fn can_fall(coords: &Vec<Coords>) -> bool {
    let l = coords.len() - 1;
    let last_inserted_rock = &coords[l];
    // Is the spot below empty
    let mut fall = true;

    'outer: for i in 0..l {
        for (y, x) in &coords[i] {
            for (ly, lx) in last_inserted_rock {
                let cant_go_down = lx == x && *ly == (y + 1);
                if cant_go_down {
                    fall = false;
                    break 'outer;
                }
            }
        }
    }

    fall
}

fn to_coords(rock: &str, offset_y: usize, offset_x: usize) -> Coords {
    let mut coords = vec![];
    for (y, l) in rock.split("\n").enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '.' { continue };
            coords.push(
                (
                    y + offset_y,
                    x + offset_x
                )
            );
        }
    }
    coords
}

fn can_push_wind_right(coords: &Vec<Coords>, jet: char) -> bool {
    if jet == '<' { return false }

    let mut max_x = 0;
    let l = coords.len() - 1;
    let last_inserted_rock = &coords[l];

    for (_, x) in last_inserted_rock {
        if *x > max_x {
            max_x = *x
        }
    }

    for i in 0..l {
        for (ly, lx) in last_inserted_rock {
            for (y, x) in &coords[i] {
                if ly != y { continue }

                if lx + 1 == *x {
                    return false
                }
            }
        }
    }

    max_x + 1 < CHAMBER_WIDTH
}

fn push_wind_right(coords: &mut Coords) {
    for (_, x) in coords {
        *x += 1
    }
}

fn can_push_wind_left(coords: &Vec<Coords>, jet: char) -> bool {
    if jet == '>' { return false }

    let mut min_x = usize::MAX;
    let l = coords.len() - 1;
    let last_inserted_rock = &coords[l];

    for (_, x) in last_inserted_rock {
        if *x < min_x {
            min_x = *x
        }
    }

    if min_x == 0 {
        return false
    }

    for i in 0..l {
        for (ly, lx) in last_inserted_rock {
            for (y, x) in &coords[i] {
                if ly != y { continue }

                if lx - 1 == *x {
                    return false
                }
            }
        }
    }

    min_x > 0
}

fn push_wind_left(coords: &mut Coords) {
    for (_, x) in coords {
        *x -= 1
    }
}

fn fall_rock(coords: &mut Coords) {
    for (y, _) in coords {
        *y -= 1
    }
}

fn debug_chamber(coords: &Vec<Coords>) {
    let mut chamber = vec![];
    let low_height = lowest_y(&coords);
    let height = highest_y(&coords) + 1;
    let h = (height - low_height);

    println!("{:?}", h);
    for _ in 0..=height {
        let mut chamber_line = vec![];
        for _ in 0..CHAMBER_WIDTH {
            chamber_line.push('⬜');
        }
        chamber.push(chamber_line);
    }

    for coords in coords {
        for (y, x) in coords {
            chamber[height - *y][*x] = '⬛';
        }
    }

    println!("");
    for l in chamber {
        println!("{}", l.into_iter().collect::<String>());
    }
}

fn lowest_y(coords: &Vec<Coords>) -> usize {
    let mut min_y = usize::MAX;
    for coords in coords {
        let y = highest_y_coords(coords);
        if y < min_y {
            min_y = y
        }
    }
    min_y
}

fn highest_y(coords: &Vec<Coords>) -> usize {
    let mut max_y = 0;
    for coords in coords {
        let y = highest_y_coords(coords);
        if y > max_y {
            max_y = y
        }
    }
    max_y
}

fn highest_y_coords(coords: &Coords) -> usize {
    let mut max_y = 0;
    for (y, _) in coords {
        if *y > max_y {
            max_y = *y
        }
    }
    max_y
}

fn lowest_y_coords(coords: &Coords) -> usize {
    let mut min_y = usize::MAX;
    for (y, _) in coords {
        if *y < min_y {
            min_y = *y
        }
    }
    min_y
}

fn delete_coords(coords: &mut Vec<Coords>) -> Vec<usize> {
    let mut comb = vec![0;7];
    let coords_len = coords.len();

    for i in 0..coords_len  {
        let rock_shape = &coords[i];
        for (y, x) in rock_shape {
            if *y > comb[*x] {
                comb[*x] = *y;
            }
        }
    }

    for j in 0..coords_len {
        let rock_shape = &mut coords[j];
        for i in (0..rock_shape.len()).rev() {
            let (y, x) = &rock_shape[i];

            if comb[*x] > (*y + 25) {
                rock_shape.remove(i);
            }
        }
    }

    coords.retain(|rock_shape| rock_shape.len() > 0);
    comb
}

fn parse_wind(file: &'static str) -> String {
    let mut wind = fs::read_to_string(file).unwrap();
    wind.trim().to_string()
}
