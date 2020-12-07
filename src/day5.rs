use rayon::prelude::*;

use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
struct BinaryPass {
	fb: [char; 7],
	lr: [char; 3]
}

impl BinaryPass {
	fn new(input: &str) -> Option<Self> {
		if input.len() != 10 {
			return None;
		}
		let chars: Vec<char> = input.chars().collect();
		Some(BinaryPass {
			fb: (&chars[0..=6]).try_into().unwrap(),
			lr: (&chars[7..=9]).try_into().unwrap()
		})
	}
	fn binary_part(input: &[char], size: usize) -> usize {
		let mut min = 0;
		let mut max = size;
		let mut half;
		for c in input {
			half = (max-min)/2;
			//println!("min={}, max={}, half={}", min, max, half);
			if *c == 'F' || *c == 'L' {
				max -= half;
			} else if *c == 'B' || *c == 'R' {
				min += half;
			} else {
				panic!("invalid char {}", c);
			}
		}
		assert_eq!(min, max-1);
		min
	}
	fn row_col(&self) -> (usize, usize) {
		(BinaryPass::binary_part(&self.fb, 128), BinaryPass::binary_part(&self.lr, 8))
	}
	fn seat_id(&self) -> usize {
		let (row, col) = self.row_col();
		(row * 8) + col
	}
}

#[aoc_generator(day5)]
fn day5_gen(input: &str) -> Vec<BinaryPass> {
	input.split('\n').map(|x| BinaryPass::new(x)).flatten().collect()
}

#[aoc(day5, part1)]
fn part1(passes: &[BinaryPass]) -> usize {
	passes.iter().map(|x| x.seat_id()).max().unwrap()
}

#[aoc(day5, part2, orig)]
fn part2(passes: &[BinaryPass]) -> usize {
	let mut seats: Vec<usize> = passes.iter().map(|x| x.seat_id()).collect();
	seats.sort();
	let lowest = seats.first().unwrap();
	let sum: usize = seats.iter().map(|x|x-lowest).sum();
	let n = seats.len();
	let missing = n * (n+1)/2 - sum;
	missing+lowest
}

#[aoc(day5, part2, orig_par)]
fn part2_par(passes: &[BinaryPass]) -> usize {
	let mut seats: Vec<usize> = passes.par_iter().map(|x| x.seat_id()).collect();
	seats.sort();
	let lowest = seats.first().unwrap();
	let sum: usize = seats.par_iter().map(|x|x-lowest).sum();
	let n = seats.len();
	let missing = n * (n+1)/2 - sum;
	missing+lowest
}

#[aoc(day5, part2, nosort)]
fn part2_nosort(passes: &[BinaryPass]) -> usize {
	let mut lowest = 999999999;
	let mut sum = 0;
	for seat in passes {
		let id = seat.seat_id();
		if id < lowest {
			lowest = id;
		}
		sum += id;
	}
	let n = passes.len();
	sum = sum-(lowest*n);
	let missing = n * (n+1)/2 - sum;
	missing+lowest
}



#[cfg(test)]
mod tests {
	use super::*;

	lazy_static! {
		static ref INPUT: &'static str = {
			include_str!("../input/2020/day5.txt").trim_end_matches('\n')
		};
	}

	#[test]
	fn pass_parser() {
		assert_eq!(BinaryPass::new("FBFBBFFRLR").unwrap().seat_id(), 357);
		assert_eq!(BinaryPass::new("BFFFBBFRRR").unwrap().seat_id(), 567);
		assert_eq!(BinaryPass::new("FFFBBBFRRR").unwrap().seat_id(), 119);
		assert_eq!(BinaryPass::new("BBFFBBFRLL").unwrap().seat_id(), 820);
	}
	
	#[test]
	fn real_answers() {
		let input = day5_gen(&INPUT);
		assert_eq!(part1(&input), 987);
		assert_eq!(part2(&input), 603);
	}
}
