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
        println!("\nROCK #{}", i);
        let rock = ROCKS[i % ROCKS.len()];
        let mut coords = to_coords(rock);

        make_space(&mut rock_coords);
        insert_rock(&mut rock_coords, &coords);
        debug_chamber(&rock_coords);

        for j in 0..4 {
            let jet = wind
                .chars()
                .nth(jet_count % wind.len())
                .unwrap();

            push_wind(rock_coords.last_mut().unwrap(), jet);
            fall_rock(&mut rock_coords);
            debug_chamber(&rock_coords);
            jet_count += 1;
        }


        // for debugging purposes
        if i == 2 {
            break;
        }
    }
    0
}

fn make_space(rock_coords: &mut Vec<Coords>) {
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    for coords in rock_coords.iter() {
        for (y, _, _) in coords {
            if *y < min_y { min_y = *y }
            if *y > max_y { max_y = *y }
        }
    }

    if max_y == 0 { return }

    let height = max_y - min_y;
    let offset = if height % 2 == 0 {
        2
    } else {
        3
    };

    for coords in rock_coords {
        for (y, _, _) in coords {
            *y += offset;
        }
    }
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

fn fall_rock(coords: &mut Vec<Coords>) {
    let len = coords.len() - 1;
    let mut v = 0;

    for (ny, nx, nc) in &coords[len] {
        if *nc == '#' && ny + 1 > v {
            v = ny + 1
        }
    }

    let mut can_fall = true;
    'outer: for i in 0..len {
        for (fy, fx, fc) in &coords[i] {
            if *fy == v {
                println!("{} {} {} {}", fc, fy, fx, v);
                can_fall = false;
            }
        }
    }

    println!("{}", can_fall);

    if can_fall {
        let last = coords.last_mut().unwrap();
        for (y, _, _) in last {
            *y += 1
        }
    }
}

fn debug_chamber(coords: &Vec<Coords>) {
    let mut chamber = vec![];
    let height = coords.len() * 5;

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
