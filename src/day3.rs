#[derive(Clone, Debug)]
struct Map {
	//
	// pattern[y][x]
	//   x ->
	//  y..##..
	//  |#...#.
	// \/.#....
	pub pattern: Vec<Vec<char>>
}

impl Map {
	fn new(input: Vec<&str>) -> Self {
		let mut pattern = vec![];
		for line in input {
			pattern.push(line.chars().collect());
		}
		Map {
			pattern
		}
	}
	fn width(&self) -> usize {
		self.pattern[0].len()
	}
	fn height(&self) -> usize {
		self.pattern.len()
	}
	fn c(&self, x: usize, y: usize) -> char {
		self.pattern[y][x%self.width()]
	}
	fn trees_encountered(&self, right: usize, down: usize) -> usize {
		assert!(right > 0, "right must be greater than 0");
		assert!(down > 0, "down must be greater than 0");
		let mut x = right;
		let mut y = down;
		let mut trees = 0;
		while y < self.height() {
			if self.c(x, y) == '#' {
				trees += 1;
			}
			y += down;
			x += right;
		}
		trees
	}
}


#[aoc_generator(day3)]
fn day3_gen(input: &str) -> Map {
	Map::new(input.split('\n').collect())
}

#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
	map.trees_encountered(3, 1)
}

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
	map.trees_encountered(1, 1)
	* map.trees_encountered(3, 1)
	* map.trees_encountered(5, 1)
	* map.trees_encountered(7, 1)
	* map.trees_encountered(1, 2)
}

/*
#[aoc(day2, part2)]
fn part2(passwds: &[Passwd]) -> usize {
	let mut valid = 0;
	for passwd in passwds {
		if passwd.is_p2_valid() {
			valid += 1;
		}
	}
	valid
}*/

#[cfg(test)]
mod tests {
	use super::*;

	static EXAMPLE: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

	lazy_static! {
		static ref INPUT: &'static str = {
			include_str!("../input/2020/day3.txt").trim_end_matches('\n')
		};
	}

	#[test]
	fn part_1() {
		assert_eq!(part1(&day3_gen(EXAMPLE)), 7);
	}
	
	#[test]
	fn part_2() {
		assert_eq!(part2(&day3_gen(EXAMPLE)), 336);
	}

	#[test]
	fn real_answers() {
		assert_eq!(part1(&day3_gen(&INPUT)), 211);
		assert_eq!(part2(&day3_gen(&INPUT)), 3584591857);
	}
}
