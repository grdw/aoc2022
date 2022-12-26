use std::fs;

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn snafu_number_to_int(string: String) -> i32 {
    let mut num: i32 = 0;
    let mut i: u32 = 0;
    for c in string.chars().rev() {
        let value = match c {
            '1' => 1,
            '2' => 2,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("PURE SNAFU")
        };

        num += 5_i32.pow(i) * value;
        i += 1;
    }
    num
}

#[test]
fn test_snafu_number_to_int() {
    assert_eq!(snafu_number_to_int(String::from("1-0---0")), 12345);
    assert_eq!(snafu_number_to_int(String::from("1121-1110-1=0")), 314159265);
}

fn int_to_snafu_number(mut int: i32) -> String {
    let mut snafu = String::new();
    let fives: i32 = 5;

    while int > 0 {
        let base = int % fives;
        int /= 5;

        let ch = match base {
            0 | 1 | 2 => (b'0' + base as u8) as char,
            3 => {
                int += 1;
                '='
            },
            4 => {
                int += 1;
                '-'
            },
            _ => panic!("DEATH TO LOGIC")
        };
        snafu.insert(0, ch);
    }

    snafu
}

#[test]
fn test_int_to_snafu_number() {
    assert_eq!(int_to_snafu_number(2), String::from("2"));
    assert_eq!(int_to_snafu_number(3), String::from("1="));
    assert_eq!(int_to_snafu_number(12345), String::from("1-0---0"));
    assert_eq!(int_to_snafu_number(314159265), String::from("1121-1110-1=0"));
}

fn part1(file: &'static str) -> String {
    let snafu_numbers = parse(file);

    let total = snafu_numbers.iter().map(|s| snafu_number_to_int(s.to_string())).sum::<i32>();

    println!("{}", total);
    int_to_snafu_number(total)
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), String::from("2=-1=0"));
}

fn part2(file: &'static str) -> usize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(input: &'static str) -> Vec<String> {
    let content = fs::read_to_string(input).unwrap();
    content
        .split_terminator("\n")
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
}
