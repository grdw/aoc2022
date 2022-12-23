use std::fs;
use std::collections::{VecDeque, HashMap};

const TRANSFORMATIONS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1)
];

#[derive(Debug)]
struct Elf {
    id: usize,
    x: isize,
    y: isize
}

impl Elf {
    fn no_elfs_in_dirs(&self, elfs: &Vec<Elf>) -> Vec<u8> {
        let mut comb = vec![0; TRANSFORMATIONS.len()];

        for (i, (tx, ty)) in TRANSFORMATIONS.iter().enumerate() {
            let elf_tx = self.x + tx;
            let elf_ty = self.y + ty;
            let mut elf_present = false;

            for c_elf in elfs {
                if elf_tx == c_elf.x && elf_ty == c_elf.y {
                    elf_present = true;
                    break;
                }
            }

            if elf_present {
                comb[i] = 1;
            }
        }

        if comb.len() != TRANSFORMATIONS.len() {
            panic!("Invalid comb");
        }

        comb
    }

    fn to_coords(&self, direction: &char) -> (isize, isize) {
        match direction {
            'N' => (self.x, self.y - 1),
            'S' => (self.x, self.y + 1),
            'W' => (self.x - 1, self.y),
            'E' => (self.x + 1, self.y),
            _   => (self.x, self.y)
        }
    }

    fn proposed_direction(&mut self, c: &Vec<u8>, directions: &VecDeque<char>) -> char {
        for dir in directions {
            let can_move = match dir {
                'N' => c[0] == 0 && c[1] == 0 && c[2] == 0,
                'S' => c[5] == 0 && c[6] == 0 && c[7] == 0,
                'W' => c[0] == 0 && c[3] == 0 && c[5] == 0,
                'E' => c[2] == 0 && c[4] == 0 && c[7] == 0,
                _   => false
            };

            if can_move {
                return *dir
            }
        }

        'I'
    }
}

#[test]
fn test_elf_in_dirs() {
    let elf = Elf { id: 0, x: 0, y: 0 };
    let elfs = vec![
        Elf { id: 0, x: 0, y: 0 }
    ];

    assert_eq!(elf.no_elfs_in_dirs(&elfs), vec![0, 0, 0, 0, 0, 0, 0, 0]);

    let elf = Elf { id: 0, x: 0, y: 0 };
    let elfs = vec![
        Elf { id: 0, x: 1, y: 0 }
    ];

    assert_eq!(elf.no_elfs_in_dirs(&elfs), vec![0, 0, 0,
                                                0,    1,
                                                0, 0, 0]);

    let elf = Elf { id: 0, x: 0, y: 0 };
    let elfs = vec![
        Elf { id: 0, x: 1, y: 0 },
        Elf { id: 0, x: -1, y: 0 },
        Elf { id: 0, x: 1, y: -1 },
        Elf { id: 0, x: -1, y: 1 },
        Elf { id: 0, x: 0, y: 1 },
        Elf { id: 0, x: 0, y: -1 },
        Elf { id: 0, x: -1, y: -1 },
        Elf { id: 0, x: 1, y: 1 }
    ];

    assert_eq!(elf.no_elfs_in_dirs(&elfs), vec![1, 1, 1,
                                                1,    1,
                                                1, 1, 1]);
}

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}


fn part1(file: &'static str) -> usize {
    let mut elfs = parse(file);
    let mut considered_directions = VecDeque::from(['N', 'S', 'W', 'E']);

    let empty = vec![0; TRANSFORMATIONS.len()];
    let mut counter = 0;

    loop {
        let mut move_set: HashMap<(isize, isize), Vec<usize>> = HashMap::new();
        let mut combs: HashMap<usize, Vec<u8>> = HashMap::new();

        for elf in &elfs {
            combs.insert(elf.id, elf.no_elfs_in_dirs(&elfs));
        }

        if combs.values().all(|c| c == &empty) {
            break;
        }

        for elf in &mut elfs {
            let c = &combs[&elf.id];

            if c == &empty {
                continue
            }

            let direction = elf.proposed_direction(c, &considered_directions);
            let (elf_dx, elf_dy) = elf.to_coords(&direction);

            move_set
                .entry((elf_dx, elf_dy))
                .and_modify(|value| value.push(elf.id))
                .or_insert(vec![elf.id]);
        }

        considered_directions.rotate_left(1);

        move_set.retain(|_, v| v.len() < 2);

        for elf in &mut elfs {
            for ((x, y), elf_ids) in &move_set {
                if elf.id == elf_ids[0] {
                    elf.x = *x;
                    elf.y = *y;
                }
            }
        }

        counter += 1;

        if counter == 10 {
            break;
        }
    }

    empty_ground_tiles(&elfs)
}

fn empty_ground_tiles(elfs: &Vec<Elf>) -> usize {
    let mut count = 0;
    let min_x = elfs.iter().map(|e| e.x).min().unwrap();
    let max_x = elfs.iter().map(|e| e.x).max().unwrap();
    let min_y = elfs.iter().map(|e| e.y).min().unwrap();
    let max_y = elfs.iter().map(|e| e.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elfs.iter().find(|e| e.x == x && e.y == y).is_none() {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn test_part1() {
    //assert_eq!(part1("test_input"), 110);
    //assert_eq!(part1("test_input2"), 25);
    assert_eq!(part1("test_input3"), 812);
}

fn part2(file: &'static str) -> isize {
    let points = parse(file);
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(file: &'static str) -> Vec<Elf> {
    let mut elfs = vec![];
    let content = fs::read_to_string(file).unwrap();

    let mut elf_id = 0;
    for (y, line) in content.split_terminator("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' { continue };

            elfs.push(
                Elf {
                    id: elf_id,
                    x: x as isize,
                    y: y as isize
                }
            );
            elf_id += 1;
        }
    }

    elfs
}
