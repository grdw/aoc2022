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
    name: String,
    children: Vec<RNode>,
    instruction: Instruction
}

impl Node {
    fn rc_root() -> RNode {
        Rc::new(
            RefCell::new(
                Node::node(String::from("rc_root"), Instruction::Number(0))
            )
        )
    }

    fn add_child(&mut self, name: String, instruction: Instruction) -> RNode {
        let rc = Rc::new(
            RefCell::new(
                Node::node(name, instruction)
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
            Instruction::Op('=') => {
                if nums[0] == nums[1] {
                    0
                } else if nums[0] > nums[1] {
                    1
                } else if nums[0] < nums[1] {
                    2
                } else {
                    panic!("DEATH TO LOGIC")
                }
            },
            Instruction::Number(val) => val,
            _ => panic!("Invalid")
        }
    }

    fn node(name: String, instruction: Instruction) -> Node {
        Node {
            name: name,
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

    fn read(&self) -> Option<u64> {
        match self.instruction {
            Instruction::Number(n) => Some(n),
            _ => None
        }
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
    let contents = parse(file, 1);
    let node = Node::rc_root();

    build_tree(node.clone(), &contents, "root".to_string());
    recurse_collapse(node.clone())
}

fn build_tree(node: RNode, contents: &Instructions, node_name: String) {
    let list = &contents[&node_name];

    match &list[0] {
        TreeBuildInstruction::AddOp(op) => {
            let p = node.borrow_mut().add_child(node_name, Instruction::Op(*op));

            for item in &list[1..] {
                if let TreeBuildInstruction::AddChild(name) = item {
                    build_tree(p.clone(), contents, name.to_string());
                }
            }
        },

        TreeBuildInstruction::AddNumber(n) => {
            node.borrow_mut().add_child(node_name, Instruction::Number(*n));
        },
        _ => ()
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 152);
}

fn part2(file: &'static str) -> u64 {
    let contents = parse(file, 2);
    let node = Node::rc_root();

    build_tree(node.clone(), &contents, "root".to_string());

    let op_node = &node.borrow().children[0].clone();
    let mut best_guess = 1;
    let mut log_factor = initial_log_factor(&op_node) - 2;
    let mut add_factor = 1;

    loop {
        let value = recurse_collapse(node.clone());
        add_factor = 10_u64.pow(log_factor);

        if value == 0 {
            break;
        } else if value == 1 { // too high
            best_guess += add_factor;
        } else if value == 2 { // too low
            best_guess -= add_factor;
            log_factor -= 1;
        }

        let me = find_node(node.clone(), "humn").unwrap();
        set_node_value(me, best_guess);
    }

    let search = find_node(node.clone(), "humn");
    search
        .unwrap()
        .borrow()
        .read()
        .unwrap()
}

fn initial_log_factor(rc_node: &RNode) -> u32 {
    let mut b = vec![];
    for c in &rc_node.borrow().children {
        let value = recurse_collapse(c.clone());
        b.push(value);
    }

    digit_length(b[1])
}

fn digit_length(n: u64) -> u32 {
    (n as f64).log10().ceil() as u32
}

fn find_node(rc_node: RNode, name: &'static str) -> Option<RNode> {
    let node = rc_node.borrow_mut();
    let name_string = name.to_string();

    if node.name == name_string {
        return Some(rc_node.clone());
    }

    for i in 0..node.children.len() {
        let child = node.children[i].clone();

        let val = find_node(child, name);
        if val.is_some() {
            return val;
        }
    }

    return None
}

fn set_node_value(rc_node: RNode, value: u64) {
    let mut node = rc_node.borrow_mut();

    if let Instruction::Number(_) = node.instruction {
        node.instruction = Instruction::Number(value);
    }
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 301);
}

fn collapse(rc_node: RNode, result: RNode) -> RNode {
    let node = rc_node.borrow();

    if node.all_leafs() {
        let val = node.operate();
        result.borrow_mut().add_child(node.name.to_string(), Instruction::Number(val));
        result
    } else {
        let n = match node.instruction {
            Instruction::Op(n) => {
                result.borrow_mut().add_child(node.name.to_string(), Instruction::Op(n))
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
    let add_root = root.borrow_mut().add_child(String::from("B"), Instruction::Op('+'));
    add_root.borrow_mut().add_child(String::from("A"), Instruction::Number(25));
    add_root.borrow_mut().add_child(String::from("A"), Instruction::Number(10));

    let new_node = Node::rc_root();
    collapse(root, new_node.clone());
    let number = new_node.borrow().read_value();

    assert_eq!(number, Some(35));
}

#[test]
fn test_unwind_equals() {
    let root = Node::rc_root();
    let add_root = root.borrow_mut().add_child(String::from("A"), Instruction::Op('='));
    add_root.borrow_mut().add_child(String::from("A"), Instruction::Number(25));
    add_root.borrow_mut().add_child(String::from("A"), Instruction::Number(25));

    let new_node = Node::rc_root();
    collapse(root, new_node.clone());
    let number = new_node.borrow().read_value();

    assert_eq!(number, Some(0));
}

#[test]
fn test_unwind_min() {
    let root = Node::rc_root();
    let add_root = root.borrow_mut().add_child(String::from("A"), Instruction::Op('-'));
    add_root.borrow_mut().add_child(String::from("A"), Instruction::Number(25));
    add_root.borrow_mut().add_child(String::from("A"), Instruction::Number(10));

    let new_node = Node::rc_root();
    collapse(root, new_node.clone());
    let number = new_node.borrow().read_value();

    assert_eq!(number, Some(15));
}

#[test]
fn test_unwind_divide() {
    let root = Node::rc_root();
    let add_root = root.borrow_mut().add_child(String::from("A"), Instruction::Op('/'));
    add_root.borrow_mut().add_child(String::from("B"), Instruction::Number(100));
    add_root.borrow_mut().add_child(String::from("C"), Instruction::Number(10));

    let new_node = Node::rc_root();
    collapse(root, new_node.clone());
    let number = new_node.borrow().read_value();

    assert_eq!(number, Some(10));
}

#[test]
fn test_unwind_multiply() {
    let root = Node::rc_root();
    let mul1 = root.borrow_mut().add_child(String::from("A"), Instruction::Op('*'));
    let add1 = mul1.borrow_mut().add_child(String::from("B"), Instruction::Op('+'));
    let add2 = mul1.borrow_mut().add_child(String::from("C"), Instruction::Op('+'));
    add1.borrow_mut().add_child(String::from("BA"), Instruction::Number(5));
    add1.borrow_mut().add_child(String::from("BB"), Instruction::Number(2));
    add2.borrow_mut().add_child(String::from("CA"), Instruction::Number(6));
    add2.borrow_mut().add_child(String::from("CB"), Instruction::Number(4));

    assert_eq!(recurse_collapse(root), 70);
}

fn parse(file: &'static str, part: usize) -> Instructions {
    let contents = fs::read_to_string(file).unwrap();
    let mut inst_map = HashMap::new();

    for line in contents.split_terminator("\n") {
        let (name, instruction) = line.split_once(": ").unwrap();

        let parse = instruction.parse::<u64>();

        let n = if parse.is_err() {
            let left = &instruction[0..4];
            let right = &instruction[7..11];
            let mut calc = instruction.chars().nth(5).unwrap();

            if name == "root" && part == 2 {
                calc = '=';
            }

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
