use std::fs;
use regex::Regex;

enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode
}

enum Cost {
    Ore(u8),
    Clay(u8),
    Obsidian(u8)
}

struct Robot {
    robot_type: RobotType,
    costs: Vec<Cost>
}

struct Blueprint {
    robots: Vec<Robot>
}

fn main() {
    println!("P1: {}", part1("input"));
    println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let blueprints = parse(file);
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 1);
}

fn part2(file: &'static str) -> usize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(file: &'static str) {
    let mut blueprints: Vec<Blueprint> = vec![];
    let re = Regex::new(r"Each ([a-z]+) robot costs ([a-z 0-9]+)").unwrap();
    let file = fs::read_to_string(file).unwrap();
    for line in file.split_terminator("\n") {
        let (_, rules) = line.split_once(":").unwrap();
        let mut robots: Vec<Robot> = vec![];

        for rule in rules.split(". ") {
            let rule = rule.trim();
            let caps = re.captures(rule).unwrap();
            let robot_type = match &caps[1] {
                "ore"      => RobotType::Ore,
                "clay"     => RobotType::Clay,
                "obsidian" => RobotType::Obsidian,
                "geode"    => RobotType::Geode,
                _          => panic!("Invalid robot type!")
            };

            let mut costs = vec![];
            for cost in caps[2].split(" and ") {
                let (price, material) = cost.split_once(" ").unwrap();
                let price = price.parse::<u8>().unwrap();

                costs.push(
                    match material {
                        "ore"      => Cost::Ore(price),
                        "clay"     => Cost::Clay(price),
                        "obsidian" => Cost::Obsidian(price),
                        _          => panic!("Invalid cost type")
                    }
                );
            }

            robots.push(
                Robot {
                    robot_type: robot_type,
                    costs: costs
                }
            );
        }

        blueprints.push(Blueprint { robots: robots });
    }
}
