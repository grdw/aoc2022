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
    let mut bottom = 3;

    for i in 0..MAX {
        let rock = ROCKS[i % ROCKS.len()];
        move_coords_down(&mut rock_coords);
        rock_coords.push(to_coords(rock));

        if i > 0 {
            bottom = highest_y(&rock_coords) - 1;
        }
        println!("======= {}", bottom);
        debug_chamber(&rock_coords);

        // BUT WHY 4 DO?
        for i in 0..4 {
            let jet = wind
                .chars()
                .nth(jet_count % wind.len())
                .unwrap();

            // The wind should push the latest rock to whichever
            // direction
            push_wind(rock_coords.last_mut().unwrap(), jet);
            jet_count += 1;
            // ... and then you should tumble
            fall_rock(&mut rock_coords, bottom);
            debug_chamber(&rock_coords);
        }

        // for debugging purposes
        if i == 3 {
            break;
        }
    }
    0
}

// Give the next rock 3 spaces
fn move_coords_down(rock_coords: &mut Vec<Coords>) {
    for coords in rock_coords {
        for (y, _, _) in coords {
            *y += 3
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

fn push_wind(coords: &mut Coords, wind: char) {
    let (min_x, max_x) = minmax_x(coords);

    if wind == '>' && (max_x + 1) < CHAMBER_WIDTH {
        for (_, x, _) in coords {
            *x += 1
        }
    } else if wind == '<' && min_x > 0 {
        for (_, x, _) in coords {
            *x -= 1
        }
    }
}

fn minmax_x(coords: &Coords) -> (usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = 0;

    for (_, x, _) in coords {
        if *x > max_x {
            max_x = *x
        }

        if *x < min_x {
            min_x = *x
        }
    }

    (min_x, max_x)
}

fn fall_rock(coords: &mut Vec<Coords>, bottom: usize) {
    let last = coords.last_mut().unwrap();
    let p = highest_y_coords(last);
    if p >= bottom {
        return
    }

    for (y, _, _) in last {
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

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 3068)
}

fn part2(file: &'static str) -> usize {
    let wind = fs::read_to_string(file).unwrap();
    0
}
