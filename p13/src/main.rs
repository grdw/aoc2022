use std::fs;

#[derive(Debug, PartialEq)]
struct GNode {
    value: Option<u8>,
    children: Vec<GNode>
}

impl GNode {
    pub fn root() -> GNode {
        GNode { value: None, children: vec![] }
    }

    pub fn node(val: u8) -> GNode {
        GNode { value: Some(val), children: vec![] }
    }

    pub fn nodec(val: Option<u8>, children: Vec<GNode>) -> GNode {
        GNode { value: val, children: children }
    }

    pub fn add(&mut self, val: u8) {
        self.children.push(GNode::node(val));
    }
}

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part1("input"));
}

fn part1(file: &'static str) -> usize {
    let parsed = fs::read_to_string(file).unwrap();
    for group in parsed.split_terminator("\n\n") {
        println!("{}", "===");
        for line in group.split("\n") {
            let tree = parse_tree(line);
        }
    }

    0
}

fn parse_tree(line: &str) -> GNode {
    println!("\nDEBUGGING: {}", line);
    let mut root: GNode = GNode::root();
    let mut depth = 0;
    let mut s = String::from("");

    for l in line.chars() {
        match l {
            '[' => depth += 1,
            ']' => {
                add_child(&mut s, &depth, &mut root);
                depth -= 1;
            },
            '0'..='9' => s.push(l),
            ',' => {
                add_child(&mut s, &depth, &mut root)
            },
            _ => panic!("Invalid char")
        }
    }

    root
}

fn add_child(s: &mut String, depth: &usize, root: &mut GNode) {
    if s.len() < 1 { return }

    let depth = depth - 1;
    let value = s.parse::<u8>().unwrap();

    let mut n = root;
    for i in 0..depth {
        n = n.children.get_mut(0).unwrap();
    }

    n.add(value);
    *s = String::from("");
}

#[test]
fn test_parse_tree() {
    assert_eq!(
        parse_tree("[1]"),
        GNode::nodec(None, vec![GNode::node(1)])
    );

    assert_eq!(
        parse_tree("[1,[2]]"),
        GNode::nodec(
            None,
            vec![
                GNode::nodec(Some(1), vec![GNode::node(2)])
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,[2,3]]"),
        GNode::nodec(
            None,
            vec![
                GNode::nodec(
                    Some(1),
                    vec![
                        GNode::node(2),
                        GNode::node(3)
                    ]
                )
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,[2,[5]],3]"),
        GNode::nodec(
            None,
            vec![
                GNode::nodec(
                    Some(1),
                    vec![
                        GNode::nodec(
                            Some(2),
                            vec![
                                GNode::node(5)
                            ]
                        )
                    ]
                ),
                GNode::node(3)
            ]
        )
    );
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 13);
}

fn part2(file: &'static str) -> usize {
    0
}
