use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

type RNode = Rc<RefCell<Node>>;

fn main() {
    let display_string = fs::read_to_string("input")
                            .unwrap_or("".to_string());

    //let node = Node::rc_root();
    //parse(node.clone());
    //let result = recurse_collapse(node.clone());
    //println!("Part 2: {:?}", result);
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Instruction {
    No,
    Number(u64),
    Op(char)
}

#[derive(Debug)]
pub struct Node {
    children: Vec<RNode>,
    instruction: Instruction
}

impl Node {
    pub fn rc_root() -> RNode {
        Rc::new(
            RefCell::new(
                Node::node(Instruction::No)
            )
        )
    }

    pub fn add_child(&mut self, instruction: Instruction) -> RNode {
        let rc = Rc::new(
            RefCell::new(
                Node::node(instruction)
            )
        );

        self.children.push(rc.clone());
        rc
    }

    fn operate(&self) -> u64 {
        let mut nums = vec![];
        for child in &self.children {
            match child.borrow().instruction {
                Instruction::Number(n) => nums.push(n),
                _ => ()
            }
        }

        match self.instruction {
            Instruction::Op('+') => nums.iter().fold(0, |a, n| a + n),
            Instruction::Op('*') => nums.iter().fold(1, |a, n| a * n),
            Instruction::Op('/') => nums.iter().fold(1, |a, n| a / n),
            Instruction::Op('-') => nums.iter().fold(0, |a, n| a - n),
            Instruction::Number(val) => val,
            _ => panic!("Invalid")
        }
    }

    fn node(instruction: Instruction) -> Node {
        Node {
            children: vec![],
            instruction: instruction
        }
    }

    fn all_leafs(&self) -> bool {
        self.children.iter().all(|n| n.borrow().is_leaf())
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn read_value(&self) -> Option<u64> {
        match &self.children[0].as_ref().borrow().instruction {
            Instruction::Number(n) => Some(*n),
            _ => None
        }
    }
}

pub fn collapse(rc_node: RNode, result: RNode) -> RNode {
    let node = rc_node.borrow();

    if node.all_leafs() {
        let val = node.operate();
        result.borrow_mut().add_child(Instruction::Number(val));
        result
    } else {
        let n = match node.instruction {
            Instruction::Op(n) => {
                result.borrow_mut().add_child(Instruction::Op(n))
            },
            _ => result.clone()
        };

        for i in 0..node.children.len() {
            collapse(node.children[i].clone(), n.clone());
        }
        result
    }
}

pub fn recurse_collapse(rc_node: RNode) -> u64 {
    if let Some(n) = rc_node.borrow().read_value() {
        n
    } else {
        recurse_collapse(
            collapse(rc_node.clone(), Node::rc_root())
        )
    }
}

#[test]
fn test_unwind_sum() {
    let root = Node::rc_root();
    let add_root = root.borrow_mut().add_child(Instruction::Op('+'));
    add_root.borrow_mut().add_child(Instruction::Number(25));
    add_root.borrow_mut().add_child(Instruction::Number(10));
    add_root.borrow_mut().add_child(Instruction::Number(1));

    let new_node = Node::rc_root();
    collapse(root, new_node.clone());
    let number = new_node.borrow().read_value();

    assert_eq!(number, Some(36));
}

#[test]
fn test_unwind_min() {
    let root = Node::rc_root();
    let add_root = root.borrow_mut().add_child(Instruction::Op('-'));
    add_root.borrow_mut().add_child(Instruction::Number(25));
    add_root.borrow_mut().add_child(Instruction::Number(10));
    add_root.borrow_mut().add_child(Instruction::Number(1));

    let new_node = Node::rc_root();
    collapse(root, new_node.clone());
    let number = new_node.borrow().read_value();

    assert_eq!(number, Some(14));
}

#[test]
fn test_unwind_divide() {
    let root = Node::rc_root();
    let add_root = root.borrow_mut().add_child(Instruction::Op('/'));
    add_root.borrow_mut().add_child(Instruction::Number(10));
    add_root.borrow_mut().add_child(Instruction::Number(100));

    let new_node = Node::rc_root();
    collapse(root, new_node.clone());
    let number = new_node.borrow().read_value();

    assert_eq!(number, Some(10));
}

#[test]
fn test_unwind_multiply() {
    let root = Node::rc_root();
    let mul1 = root.borrow_mut().add_child(Instruction::Op('*'));
    let add1 = mul1.borrow_mut().add_child(Instruction::Op('+'));
    let add2 = mul1.borrow_mut().add_child(Instruction::Op('+'));
    add1.borrow_mut().add_child(Instruction::Number(5));
    add1.borrow_mut().add_child(Instruction::Number(2));
    add2.borrow_mut().add_child(Instruction::Number(6));
    add2.borrow_mut().add_child(Instruction::Number(4));

    assert_eq!(recurse_collapse(root), 70);
}
