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

    pub fn comparible(&self) -> bool {
        self.children.len() == 0 && self.value.is_some()
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
        println!("\nPAIR: {}", i + 1);
        traverse(&l_tree, &r_tree, &mut ordered);

        if ordered {
            println!("🎉 {}", i);
            total += (i + 1)
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

                if l_child.comparible() && r_child.comparible() {
                    let l_val = l_child.value.unwrap();
                    let r_val = r_child.value.unwrap();
                    if l_val < r_val {
                        *ordered = true
                    }
                } else if l_child.comparible() {
                    let l_val = l_child.value.unwrap();
                    if let Some(n) = r_child.children.get(0) {
                        let r_val = n.value.unwrap_or(l_val);
                        if l_val < r_val {
                            *ordered = true
                        }
                    }
                    break;
                } else if r_child.comparible() {
                    let r_val = r_child.value.unwrap();
                    if let Some(n) = l_child.children.get(0) {
                        let l_val = n.value.unwrap_or(r_val);
                        if l_val < r_val {
                            *ordered = true
                        }
                    }
                    break;
                } else {

                }

                traverse(l_child, r_child, ordered)
            },
            (Some(l_child), None) => {
                println!("{}", "Only a left child");
            },
            (None, Some(r_child)) => {
                if r_child.children.len() == 0 {
                    *ordered = true;
                }
                println!("{}", "Only a right child");
            },
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

    n.children.push(GNode::root());
}

#[test]
fn test_parse_tree() {
    assert_eq!(
        parse_tree("[1]"),
        GNode::nodec(None, vec![GNode::node(1)])
    );

    assert_eq!(
        parse_tree("[[1],[2]]"),
        GNode::nodec(
            None,
            vec![
                GNode::nodec(None, vec![GNode::node(1)]),
                GNode::nodec(None, vec![GNode::node(2)])
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,[2]]"),
        GNode::nodec(
            None,
            vec![
                GNode::node(1),
                GNode::nodec(None, vec![GNode::node(2)])
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,[2,3]]"),
        GNode::nodec(
            None,
            vec![
                GNode::node(1),
                GNode::nodec(
                    None,
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
            None,
            vec![
                GNode::node(1),
                GNode::node(5),
                GNode::nodec(
                    None,
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
            None,
            vec![
                GNode::node(1),
                GNode::nodec(
                    None,
                    vec![
                        GNode::node(2),
                        GNode::nodec(
                            None,
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
            None,
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
