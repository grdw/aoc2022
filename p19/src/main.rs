use regex::Regex;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode
}

#[derive(Debug, Clone)]
enum Cost {
    Ore(usize),
    Clay(usize),
    Obsidian(usize)
}

#[derive(Debug, Clone)]
struct Robot {
    robot_type: RobotType,
    costs: Vec<Cost>
}

#[derive(Debug)]
struct Mining {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize
}

impl Robot {
    pub fn backpack_robot() -> Robot {
        Robot { robot_type: RobotType::Ore, costs: vec![] }
    }
}

#[derive(Debug)]
struct Blueprint {
    robots: Vec<Robot>
}

impl Blueprint {
    pub fn buy_robot(&self, mining: &mut Mining, active_robots: &Vec<Robot>) -> Option<Robot> {
        for robot in &self.robots {
            if self.can_pay(robot, mining) && self.fits_buying_strategy(active_robots, mining, robot) {
                for cost in robot.costs.iter() {
                    match cost {
                        Cost::Ore(price)      => mining.ore -= *price,
                        Cost::Clay(price)     => mining.clay -= *price,
                        Cost::Obsidian(price) => mining.obsidian -= *price
                    }
                }
                return Some(robot.clone())
            }
        }
        None
    }

    fn can_pay(&self, robot: &Robot, mining: &Mining) -> bool {
        robot.costs.iter().rev().all(|cost|
            match cost {
                Cost::Ore(price)      => mining.ore >= *price,
                Cost::Clay(price)     => mining.clay >= *price,
                Cost::Obsidian(price) => mining.obsidian >= *price
            }
        )
    }

    fn fits_buying_strategy(&self, active_robots: &Vec<Robot>, mining: &Mining, robot: &Robot) -> bool {
        for cost in robot.costs.iter() {
            let money = match cost {
                Cost::Ore(price)      => *price,
                Cost::Clay(price)     => *price,
                Cost::Obsidian(price) => *price
            };
        }

        let mut robot_counts = HashMap::new();
        for robot in active_robots {
            robot_counts
                .entry(robot.robot_type)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        //println!("{:?}", active_robots);
        //println!("{:?}", mining);

        match robot_counts.get(&robot.robot_type) {
            Some(n) => *n < 3,
            None => true
        }
    }
}

fn main() {
    println!("P1: {}", part1("test_input"));
    //println!("P2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let blueprints = parse(file);
    let mut total = 0;

    for (i, blueprint) in blueprints.iter().enumerate() {
        let quality_level = walk_blueprint(blueprint);
        println!("Blueprint #{} has q: {:?}", i + 1, quality_level);
        total += (quality_level) * (i + 1);
    }
    total
}

fn walk_blueprint(blueprint: &Blueprint) -> usize {
    let mut active_robots = vec![Robot::backpack_robot()];
    let mut building_robots = vec![];
    let mut mining = Mining {
        ore: 0, clay: 0, obsidian: 0, geode: 0
    };

    for _ in 0..24 {
        active_robots.append(&mut building_robots);

        if let Some(robot) = blueprint.buy_robot(&mut mining, &active_robots) {
            building_robots.push(robot);
        }

        for robot in &active_robots {
            match robot.robot_type {
                RobotType::Ore      => mining.ore += 1,
                RobotType::Clay     => mining.clay += 1,
                RobotType::Obsidian => mining.obsidian += 1,
                RobotType::Geode    => mining.geode += 1

            }
        }
    }

    mining.geode
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 33);
}

fn part2(file: &'static str) -> usize {
    0
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 1);
}

fn parse(file: &'static str) -> Vec<Blueprint> {
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
                let price = price.parse::<usize>().unwrap();

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

    blueprints
}
