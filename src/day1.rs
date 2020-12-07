#[aoc_generator(day1)]
pub fn day1_gen(input: &str) -> Vec<usize> {
	input.split('\n').map(|x| x.parse().unwrap_or(0)).collect()
}

#[aoc(day1, part1)]
fn part1(nums: &[usize]) -> usize {
	for i in nums {
		for j in nums {
			if i+j == 2020 {
				return i*j;
			}
		}
	}
	unreachable!("malformed input");
}

#[aoc(day1, part2)]
fn part2(nums: &[usize]) -> usize {
	for num in nums {
		for i in nums {
			for j in nums {
				if i+j+num == 2020 {
					return num*i*j;
				}
			}
		}
	}
	unreachable!()
}


#[cfg(test)]
mod tests {
	use super::{day1_gen, part1, part2};

	#[test]
	fn part_1() {
		assert_eq!(part1(&day1_gen("1721\n979\n366\n299\n675\n1456")), 514579);
	}
	
	#[test]
	fn part_2() {
		assert_eq!(part2(&day1_gen("1721\n979\n366\n299\n675\n1456")), 241861950);
	}

	#[test]
	fn real_answers() {
		let input = day1_gen(include_str!("../input/2020/day1.txt").trim_end_matches('\n'));
		assert_eq!(part1(&input), 633216);
		assert_eq!(part2(&input), 68348924);
	}
}
