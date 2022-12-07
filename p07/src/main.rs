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
    println!("{:?}", col);
    col.values().filter(|&&val| val <= 100_000).sum()
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
                let d: Vec<_> = (&c[1..c.len() -1]).to_vec();

                for (i, p) in d.iter().enumerate(){
                   let component = p
                       .as_os_str()
                       .to_os_string()
                       .into_string()
                       .unwrap();

                   let n = format!("{}{}", component, i);
                   let l = fs::read_to_string(&path)
                       .unwrap()
                       .parse::<u64>()
                       .unwrap();

                   col.entry(n).and_modify(|x| *x += l).or_insert(l);
                }
            }
        }
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 99999);
    assert_eq!(part1("test_input2"), 95437);
}

fn part2(file: &'static str) -> usize {
    //fs::remove_dir_all("tmp").unwrap();
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
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
                let path = total + "/" + t[1];
                fs::create_dir(&path).unwrap();
            },
            _ => {
                let total = current_dir.join("/");
                let path = total + "/" + t[1];
                fs::write(&path, t[0]).unwrap();
            }
        }
    }
}

