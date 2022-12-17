#[derive(Debug, PartialEq)]
pub enum NType {
    List,
    Value(u8)
}

#[derive(Debug, PartialEq)]
pub struct GNode {
    pub value: NType,
    pub children: Vec<GNode>
}

impl GNode {
    pub fn list() -> GNode {
        GNode { value: NType::List, children: vec![] }
    }

    pub fn node(val: u8) -> GNode {
        GNode { value: NType::Value(val), children: vec![] }
    }

    pub fn nodec(val: NType, children: Vec<GNode>) -> GNode {
        GNode { value: val, children: children }
    }

    pub fn add(&mut self, val: u8) {
        self.children.push(GNode::node(val));
    }

    pub fn debug(&self, depth: usize) {
        for child in &self.children {
            println!("|{}{:?}", "___".repeat(depth), child.value);
            child.debug(depth + 1)
        }
    }
}

pub fn parse_tree(line: &str) -> GNode {
    let mut root: GNode = GNode::list();
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

    n.children.push(GNode::list());
}

#[test]
fn test_parse_tree() {
    assert_eq!(
        parse_tree("[1]"),
        GNode::nodec(NType::List, vec![GNode::node(1)])
    );

    assert_eq!(
        parse_tree("[[1],[2]]"),
        GNode::nodec(
            NType::List,
            vec![
                GNode::nodec(NType::List, vec![GNode::node(1)]),
                GNode::nodec(NType::List, vec![GNode::node(2)])
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,[2]]"),
        GNode::nodec(
            NType::List,
            vec![
                GNode::node(1),
                GNode::nodec(NType::List, vec![GNode::node(2)])
            ]
        )
    );

    assert_eq!(
        parse_tree("[1,[2,3]]"),
        GNode::nodec(
            NType::List,
            vec![
                GNode::node(1),
                GNode::nodec(
                    NType::List,
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
            NType::List,
            vec![
                GNode::node(1),
                GNode::node(5),
                GNode::nodec(
                    NType::List,
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
            NType::List,
            vec![
                GNode::node(1),
                GNode::nodec(
                    NType::List,
                    vec![
                        GNode::node(2),
                        GNode::nodec(
                            NType::List,
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
            NType::List,
            vec![]
        )
    );
}

