use std::fs;

#[derive(Debug, PartialEq)]
enum NodeType {
    Root,
    List,
    Value(u8)
}

#[derive(Debug, PartialEq)]
struct GNode {
    value: NodeType,
    children: Vec<GNode>
}

impl GNode {
    pub fn root() -> GNode {
        GNode { value: NodeType::Root, children: vec![] }
    }

    pub fn parent() -> GNode {
        GNode { value: NodeType::List, children: vec![] }
    }

    pub fn node(val: u8) -> GNode {
        GNode { value: NodeType::Value(val), children: vec![] }
    }

    pub fn nodec(val: NodeType, children: Vec<GNode>) -> GNode {
        GNode { value: val, children: children }
    }

    pub fn add(&mut self, val: u8) {
        self.children.push(GNode::node(val));
    }

    pub fn first_child_val(&self, backup: u8) -> u8 {
        if let Some(first_child) = self.children.get(0) {
            if let NodeType::Value(value) = first_child.value {
                return value
            }
        }
        backup
    }
}

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let parsed = fs::read_to_string(file).unwrap();
    let mut total = 0;
    for (i, group) in parsed.split_terminator("\n\n").enumerate() {
        let (left, right) = group.split_once("\n").unwrap();
        let l_tree = parse_tree(left);
        let r_tree = parse_tree(right);
        let mut ordered = false;
        let j = i + 1;
        println!("\n 🌿 PAIR: {}", j);
        traverse(&l_tree, &r_tree, &mut ordered);

        if ordered {
            println!("🎉 {}", j);
            total += j
        }
    }
    total
}

fn traverse(l: &GNode, r: &GNode, ordered: &mut bool) {
    let mut i = 0;

    loop {
        match (l.children.get(i), r.children.get(i)) {
            (Some(l_child), Some(r_child)) => {
                println!("\n{:?} --- 🥊 --- {:?}", l_child, r_child);

                match (&l_child.value, &r_child.value) {
                    (NodeType::Value(l_val), NodeType::Value(r_val)) => {
                        if l_val < r_val {
                            *ordered = true
                        }
                    },
                    (NodeType::Value(l_val), NodeType::List) => {
                        if *l_val < r_child.first_child_val(*l_val) {
                            *ordered = true
                        }
                        break;
                    },
                    (NodeType::List, NodeType::Value(r_val)) => {
                        if l_child.first_child_val(*r_val) < *r_val {
                            *ordered = true
                        }
                        break;
                    },
                    (_, _) => ()
                }

                traverse(l_child, r_child, ordered);
            },
            (Some(_l_child), None) => (),
            (None, Some(_r_child)) => *ordered = true,
            (None, None) => break
        }

        i += 1;
    }
}

fn parse_tree(line: &str) -> GNode {
    //println!("\nDEBUGGING: {}", line);
    let mut root: GNode = GNode::root();
    let mut depth = 0;
    let mut s = String::from("");

    for l in line.chars() {
        match l {
            '[' => {
                add_blank(&depth, &mut root);
                depth += 1;
            },
            ']' => {
                add_child(&mut s, &depth, &mut root);
                depth -= 1;
            },
            '0'..='9' => s.push(l),
            ',' => add_child(&mut s, &depth, &mut root),
            '\n' => (),
            _ => panic!("Invalid char {}", l)
        }
    }

    root
}

fn add_child(s: &mut String, depth: &usize, root: &mut GNode) {
    if s.len() < 1 { return }

    let depth = depth - 1;
    let value = s.parse::<u8>().unwrap();
    let mut n = root;

    for _ in 0..depth {
        let len = n.children.len() - 1;
        n = n.children.get_mut(len).unwrap();
    }

    n.add(value);
    *s = String::from("");
}

fn add_blank(depth: &usize, root: &mut GNode) {
    if *depth < 1 { return }

    let depth = depth - 1;
    let mut n = root;

    for _ in 0..depth {
        let len = n.children.len() - 1;
        n = n.children.get_mut(len).unwrap();
    }

    n.children.push(GNode::parent());
}

#[test]
fn test_parse_tree() {
    assert_eq!(
        parse_tree("[1]"),
        GNode::nodec(NodeType::Root, vec![GNode::node(1)])
    );

    assert_eq!(
        parse_tree("[[1],[2]]"),
        GNode::nodec(
            NodeType::Root,
            vec![
                GNode::nodec(NodeType::List, vec![GNode::node(1)]),
                GNode::nodec(NodeType::List, vec![GNode::node(2)])
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,[2]]"),
        GNode::nodec(
            NodeType::Root,
            vec![
                GNode::node(1),
                GNode::nodec(NodeType::List, vec![GNode::node(2)])
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,[2,3]]"),
        GNode::nodec(
            NodeType::Root,
            vec![
                GNode::node(1),
                GNode::nodec(
                    NodeType::List,
                    vec![
                        GNode::node(2),
                        GNode::node(3)
                    ]
                )
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,5,[2,3],6]"),
        GNode::nodec(
            NodeType::Root,
            vec![
                GNode::node(1),
                GNode::node(5),
                GNode::nodec(
                    NodeType::List,
                    vec![
                        GNode::node(2),
                        GNode::node(3)
                    ]
                ),
                GNode::node(6),
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,[2,[5]],3]"),
        GNode::nodec(
            NodeType::Root,
            vec![
                GNode::node(1),
                GNode::nodec(
                    NodeType::List,
                    vec![
                        GNode::node(2),
                        GNode::nodec(
                            NodeType::List,
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

    assert_eq!(
        parse_tree("[]"),
        GNode::nodec(
            NodeType::Root,
            vec![]
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
