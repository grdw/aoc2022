use std::fs;
use std::path::{Path};
use std::collections::HashMap;

type Sizes = HashMap<String, u64>;

fn main() {
    println!("Part 1: {}", part1("input"));
    //println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> u64 {
    parse_structure(file);
    let mut col: Sizes = HashMap::new();
    recurse_check(&Path::new("tmp"), &mut col);
    fs::remove_dir_all("tmp").unwrap();
    col.into_iter().map(|(_, val)| {
        if val < 100000 {
            val
        } else {
            0
        }
    }).sum()
}

fn recurse_check(path: &Path, col: &mut Sizes) {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                recurse_check(&path, col);
            } else {
                let c: Vec<_> = path.components().collect();

                for i in 1..c.len()-1 {
                    let component = c[i]
                        .as_os_str()
                        .to_os_string()
                        .into_string()
                        .unwrap();

                    let l = fs::metadata(&path).unwrap().len();
                    col.entry(component).and_modify(|x| *x += l).or_insert(l);
                }
            }
        }
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 95437);
}

fn part2(file: &'static str) -> usize {
    fs::remove_dir_all("tmp").unwrap();
    0
}

#[test]
fn test_part2() {
    assert_eq!(part1("test_input"), 1);
}

fn parse_structure(file: &'static str) {
    let commands = fs::read_to_string(file).unwrap();
    let mut current_dir = vec!["tmp"];
    fs::create_dir(current_dir.join("/")).unwrap();

    for c in commands.split_terminator("\n") {
        let t: Vec<&str> = c.split(" ").collect();

        match t[0] {
            "$" => {
                match t[1] {
                    "cd" => {
                        if t[2] == ".." {
                            current_dir.pop();
                        } else if t[2] != "/" {
                            current_dir.push(t[2]);
                        }
                    },
                    "ls" => (),
                    _ => panic!("booooom!")
                }
            }
            "dir" => {
                let total = current_dir.join("/");
                fs::create_dir(total + "/" + t[1]).unwrap()
            },
            _ => {
                let total = current_dir.join("/");
                let size = t[0].parse::<usize>().unwrap();
                let path = total + "/" + t[1];
                fs::write(&path, vec![0; size]).unwrap();
            }
        }
    }
}

