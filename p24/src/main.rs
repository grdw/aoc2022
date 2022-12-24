use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Point {
    id: usize,
    x: isize,
    y: isize,
    p_type: char
}

#[derive(Debug)]
struct Node {
    x: isize,
    y: isize,
    action: char,
    children: Vec<RNode>
}

impl Node {
    fn rc_root() -> RNode {
        Rc::new(
            RefCell::new(
                Node::node(1, 0, 'I')
            )
        )
    }

    fn add_child(&mut self, x: isize, y: isize, action: char) -> RNode {
        let rc = Rc::new(
            RefCell::new(
                Node::node(x, y, action)
            )
        );

        self.children.push(rc.clone());
        rc
    }

    fn node(x: isize, y: isize, action: char) -> Node {
        Node {
            x: x,
            y: y,
            action: action,
            children: vec![]
        }
    }

    fn all_leafs(&self) -> bool {
        self.children.iter().all(|n| n.borrow().is_leaf())
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
    let mut minutes = 0;
    let mut basin = parse(file);
    let (start_x, end_x, start_y, end_y) = maxes(&basin);
    let width = end_x;
    let height = end_y;

    let mut start = Point {
        id: 0,
        x: start_x + 1,
        y: start_y,
        p_type: 'S'
    };

    let end = Point {
        id: ((end_x * end_y) + 1) as usize,
        x: end_x - 1,
        y: end_y,
        p_type: 'E'
    };

    let mut node = Node::rc_root();
    let mut nodes = vec![vec![node.clone()]];

    debug_blizzards(&basin);
    move_me(&mut nodes, &mut basin, &end);
    debug_blizzards(&basin);

    minutes
}

fn maxes(grid: &Grid) -> (isize, isize, isize, isize) {
    let mi_x = 0;
    let mx_x = grid.iter().map(|p| p.x).max().unwrap();
    let mi_y = 0;
    let mx_y = grid.iter().map(|p| p.y).max().unwrap();

    (mi_x, mx_x, mi_y, mx_y)
}

fn move_me(nodes: &mut Vec<Vec<RNode>>, grid: &mut Grid, end: &Point) {
    while let Some(l_nodes) = nodes.pop() {
        move_blizzards(grid);

        for node in &l_nodes {
            let mut new_nodes = vec![];
            let mut n = node.borrow_mut();
            let (mut x, mut y) = (n.x, n.y);
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

                if let Some(fp) = grid.iter().find(|p| p.x == px && p.y == py) {
                    continue
                }

                if py < 0 || py > max_y {
                    continue;
                }

                if py < y || px < x {
                    continue
                }

                idle = false;
                let child = n.add_child(px, py, action);
                new_nodes.push(child.clone());
            }

            if idle && x != end.x && y != end.y {
                let child = n.add_child(x, y, 'I');
                new_nodes.push(child.clone());
            }

            nodes.push(new_nodes);
        }
    }
}

fn move_blizzards(grid: &mut Grid) {
    let (start_x, end_x, start_y, end_y) = maxes(&grid);
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
    assert_eq!(part1("test_input"), 1);
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
    let mut id = 1;

    for (y, line) in content.split_terminator("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                grid.push(
                    Point {
                        id: id,
                        x: x as isize,
                        y: y as isize,
                        p_type: c
                    }
                );
            }

            id += 1
        }
    }

    grid
}
