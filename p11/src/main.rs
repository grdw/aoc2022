use std::fs;

type Monkeys = Vec<Monkey>;

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: String,
    test: u32,
    test_positive_id: usize,
    test_negative_id: usize
}

impl Monkey {
    pub fn determine_worry_level(&mut self) {
        let operands: Vec<&str> = self.operation.split(" ").collect();

        for item in &mut self.items {
            let n = operands[2].parse::<u32>().unwrap_or(*item);

            match operands[1] {
                "*" => *item *= n,
                "+" => *item += n,
                _ => panic!("Invalid operation")
            }

            *item /= 3
        }
    }

    pub fn throw_id(&self, value: u32) -> usize {
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
    let mut move_items = vec![vec![]; monkeys.len()];
    let mut inspection_count = vec![0; monkeys.len()];

    for _ in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_id];
            monkey.items.append(&mut move_items[monkey_id]);
            inspection_count[monkey_id] += monkey.items.len();
            monkey.determine_worry_level();

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
    0
}
#[test]
fn test_part2() {
    assert_eq!(part2("test_input"), 8);
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
                        items.push(item.trim().parse::<u32>().unwrap());
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
                    test = method.parse::<u32>().unwrap();
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
