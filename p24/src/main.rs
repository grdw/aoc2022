use std::fs;
use std::cmp;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
    p_type: char
}

#[derive(Debug)]
struct Node {
    x: isize,
    y: isize,
    minute: usize,
    action: char,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<RNode>
}

impl Node {
    fn rc_root() -> RNode {
        Rc::new(
            RefCell::new(
                Node::root(1, 0, 'I', 0)
            )
        )
    }

    fn add_child(
        &mut self,
        x: isize,
        y: isize,
        action: char,
        minute: usize,
        parent: Weak<RefCell<Node>>) -> RNode {

        let rc = Rc::new(
            RefCell::new(
                Node::node(x, y, action, minute, parent)
            )
        );

        self.children.push(rc.clone());
        rc
    }

    fn node(
        x: isize,
        y: isize,
        action: char,
        minute: usize,
        parent: Weak<RefCell<Node>>) -> Node {
        Node {
            x: x,
            y: y,
            parent: Some(parent),
            minute: minute,
            action: action,
            children: vec![]
        }
    }

    fn root(
        x: isize,
        y: isize,
        action: char,
        minute: usize) -> Node {
        Node {
            x: x,
            y: y,
            parent: None,
            minute: minute,
            action: action,
            children: vec![]
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

type Grid = Vec<Point>;
type RNode = Rc<RefCell<Node>>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let mut basin = parse(file);
    let (start_x, end_x, start_y, end_y) = maxes(&basin);

    let end = Point {
        x: end_x - 1,
        y: end_y,
        p_type: 'E'
    };

    let node = Node::rc_root();
    let mut nodes = vec![vec![node.clone()]];

    move_me(&mut nodes, &mut basin, &end);
    let mut depths = vec![];
    find_depth(node.clone(), &mut depths);
    println!("{:?}", depths);

    depths.iter().min().unwrap() + 1
}

fn find_depth(node: RNode, depths: &mut Vec<usize>) {
    let n = node.borrow();

    if n.is_leaf() {
        depths.push(n.minute);
    }

    for c in &node.borrow().children {
        find_depth(c.clone(), depths);
    }
}

fn maxes(grid: &Grid) -> (isize, isize, isize, isize) {
    let mi_x = 0;
    let mx_x = grid.iter().map(|p| p.x).max().unwrap();
    let mi_y = 0;
    let mx_y = grid.iter().map(|p| p.y).max().unwrap();

    (mi_x, mx_x, mi_y, mx_y)
}

fn move_me(nodes: &mut Vec<Vec<RNode>>, grid: &mut Grid, end: &Point) {
    let mut minutes = 0;
    let mut set = HashSet::new();

    while let Some(l_nodes) = nodes.pop() {
        minutes += 1;
        move_blizzards(grid);

        for node in &l_nodes {
            let mut new_nodes = vec![];
            let mut n = node.borrow_mut();
            let (x, y) = (n.x, n.y);
            let mut idle = true;
            let max_y = grid.iter().map(|p| p.y).max().unwrap();

            let possible_positions = vec![
                (0, 1,  'v'), // D
                (0, -1, '^'), // U
                (-1, 0, '<'), // L
                (1, 0,  '>')  // R
            ];

            for (tx, ty, action) in possible_positions {
                let px = x + tx;
                let py = y + ty;

                if let Some(_) = grid.iter().find(|p| p.x == px && p.y == py) {
                    continue
                }

                if py < 1 || py > max_y {
                    continue;
                }

                // This limits my movement up and down
                // by 1 step :(
                //let mut set = HashSet::new();
                //let mut count = 0;
                //n.repeat_count(&mut set, &mut count);

                if (py < y || px < x) {
                    continue
                }

                idle = false;
                let child = n.add_child(
                    px,
                    py,
                    action,
                    minutes,
                    Rc::downgrade(&node)
                );
                new_nodes.push(child.clone());
            }

            if idle && x != end.x && y != end.y {
                n.minute = minutes;
                new_nodes.push(node.clone());
            }

            nodes.push(new_nodes);
        }
    }
}

fn move_blizzards(grid: &mut Grid) {
    let (__x, end_x, _y, end_y) = maxes(&grid);
    let width = end_x;
    let height = end_y;

    for p in grid {
        match p.p_type {
            'v' => p.y += 1,
            '>' => p.x += 1,
            '^' => p.y -= 1,
            '<' => p.x -= 1,
            _   => continue
        }

        if p.x == 0 {
            p.x = width - 1
        } else if p.x == width {
            p.x = 1
        }

        if p.y == 0 {
            p.y = height - 1
        } else if p.y == height {
            p.y = 1
        }
    }
}

fn debug_blizzards(grid: &Grid) {
    let (min_x, max_x, min_y, max_y) = maxes(grid);

    for y in min_y..=max_y {
        let mut s = String::new();
        for x in min_x..=max_x {
            let f = grid.iter().find(|p| p.x == x && p.y == y);

            match f {
                Some(p) => s.push(p.p_type),
                None => s.push('.')
            }
        }
        println!("{}", s);
    }
}

#[test]
fn test_part1() {
    //assert_eq!(part1("test_input"), 1);
    assert_eq!(part1("test_input2"), 18);
}

fn part2(file: &'static str) -> isize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(file: &'static str) -> Grid {
    let mut grid = vec![];
    let content = fs::read_to_string(file).unwrap();

    for (y, line) in content.split_terminator("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                grid.push(
                    Point {
                        x: x as isize,
                        y: y as isize,
                        p_type: c
                    }
                );
            }
        }
    }

    grid
}
