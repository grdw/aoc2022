use std::fs;

fn main() {
	let file = fs::read_to_string("input").unwrap();
    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<u64> {
    let (mut total, mut subdirs) = (0, vec![]);
    loop {
        match input
            .next()
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .as_deref()
        {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", s]) if *s != "/" => {
                subdirs.extend(parse(input));
                total += subdirs.last().unwrap();
            }
            Some([s, _]) if *s != "$" && *s != "dir" => {
                total += s.parse::<u64>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(total);
    subdirs
}

fn part1(input: &str) -> u64 {
    parse(&mut input.lines())
        .into_iter()
        .filter(|&s| s <= 100_000)
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut sizes = parse(&mut input.lines());
    let missing = 30_000_000 - (70_000_000 - sizes.last().unwrap());
    sizes.sort_unstable();
    sizes.into_iter().find(|&s| s >= missing).unwrap()
}
