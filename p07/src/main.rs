use std::fs;
use std::collections::HashMap;

type Sizes = HashMap<String, usize>;

fn main() {
    println!("Part 1: {}", part1("input"));
    //println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let sizes = parse_structure(file);
    let mut total_sizes: Sizes = HashMap::new();

    for (key, val) in sizes {
        let subkeys: Vec<&str> = key.split("/").collect();
        for depth in 0..subkeys.len()-1 {
            let sub_key = String::from(subkeys[depth].repeat(depth + 1));

            total_sizes.entry(sub_key).and_modify(|n| *n += val).or_insert(val);
        }
    }
    total_sizes.values().filter(|&&val| val <= 100_000).sum()
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

fn parse_structure(file: &'static str) -> Sizes {
    let commands = fs::read_to_string(file).unwrap();
    let mut current_dir = vec![];
    let mut col: Sizes = HashMap::new();

    for c in commands.split_terminator("\n") {
        let t: Vec<&str> = c.split(" ").collect();

        if t[0] == "$" {
            if t[1] == "cd" {
                if t[2] == ".." {
                    current_dir.pop();
                } else if t[2] != "/" {
                    current_dir.push(t[2]);
                }
            }
        } else {
            let total = current_dir.join("/");
            let path = total + "/" + t[1];

            if t[0] != "dir" {
                let value = t[0].parse::<usize>().unwrap();
                let key = String::from(&path);
                col.entry(key).and_modify(|n| *n += value).or_insert(value);
            }
        }
    }

    col
}
