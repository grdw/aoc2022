mod gnode;

use std::fs;
use gnode::{GNode, NType};

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let parsed = fs::read_to_string(file).unwrap();
    let mut total = 0;
    for (i, group) in parsed.split_terminator("\n\n").enumerate() {
        let (left, right) = group.split_once("\n").unwrap();
        let l_tree = gnode::parse_tree(left);
        let r_tree = gnode::parse_tree(right);
        let mut ordered = 0;
        let j = i + 1;
        println!("\n 🌿 PAIR: {}", j);
        traverse(&l_tree, &r_tree, &mut ordered);

        if ordered == 2 {
            println!("🎉 {}", j);
            total += j
        }
    }
    total
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 13);
    assert_eq!(part1("test_input2"), 0);
    assert_eq!(part1("test_input3"), 0);
    assert_eq!(part1("test_input4"), 1);
    assert_eq!(part1("test_input5"), 0);
    assert_eq!(part1("test_input6"), 1);
    assert_eq!(part1("test_input7"), 0);
}

fn traverse(l: &GNode, r: &GNode, ordered: &mut u8) {
    println!("{:?} {:?}", l.value, r.value);
    match (&l.value, &r.value) {
        (NType::Value(l_val), NType::Value(r_val)) => {
            if l_val < r_val {
                *ordered = 2;
            } else if l_val == r_val {
                *ordered = 0;
            } else {
                *ordered = 1;
            }
        },
        (NType::Value(l_val), NType::List) => {
            let mut l_list = GNode::list();
            l_list.add(*l_val);

            traverse(&l_list, r, ordered)
        },
        (NType::List, NType::Value(r_val)) => {
            let mut r_list = GNode::list();
            r_list.add(*r_val);

            traverse(l, &r_list, ordered)
        },
        (NType::List, NType::List) => {
            let mut i = 0;
            loop {
                match (l.children.get(i), r.children.get(i)) {
                    (Some(lc), Some(rc)) => {
                        traverse(lc, rc, ordered);

                        if ordered != &0 {
                            break;
                        }
                    },
                    (None, Some(rc)) => *ordered = 2,
                    (None, None) => break,
                    (_, _) => ()
                }
                i += 1
            }
        },
        (_, _) => ()
    }
}

#[test]
fn test_traverse_lists_in_lists_left() {
    let l_tree = gnode::parse_tree("[[[5]]]");
    let r_tree = gnode::parse_tree("[7]");
    let mut ordered = 0;

    traverse(&l_tree, &r_tree, &mut ordered);
    assert_eq!(ordered, 2)
}

#[test]
fn test_traverse_lists_in_lists_left_empty() {
    let l_tree = gnode::parse_tree("[[[]]]");
    let r_tree = gnode::parse_tree("[7]");
    let mut ordered = 0;

    traverse(&l_tree, &r_tree, &mut ordered);
    assert_eq!(ordered, 2)
}

#[test]
fn test_traverse_lists_in_lists_right() {
    let l_tree = gnode::parse_tree("[[[5]]]");
    let r_tree = gnode::parse_tree("[[[[[7]]]]]");
    let mut ordered = 0;

    traverse(&l_tree, &r_tree, &mut ordered);
    assert_eq!(ordered, 2)
}

#[test]
fn test_traverse_lists_in_lists_right_empty() {
    let l_tree = gnode::parse_tree("[[[5]]]");
    let r_tree = gnode::parse_tree("[[[[[]]]]]");
    let mut ordered = 0;

    traverse(&l_tree, &r_tree, &mut ordered);
    assert_eq!(ordered, 0)
}

#[test]
fn test_traverse_example_reddit_1() {
    let l_tree = gnode::parse_tree("[[1,2],4]");
    let r_tree = gnode::parse_tree("[[1,2],3]");
    let mut ordered = 0;

    traverse(&l_tree, &r_tree, &mut ordered);
    assert_eq!(ordered, 1)
}

// [[1,4,6,[]]]
// [[1,3],[2],[6,6,0,[5,[],9]],[[2,[1,4,5],[6],[5,10,9,4]],[7,4,6,[2,6,8,9],[5,6]],[[8],[]],[[9,5,6],[]],8],[[0,[2,9,6,3],[5,3]]]]
// :Incorrect
#[test]
fn test_traverse_example_reddit_2() {
    let l_tree = gnode::parse_tree("[[1,4,6,[]]]");
    let r_tree = gnode::parse_tree("[[1,3],[2],[6,6,0,[5,[],9]],[[2,[1,4,5],[6],[5,10,9,4]],[7,4,6,[2,6,8,9],[5,6]],[[8],[]],[[9,5,6],[]],8],[[0,[2,9,6,3],[5,3]]]]");
    let mut ordered = 0;

    traverse(&l_tree, &r_tree, &mut ordered);
    assert_eq!(ordered, 1)
}

// [[4,2,[[4,3],2,[4,5,5,1,1]]],[[8,[7],[9,7],4,[0,3,8,6]],0],[5],[2,2,[10,10,[3,6,10,2,5],[],0]]]
// [[[[6,3,2,2,6],1,[0,2,5,9,4],[6],[2]],9,[7,3]],[[[7],7,4,2,2],5,6,[3],3]]
// :Correct
#[test]
fn test_traverse_example_reddit_3() {
    let l_tree = gnode::parse_tree("[[4,2,[[4,3],2,[4,5,5,1,1]]],[[8,[7],[9,7],4,[0,3,8,6]],0],[5],[2,2,[10,10,[3,6,10,2,5],[],0]]]");
    let r_tree = gnode::parse_tree("[[[[6,3,2,2,6],1,[0,2,5,9,4],[6],[2]],9,[7,3]],[[[7],7,4,2,2],5,6,[3],3]]");
    let mut ordered = 0;

    traverse(&l_tree, &r_tree, &mut ordered);
    assert_eq!(ordered, 2)
}

fn part2(file: &'static str) -> usize {
    0
}
