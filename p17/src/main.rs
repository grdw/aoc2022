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
        let mut coords = to_coords(rock);
        insert_rock(&mut rock_coords, &coords);
        debug_chamber(&rock_coords);

        for j in 0..4 {
            let jet = wind
                .chars()
                .nth(jet_count % wind.len())
                .unwrap();

            push_wind(rock_coords.last_mut().unwrap(), jet);

            if j < 3 {
                fall_rock(rock_coords.last_mut().unwrap());
            }
            debug_chamber(&rock_coords);
            jet_count += 1;
        }

        // for debugging purposes
        if i == 1 {
            break;
        }
    }
    0
}

fn to_coords(rock: &str) -> Coords {
    let mut coords = vec![];
    for (y, l) in rock.split("\n").enumerate() {
        for (x, c) in l.chars().enumerate() {
            coords.push((y, x, c));
        }
    }
    coords
}

fn push_wind(coords: &mut Coords, wind: char) {
    if wind == '>' {
        let highest_x = coords
            .iter()
            .map(|(_, x, _)| x)
            .max()
            .unwrap();

        if (highest_x + 1) < CHAMBER_WIDTH {
            for (_, x, _) in coords {
                *x += 1
            }
        }
    } else if wind == '<' {
        let lowest_x = coords
            .iter()
            .map(|(_, x, _)| x)
            .min()
            .unwrap();

        if lowest_x > &0 {
            for (_, x, _) in coords {
                *x -= 1
            }
        }
    }
}

fn fall_rock(coords: &mut Coords) {
    for (y, _, _) in coords {
        *y += 1
    }
}

fn debug_chamber(coords: &Vec<Coords>) {
    let mut chamber = vec![];
    let height = coords.len() * 4;

    for _ in 0..height {
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

fn insert_rock(rock_coords: &mut Vec<Coords>, coords: &Coords) {
    let mut inter = vec![];
    for (y, x, c) in coords {
        inter.push((*y, *x + 2, *c))
    }
    rock_coords.push(inter);
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 3068)
}

fn part2(file: &'static str) -> usize {
    let wind = fs::read_to_string(file).unwrap();
    0
}
