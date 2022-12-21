use std::fs;

const MAX: usize = 2022;
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
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let wind = fs::read_to_string(file).unwrap();
    let mut jet_count = 0;
    let mut rock_coords: Vec<Coords> = vec![
        to_coords("#######", 0, 0)
    ];

    for i in 0..10 {
        println!("\n⚡⚡⚡⚡⚡ CYCLE: {} ⚡⚡⚡⚡⚡", i);
        let rock = ROCKS[i % ROCKS.len()];
        let y_offset = highest_y(&rock_coords) + 4;
        let insert_rock_coords = to_coords(rock, y_offset, 2);
        rock_coords.push(insert_rock_coords);

        println!("AFTER INSERT:");
        debug_chamber(&rock_coords);

        loop {
            let jet = wind
                .chars()
                .nth(jet_count % wind.len())
                .unwrap();

            // The wind should push the latest rock to whichever
            // direction
            if can_push_wind_right(&rock_coords, jet) {
                println!("🍇 RIGHT PUSH >");
                push_wind_right(rock_coords.last_mut().unwrap())
            } else if can_push_wind_left(&rock_coords, jet) {
                println!("🍓 LEFT PUSH <");
                push_wind_left(rock_coords.last_mut().unwrap())
            } else {
                println!("🍍 IDLE");
            }

            debug_chamber(&rock_coords);

            let can_fall = can_fall(&rock_coords);
            jet_count += 1;
            // ... and then you should tumble
            // but only if it fits
            if can_fall {
                fall_rock(rock_coords.last_mut().unwrap());
                debug_chamber(&rock_coords);
            }

            if !can_fall {
                break;
            }
        }
    }

    0
}

fn can_fall(coords: &Vec<Coords>) -> bool {
    let l = coords.len() - 1;
    let last_inserted_rock = &coords[l];
    // Is the spot below empty
    let max_y = lowest_y_coords(last_inserted_rock);
    let mut fall = true;
    let mut exes = vec![];

    for (y, x) in last_inserted_rock {
        if *y != max_y { continue }

        exes.push(x);
    }

    for i in 0..l {
        for (y, x) in &coords[i] {
            if *y != max_y - 1 {
                continue
            }

            if exes.contains(&x)  {
                fall = false
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
    let height = highest_y(&coords);

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

fn lowest_y(coords: &Vec<Coords>) -> usize {
    let mut min_y = usize::MAX;
    for coords in coords {
        let y = lowest_y_coords(coords);
        if y < min_y {
            min_y = y
        }
    }
    min_y
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

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 3068)
}

fn part2(file: &'static str) -> usize {
    let wind = fs::read_to_string(file).unwrap();
    0
}
