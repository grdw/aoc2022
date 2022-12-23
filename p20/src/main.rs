use std::fs;

type DodgyLinkedList = Vec<(i32, i32)>;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> i32 {
    let nums = parse_file(file);
    decrypt(&nums)
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 3);
}

fn decrypt(list: &Vec<i32>) -> i32 {
    let l = list.len() as i32;
    let mut move_list = list.clone();

    for (i, n) in list.iter().enumerate() {
        if n == &0 { continue };

        let ci = move_list.iter().position(|m| m == n).unwrap() as i32;

        let pos = if ci + n >= (l - 1) {
            //println!("🍄 {}", n);
            (ci + n) % l
        } else if ci + n <= 0 {
            //println!("🎂 {}", n);
            (ci + n).rem_euclid(l)
        } else {
            //println!("🍇 {}", n);
            ci + *n
        };

        //println!("{} {}", ci, pos);

        //println!("{:?}", move_list);
        move_list.remove(ci as usize);
        move_list.insert(pos as usize, *n);
        //println!("{:?}", move_list);
    }

    //println!("FINAL: {:?}", move_list);

    let qi = move_list.iter().position(|m| m == &0).unwrap();
    (1..=3)
        .map(|n| {
            let j = (n * 1000) + qi;
            let i = j % (list.len() - 1);
            println!("{:?}", move_list[i]);
            move_list[i]
        })
        .sum()
}

#[test]
fn test_example_decrypt() {
    let vector = vec![1, 2, -3, 3, -2, 0, 4];
    assert_eq!(decrypt(&vector), 3);
}

#[test]
fn test_extra_decrypt() {
    let vector = vec![0, 1, 2, 3];
    assert_eq!(decrypt(&vector), 4);

    let vector = vec![0, 1, 2, -3];
    assert_eq!(decrypt(&vector), -2);
}

fn part2(file: &'static str) -> i32 {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse_file(file: &'static str) -> Vec<i32> {
    let contents = fs::read_to_string(file).unwrap();
    contents
        .split_terminator("\n")
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}
