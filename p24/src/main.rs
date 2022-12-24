use std::fs;

const MAX_ROWS: usize = 32;

fn main() {
	println!("P1: {}", part1("input"));
	println!("P2: {}", part2("input"));
}

fn part1(input: &str) -> i32 {
	let mut grid = Grid::parse(input);
	let mut t = 0;
	while !grid.has_reached_exit() {
		t += 1;
		grid.step();
	}
	t
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input2"), 18);
}

fn part2(input: &str) -> i32 {
	let mut grid = Grid::parse(input);
	let mut t = 0;
	while !grid.has_reached_exit() {
		t += 1;
		grid.step();
	}
	grid.continue_from_exit();
	while !grid.has_reached_start() {
		t += 1;
		grid.step();
	}
	grid.continue_from_start();
	while !grid.has_reached_exit() {
		t += 1;
		grid.step();
	}
	t
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input2"), 54);
}

struct Grid {
	walls: [u128; MAX_ROWS],
	wind_north: [u128; MAX_ROWS],
	wind_south: [u128; MAX_ROWS],
	wind_west: [u128; MAX_ROWS],
	wind_east: [u128; MAX_ROWS],
	presence: [u128; MAX_ROWS],
	width: usize,
	height: usize,
}

impl Grid {
	fn parse(input: &str) -> Grid {
		let mut width = 0;
		let mut height = 0;
		let mut walls = [0u128; MAX_ROWS];
		let mut wind_north = [0u128; MAX_ROWS];
		let mut wind_south = [0u128; MAX_ROWS];
		let mut wind_west = [0u128; MAX_ROWS];
		let mut wind_east = [0u128; MAX_ROWS];
		let mut presence = [0u128; MAX_ROWS];

        let contents = fs::read_to_string(input).unwrap();
		for line in contents.split_terminator("\n") {
			width = line.len();

            for (c, glyph) in line.bytes().enumerate() {
				match glyph {
					b'#' => walls[height] |= 1 << c,
					b'.' => (),
					b'^' => wind_north[height] |= 1 << c,
					b'v' => wind_south[height] |= 1 << c,
					b'<' => wind_west[height] |= 1 << c,
					b'>' => wind_east[height] |= 1 << c,
					_ => unreachable!(),
				}
			}
			height += 1;
		}
		presence[0] = 1 << 1;

        Grid {
			presence,
			walls,
			wind_north,
			wind_south,
			wind_west,
			wind_east,
			width,
			height,
		}
	}

	fn has_reached_start(&self) -> bool {
		(self.presence[0] & (1 << 1)) != 0
	}

	fn has_reached_exit(&self) -> bool {
		(self.presence[self.height - 1] & (1 << (self.width - 2))) != 0
	}

	fn continue_from_start(&mut self) {
		self.presence.fill(0);
		self.presence[0] |= 1 << 1;
	}

	fn continue_from_exit(&mut self) {
		self.presence.fill(0);
		self.presence[self.height - 1] |= 1 << (self.width - 2);
	}

	fn step(&mut self) {
		self.wind_north[1..(self.height - 1)].rotate_left(1);
		self.wind_south[1..(self.height - 1)].rotate_right(1);

        for r in 1..(self.height - 1) {
			self.wind_west[r] = blow_l(self.wind_west[r], self.walls[r]);
			self.wind_east[r] = blow_m(self.wind_east[r], self.walls[r]);
		}

		let mut above = 0;
		for r in 0..self.height {
			let current = self.presence[r];
			self.presence[r] |= above | (current >> 1) | (current << 1);
			if r + 1 < self.height {
				self.presence[r] |= self.presence[r + 1];
			}
			above = current;
			// The example makes it clear that you can brave the storm by
			// moving right through it, as long as you don't end up in part1.
			let obstacle =
				self.walls[r]
					| self.wind_north[r] | self.wind_south[r]
					| self.wind_west[r] | self.wind_east[r];
			self.presence[r] &= !obstacle;
		}
	}
}

fn blow_l(before: u128, walls: u128) -> u128 {
	let wind = before >> 1;
	if wind & walls != 0 {
		wind | (walls >> 1)
	} else {
		wind
	}
}

fn blow_m(before: u128, walls: u128) -> u128 {
	let wind = before << 1;
	if wind & walls != 0 {
		wind | (1 << 1)
	} else {
		wind
	}
}

