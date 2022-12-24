use std::fs;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> i64 {
    let nums = parse_file(file);
    decrypt(&nums, 1, 1)
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 3);
}

fn decrypt(
    nums: &Vec<i64>,
    decryption_key: i64,
    iterations: usize) -> i64 {

    let nums = nums
        .iter()
        .map(|x| x * decryption_key)
        .collect::<Vec<_>>();

    let mut ans = (0..nums.len()).collect::<Vec<_>>();
    for _ in 0..iterations {
        for (i, &x) in nums.iter().enumerate() {
            let pos = ans.iter().position(|&y| y == i).unwrap();
            ans.remove(pos);

            let new_i = (pos as i64 + x).rem_euclid(ans.len() as i64) as usize;
            ans.insert(new_i, i);
        }
    }
    let orig_zero_i = nums.iter().position(|&i| i == 0).unwrap();
    let zero_i = ans.iter().position(|&i| i == orig_zero_i).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| nums[ans[(zero_i + i) % ans.len()]])
        .sum()
}

#[test]
fn test_example_decrypt() {
    let vector = vec![1, 2, -3, 3, -2, 0, 4];
    assert_eq!(decrypt(&vector, 1, 1), 3);
}


fn part2(file: &'static str) -> i64 {
    let nums = parse_file(file);
    decrypt(&nums, 811589153, 10)
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1623178306);
}

fn parse_file(file: &'static str) -> Vec<i64> {
    let contents = fs::read_to_string(file).unwrap();
    contents
        .split_terminator("\n")
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}
