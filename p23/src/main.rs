use std::fs;
use std::collections::{VecDeque, HashMap};

const TRANSFORMATIONS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1)
];

#[derive(Debug, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST,
    IDLE
}

#[derive(Debug)]
struct Elf {
    id: usize,
    x: isize,
    y: isize,
    considered_directions: VecDeque<Direction>
}

impl Elf {
    fn no_elfs_in_dirs(&self, elfs: &Vec<Elf>) -> Vec<u8> {
        let mut comb = vec![];

        for (i, (tx, ty)) in TRANSFORMATIONS.iter().enumerate() {
            let elf_tx = self.x + tx;
            let elf_ty = self.y + ty;
            let mut elf_present = false;

            for c_elf in elfs {
                if c_elf.x == self.x && c_elf.y == self.y {
                    continue
                }

                if elf_tx == c_elf.x && elf_ty == c_elf.y {
                    elf_present = true;
                    break;
                }
            }

            let c = if elf_present { 1 } else { 0 };

            comb.push(c);
        }

        comb
    }

    fn to_coords(&self, direction: Direction) -> (isize, isize) {
        match direction {
            Direction::NORTH => (self.x, self.y - 1),
            Direction::SOUTH => (self.x, self.y + 1),
            Direction::WEST  => (self.x - 1, self.y),
            Direction::EAST  => (self.x + 1, self.y),
            _                => (self.x, self.y)
        }
    }

    fn proposed_direction(&mut self, c: &Vec<u8>) -> (isize, isize) {
        while let Some(dir) = self.considered_directions.pop_front() {
            let can_move = match dir {
                Direction::NORTH => c[0] == 0 && c[1] == 0 && c[2] == 0,
                Direction::SOUTH => c[5] == 0 && c[6] == 0 && c[7] == 0,
                Direction::WEST  => c[1] == 0 && c[3] == 0 && c[5] == 0,
                Direction::EAST  => c[2] == 0 && c[4] == 0 && c[7] == 0,
                _ => false
            };

            let copy = dir.clone();
            self.considered_directions.push_back(dir);
            if can_move {
                println!("PROPOSE DADDY: {} {:?}", self.id, copy);
                return self.to_coords(copy)
            }
        }

        (self.x, self.y)

    }
}

type Point = Elf;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> isize {
    let mut elfs = parse(file);
    let empty = vec![0; TRANSFORMATIONS.len()];

    loop {
        println!("\nLOOP CYCLE===========");
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

            let (elf_dx, elf_dy) = elf.proposed_direction(c);
            move_set
                .entry((elf_dx, elf_dy))
                .and_modify(|mut value| value.push(elf.id))
                .or_insert(vec![elf.id]);
        }

        move_set.retain(|_, v| v.len() < 2);

        for elf in &mut elfs {
            println!("ELF STATE {:?}", elf);
            for ((x, y), elf_ids) in &move_set {
                if elf.id == elf_ids[0] {
                    println!("THIS ELF ACTUALLY MOVED {}", elf.id);
                    elf.x = *x;
                    elf.y = *y;
                }
            }
        }

        println!("{:?}", move_set);
    }
    //println!("{:?}", elfs);
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input2"), 1);
    //assert_eq!(part1("test_input"), 1);
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
                    y: y as isize,
                    considered_directions: VecDeque::from([
                        Direction::NORTH,
                        Direction::SOUTH,
                        Direction::WEST,
                        Direction::EAST
                    ])
                }
            );
            elf_id += 1;
        }
    }

    elfs
}
