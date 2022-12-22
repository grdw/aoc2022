use std::fs;

type Coords = Vec<(isize, isize, isize)>;
type FCoords<'a> = Vec<&'a(isize, isize, isize)>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let coords = parse(file);
    visible_sides(&coords)
}

fn visible_sides(coords: &Coords) -> usize {
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
    let mut coords = parse(file);
    coords.sort();
    let mut visible_sides = visible_sides(&coords);
    let (mut min_x, mut max_x) = (isize::MAX, 0);
    let (mut min_y, mut max_y) = (isize::MAX, 0);
    let (mut min_z, mut max_z) = (isize::MAX, 0);

    for i in 0..coords.len() {
        let (cx, cy, cz) = coords[i];

        if cx < min_x { min_x = cx };
        if cx > max_x { max_x = cx };
        if cy < min_y { min_y = cy };
        if cy > max_y { max_y = cy };
        if cz < min_z { min_z = cz };
        if cz > max_z { max_z = cz };
    }

    // Open https://www.math3d.org/
    // Then to look at all the visible cubes from all the sides is:
    //
    // (Assume LE, RI, TO, BO, FR, BA)
    // The 6 sides:
    // (x, z) (LE + RI)
    // (y, z) (FR + BA
    // (x, y) (TO + BO)

    let mut x_air_cubes: Coords = vec![];
    let mut y_air_cubes: Coords = vec![];
    let mut z_air_cubes: Coords = vec![];

    for lx in min_x..=max_x {
        for lz in min_z..=max_z {
            let f: FCoords = coords
                .iter()
                .filter(|(x, _, z)| *x == lx && *z == lz)
                .collect();

            if f.len() > 1 {
                let mut prev_y = f[0].1 - 1;
                for (_, fy, _) in &f {
                    if fy - prev_y > 1 {
                        for y in (prev_y+1..*fy) {
                            y_air_cubes.push((lx, y, lz));
                        }
                    }

                    prev_y = *fy;
                }
            }
        }
    }

    for lx in min_x..=max_x {
        for ly in min_y..=max_y {
            let f: FCoords = coords
                .iter()
                .filter(|(x, y, _)| *x == lx && *y == ly)
                .collect();

            if f.len() > 1 {
                let mut prev_z = f[0].1 - 1;
                for (_, _, fz) in &f {
                    if fz - prev_z > 1 {
                        for z in (prev_z+1..*fz) {
                            z_air_cubes.push((lx, ly, z));
                        }
                    }

                    prev_z = *fz;
                }
            }
        }
    }

    for ly in min_y..=max_y {
        for lz in min_z..=max_z {
            let f: FCoords = coords
                .iter()
                .filter(|(_, y, z)| *y == ly && *z == lz)
                .collect();

            if f.len() > 1 {
                let mut prev_x = f[0].1 - 1;
                for (fx, _, _) in &f {
                    if fx - prev_x > 1 {
                        for x in (prev_x+1..*fx) {
                            x_air_cubes.push((x, ly, lz));
                        }
                    }

                    prev_x = *fx;
                }
            }
        }
    }

    println!("{:?}", x_air_cubes.len());
    println!("{:?}", y_air_cubes.len());
    println!("{:?}", z_air_cubes.len());
    println!("==========");
    x_air_cubes
        .retain(|(x, y, z)| {
            y_air_cubes.contains(&(*x, *y, *z)) &&
            z_air_cubes.contains(&(*x, *y, *z))
        });

    //println!("{:?}", x_air_cubes);
    let remove = x_air_cubes.len() * 6;
    println!("{}", visible_sides);
    println!("{:?}", x_air_cubes.len());
    println!("{}", remove);

    visible_sides - remove
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 58);
}

fn parse(file: &'static str) -> Coords {
    let contents = fs::read_to_string(file).unwrap();
    contents.split_terminator("\n").map(|line| {
        let nums: Vec<isize> = line
            .split(",")
            .map(|n| n.parse::<isize>().unwrap())
            .collect();

        (nums[0], nums[1], nums[2])
    }).collect()
}
