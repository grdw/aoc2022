use std::fs;

type Monkeys = Vec<Monkey>;

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: String,
    test: u128,
    test_positive_id: usize,
    test_negative_id: usize
}

impl Monkey {
    pub fn determine_worry_level(&mut self, div: u128, modulo: u128) {
        let operands: Vec<&str> = self.operation.split(" ").collect();

        for item in &mut self.items {
            let n = operands[2].parse::<u128>().unwrap_or(*item);

            match operands[1] {
                "*" => *item *= n,
                "+" => *item += n,
                _ => panic!("Invalid operation")
            }

            *item = *item / div;
            *item = *item % modulo;
        }
    }

    pub fn throw_id(&self, value: u128) -> usize {
        if value % self.test == 0 {
            self.test_positive_id
        } else {
            self.test_negative_id
        }
    }
}

fn main() {
    println!("Part 1: {}", part1("input"));
    println!("Part 2: {}", part2("input"));
}

fn part1(file: &'static str) -> usize {
    let mut monkeys = parse(file);
    trick(&mut monkeys, 20, 3)
}

fn trick(monkeys: &mut Monkeys, steps: usize, division: u128) -> usize {
    let mut move_items = vec![vec![]; monkeys.len()];
    let mut inspection_count = vec![0; monkeys.len()];
    let total_mods: u128 = monkeys.iter().map(|n| n.test).product();

    for _ in 0..steps {
        for monkey_id in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_id];
            monkey.items.append(&mut move_items[monkey_id]);
            inspection_count[monkey_id] += monkey.items.len();
            monkey.determine_worry_level(division, total_mods);

            while let Some(item) = monkey.items.pop() {
                let id = monkey.throw_id(item);

                move_items[id].push(item)
            }
        }
    }

    inspection_count.sort();
    inspection_count[monkeys.len() - 1] * inspection_count[monkeys.len() - 2]
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 10605);
}

fn part2(file: &'static str) -> usize {
    let mut monkeys = parse(file);
    trick(&mut monkeys, 10_000, 1)
}
#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 2713310158);
}

fn parse(file: &'static str) -> Monkeys {
    let monkey_text = fs::read_to_string(file).unwrap();
    let mut monkeys: Monkeys = vec![];

    for monkey in monkey_text.split_terminator("\n\n") {
        let mut items = vec![];
        let mut operation = String::new();
        let mut test = 0;
        let mut test_positive_id = 0;
        let mut test_negative_id = 0;

        for n in monkey.split_terminator("\n") {
            let (key, val) = n.split_once(":").unwrap();

            match key {
                "  Starting items" => {
                    for item in val.split(", ") {
                        let n = item.trim().parse::<u128>().unwrap();
                        items.push(n);
                    }
                },
                "  Operation" => {
                    let op = val.trim();
                    let (_, method) = op.split_once("new = ").unwrap();
                    operation.push_str(method)
                },
                "  Test" => {
                    let op = val.trim();
                    let (_, method) = op.split_once("divisible by ").unwrap();
                    test = method.parse::<u128>().unwrap();
                },
                "    If true" => {
                    test_positive_id = (
                        (val.chars().nth(val.len() - 1).unwrap() as u8) - 48
                    ) as usize
                },
                "    If false" => {
                    test_negative_id = (
                        (val.chars().nth(val.len() - 1).unwrap() as u8) - 48
                    ) as usize
                },
                _ => ()
            }
        }

        monkeys.push(Monkey {
            items: items,
            operation: operation,
            test: test,
            test_positive_id: test_positive_id,
            test_negative_id: test_negative_id
        });
    };

    monkeys
}
