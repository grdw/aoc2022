use std::fs;

const MAX: usize = 2022;
const CHAMBER_WIDTH: usize = 7;
const ROCKS: [&'static str; 5] = [
    "####",

    ".#.\n\
     ###\n\
     .#.",

    "..#\n\
     ..#\n\
     ###",

     "#\n\
      #\n\
      #\n\
      #",

     "##\n\
      ##"
];

type Coords = Vec<(usize, usize, char)>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let wind = fs::read_to_string(file).unwrap();
    let mut jet_count = 0;
    let mut rock_coords: Vec<Coords> = vec![];

    for i in 0..MAX {
        let rock = ROCKS[i % ROCKS.len()];
        let insert_rock_coords = to_coords(rock);
        move_coords_down(&mut rock_coords, &insert_rock_coords);
        rock_coords.push(insert_rock_coords);

        //println!("AFTER INSERT");
        //debug_chamber(&rock_coords);

        loop {
            let jet = wind
                .chars()
                .nth(jet_count % wind.len())
                .unwrap();

            // The wind should push the latest rock to whichever
            // direction

            if can_push_wind_right(&rock_coords, jet) {
                push_wind_right(rock_coords.last_mut().unwrap())
            } else if can_push_wind_left(&rock_coords, jet) {
                push_wind_left(rock_coords.last_mut().unwrap())
            }

            let can_fall = can_fall(&rock_coords, jet_count);
            jet_count += 1;
            // ... and then you should tumble
            // but only if it fits
            if can_fall {
                fall_rock(rock_coords.last_mut().unwrap());
            }

            //debug_chamber(&rock_coords);

            if !can_fall {
                break;
            }
        }

        //println!("FINAL AFTER ALL MOVES:");
        //debug_chamber(&rock_coords);
    }

    highest_y(&rock_coords)
}

fn can_fall(coords: &Vec<Coords>, count: usize) -> bool {
    let l = coords.len() - 1;

    if l == 0 {
        return count < 3
    }

    let last_inserted_rock = &coords[l];
    // Is the spot below empty
    let max_y = highest_y_coords(last_inserted_rock);
    let mut fall = true;
    let mut exes = vec![];

    for (y, x, _) in last_inserted_rock {
        if *y != max_y { continue }

        exes.push(x);
    }

    for i in 0..l {
        for (y, x, _) in &coords[i] {
            if *y == max_y + 1 {
                if exes.contains(&x)  {
                    fall = false
                }
            }
        }
    }

    fall
}

// Give the next rock 3 spaces
fn move_coords_down(
    rock_coords: &mut Vec<Coords>,
    to_be_inserted_rock: &Coords
) {
    let mut topy = 0;
    let mut space_to_make: isize = 3;
    let low = lowest_y_coords(to_be_inserted_rock);
    let high = highest_y_coords(to_be_inserted_rock);
    let height = (high - low) + 1;

    if rock_coords.len() == 0 {
        // First time
        topy = 1;
    } else {
        topy = lowest_y(rock_coords);
    }

    space_to_make += (height as isize - topy as isize);
    for coords in rock_coords {
        for (y, _, _) in coords {
            if space_to_make < 0 {
                *y -= space_to_make.abs() as usize;
            } else {
                *y += space_to_make as usize;
            }
        }
    }
}

fn to_coords(rock: &str) -> Coords {
    let mut coords = vec![];
    for (y, l) in rock.split("\n").enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '.' { continue };
            coords.push((y, x + 2, c));
        }
    }
    coords
}

fn can_push_wind_right(coords: &Vec<Coords>, jet: char) -> bool {
    if jet == '<' { return false }

    let mut max_x = 0;
    let l = coords.len() - 1;
    let last_inserted_rock = &coords[l];

    for (_, x, _) in last_inserted_rock {
        if *x > max_x {
            max_x = *x
        }
    }

    for i in 0..l {
        for (ly, lx, _) in last_inserted_rock {
            for (y, x, _) in &coords[i] {
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
    for (_, x, _) in coords {
        *x += 1
    }
}

fn can_push_wind_left(coords: &Vec<Coords>, jet: char) -> bool {
    if jet == '>' { return false }

    let mut min_x = usize::MAX;
    let l = coords.len() - 1;
    let last_inserted_rock = &coords[l];

    for (_, x, _) in last_inserted_rock {
        if *x < min_x {
            min_x = *x
        }
    }

    if min_x == 0 {
        return false
    }

    for i in 0..l {
        for (ly, lx, _) in last_inserted_rock {
            for (y, x, _) in &coords[i] {
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
    for (_, x, _) in coords {
        *x -= 1
    }
}

fn fall_rock(coords: &mut Coords) {
    for (y, _, _) in coords {
        *y += 1
    }
}

fn debug_chamber(coords: &Vec<Coords>) {
    let mut chamber = vec![];
    let height = highest_y(&coords);

    for _ in 0..=height {
        let mut chamber_line = vec![];
        for _ in 0..CHAMBER_WIDTH {
            chamber_line.push('.');
        }
        chamber.push(chamber_line);
    }

    for coords in coords {
        for (y, x, c) in coords {
            chamber[*y][*x] = *c
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
    for (y, _, _) in coords {
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
    for (y, _, _) in coords {
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
