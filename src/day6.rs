use std::collections::HashMap;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
	let mut groups: Vec<Vec<char>> = vec![];
	let mut temp: Vec<char> = vec![];
	for line in input.split('\n') {
		if line.len() == 0 {
			groups.push(temp);
			temp = vec![];
		} else {
			for c in line.chars() {
				match temp.binary_search(&c) {
					Ok(_) => {},
					Err(x) => {temp.insert(x, c)}
				}
			}
		}
	}
	groups.push(temp);
	groups.iter().map(|x| x.len()).sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
	let mut total_groups = 0;
	let mut chars: HashMap<char, usize> = HashMap::new();
	let mut people = 0usize;
	for line in input.split('\n') {
		if line.len() == 0 {
			chars.retain(|_, v| *v == people);
			total_groups += chars.len();
			//reset
			people = 0;
			chars.retain(|_, _| false);
		} else {
			for c in line.chars() {
				let i = chars.get(&c).unwrap_or(&0).clone();
				chars.insert(c, i+1);
			}
			people += 1;
		}
	}
	chars.retain(|_, v| *v == people);
	total_groups += chars.len();
	total_groups
}

#[cfg(test)]
mod tests {
	use super::*;

	static EXAMPLE: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

	lazy_static! {
		static ref INPUT: &'static str = {
			include_str!("../input/2020/day6.txt").trim_end_matches('\n')
		};
	}

	#[test]
	fn part_1() {
		assert_eq!(part1(&EXAMPLE), 11);
	}
	
	#[test]
	fn part_2() {
		assert_eq!(part2(&EXAMPLE), 6);
	}

	#[test]
	fn real_answers() {
		assert_eq!(part1(&INPUT), 6443);
		assert_eq!(part2(&INPUT), 3232);
	}
}
