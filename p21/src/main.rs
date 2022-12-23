use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

type RNode = Rc<RefCell<Node>>;
type Instructions = HashMap<String, Vec<TreeBuildInstruction>>;

#[derive(Debug)]
enum TreeBuildInstruction {
    AddNumber(u64),
    AddOp(char),
    AddChild(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Instruction {
    Number(u64),
    Op(char)
}

#[derive(Debug)]
struct Node {
    children: Vec<RNode>,
    instruction: Instruction
}

impl Node {
    fn rc_root() -> RNode {
        Rc::new(
            RefCell::new(
                Node::node(Instruction::Number(0))
            )
        )
    }

    fn add_child(&mut self, instruction: Instruction) -> RNode {
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
            Instruction::Op('+') => nums[0] + nums[1],
            Instruction::Op('*') => nums[0] * nums[1],
            Instruction::Op('/') => nums[0] / nums[1],
            Instruction::Op('-') => nums[0] - nums[1],
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

    fn read_value(&self) -> Option<u64> {
        match &self.children[0].as_ref().borrow().instruction {
            Instruction::Number(n) => Some(*n),
            _ => None
        }
    }
}

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> u64 {
    let contents = parse(file);
    let node = Node::rc_root();

    build_tree(node.clone(), &contents, "root".to_string());
    recurse_collapse(node.clone())
}

fn build_tree(node: RNode, contents: &Instructions, node_name: String) {
    let list = &contents[&node_name];

    match &list[0] {
        TreeBuildInstruction::AddOp(op) => {
            let p = node.borrow_mut().add_child(Instruction::Op(*op));

            for item in &list[1..] {
                if let TreeBuildInstruction::AddChild(name) = item {
                    build_tree(p.clone(), contents, name.to_string());
                }
            }
        },

        TreeBuildInstruction::AddNumber(n) => {
            node.borrow_mut().add_child(Instruction::Number(*n));
        },
        _ => ()
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 152);
}

fn part2(file: &'static str) -> usize {
    0
}

fn collapse(rc_node: RNode, result: RNode) -> RNode {
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

fn recurse_collapse(rc_node: RNode) -> u64 {
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

    let new_node = Node::rc_root();
    collapse(root, new_node.clone());
    let number = new_node.borrow().read_value();

    assert_eq!(number, Some(35));
}

#[test]
fn test_unwind_min() {
    let root = Node::rc_root();
    let add_root = root.borrow_mut().add_child(Instruction::Op('-'));
    add_root.borrow_mut().add_child(Instruction::Number(25));
    add_root.borrow_mut().add_child(Instruction::Number(10));

    let new_node = Node::rc_root();
    collapse(root, new_node.clone());
    let number = new_node.borrow().read_value();

    assert_eq!(number, Some(15));
}

#[test]
fn test_unwind_divide() {
    let root = Node::rc_root();
    let add_root = root.borrow_mut().add_child(Instruction::Op('/'));
    add_root.borrow_mut().add_child(Instruction::Number(100));
    add_root.borrow_mut().add_child(Instruction::Number(10));

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

fn parse(file: &'static str) -> Instructions {
    let contents = fs::read_to_string(file).unwrap();
    let mut inst_map = HashMap::new();

    for line in contents.split_terminator("\n") {
        let (name, instruction) = line.split_once(": ").unwrap();

        let parse = instruction.parse::<u64>();

        let n = if parse.is_err() {
            let left = &instruction[0..4];
            let right = &instruction[7..11];
            let calc = instruction.chars().nth(5).unwrap();
            vec![
                TreeBuildInstruction::AddOp(calc),
                TreeBuildInstruction::AddChild(left.to_string()),
                TreeBuildInstruction::AddChild(right.to_string())
            ]
        } else {
            vec![
                TreeBuildInstruction::AddNumber(parse.unwrap())
            ]
        };

        inst_map.insert(name.to_string(), n);
    }

    inst_map
}
