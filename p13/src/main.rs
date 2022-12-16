use std::fs;
use id_tree::*;
use id_tree::InsertBehavior::*;

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part1("input"));
}

fn part1(file: &'static str) -> usize {
    let parsed = fs::read_to_string(file).unwrap();
    for group in parsed.split_terminator("\n\n") {
        println!("{}", "----");
        for line in group.split("\n") {
            let tree = parse_tree(line);
        }
    }

    0
}

fn parse_tree(line: &str) -> Tree<u8> {
    let mut root: Tree<u8> = Tree::new();
    let mut depth = 0;
    let mut s = String::from("");
    let mut id = root.insert(Node::new(0), AsRoot).unwrap();

    for l in line.chars() {
        //println!("{}", root.to_string());
        match l {
            '[' => depth += 1,
            ']' => {
                if s.len() > 1 {
                    add_child(&mut s, &id, &mut root);
                }

                depth -= 1;
            },
            '0'..='9' => s.push(l),
            ',' => id = add_child(&mut s, &id, &mut root),
            _ => panic!("Invalid char")
        }

        println!("-----> {}", s);
    }

    println!("\n\n\n\n\n");
    let mut ssss = String::new();
    root.write_formatted(&mut ssss).unwrap();
    println!("{}", ssss);

    root
}

fn add_child(s: &mut String, id: &NodeId, root: &mut Tree<u8>) -> NodeId {
    println!("{:?}, {:?}", s, id);
    let value = s.parse::<u8>().unwrap();
    let child = Node::new(value);
    *s = String::from("");
    root.insert(child, UnderNode(id)).unwrap()
}

#[test]
fn test_parse_tree() {
    parse_tree("[[[[[[]]]]]]");
    //parse_tree("[1,[2,[3,[4,[5,6,7]]]],8,9]");
    assert_eq!(true, false);
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 13);
}

fn part2(file: &'static str) -> usize {
    0
}
