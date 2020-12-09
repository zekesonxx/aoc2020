use std::collections::VecDeque;


const PREAMBLE: usize = 25;

fn can_add_2(nums: &[usize], search: usize) -> bool {
	for i in nums {
		for j in nums {
			if i+j == search {
				return true;
			}
		}
	}
	false
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
	let nums: Vec<usize> = input.split('\n').map(|x| x.parse()).flatten().collect();
	let mut preamble = VecDeque::with_capacity(PREAMBLE+1);
	for num in nums {
		if preamble.len() < PREAMBLE {
			//initial preamble
			preamble.push_back(num);
		} else {
			//in the real thing
			preamble.make_contiguous();
			if can_add_2(preamble.as_slices().0, num) {
				preamble.pop_front();
				preamble.push_back(num);
			} else {
				return num;
			}
		}
	}
	unreachable!("unsolvable input");
}
