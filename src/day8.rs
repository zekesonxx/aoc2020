use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Instruction {
	Nop(isize),
	Acc(isize),
	Jmp(isize)
}

fn cpu(ops: &[Instruction], swap: Option<usize>) -> Result<isize, isize> {
	let mut visited = vec![false; ops.len()];
	let mut ptr = 0;
	let mut acc = 0;
	let swap = swap.unwrap_or(usize::MAX);
	while ptr < ops.len() && visited[ptr] != true {
		visited[ptr] = true;
		use Instruction::*;
		match ops[ptr] {
			Nop(x) => if ptr == swap {
				ptr = usize::try_from(isize::try_from(ptr).unwrap() + x).unwrap()
			} else { ptr += 1; },
			Acc(x) => { acc += x; ptr += 1; },
			Jmp(x) => if ptr != swap {
				ptr = usize::try_from(isize::try_from(ptr).unwrap() + x).unwrap()
			} else { ptr += 1; },
		}
	}
	if ptr==ops.len() {
		Ok(acc)
	} else {
		Err(acc)
	}
}

use std::convert::TryFrom;

#[aoc_generator(day8)]
fn day_gen(input: &str) -> Vec<Instruction> {
	input.split('\n').map(|line| {
		let line: Vec<&str> = line.split(' ').collect();
		let cmd = line[0];
		let i = line[1].parse::<isize>().unwrap();
		match cmd {
			"nop" => Instruction::Nop(i),
			"acc" => Instruction::Acc(i),
			"jmp" => Instruction::Jmp(i),
			_ => unreachable!("invalid input")
		}
	}).collect()
}

#[aoc(day8, part1)]
fn part1(ops: &[Instruction]) -> isize {
	cpu(ops, None).unwrap_err()
}

#[aoc(day8, part2)]
fn part2(ops: &[Instruction]) -> isize {
	ops.par_iter().enumerate().map(|x| {
		use Instruction::*;
		match x.1 {
			Nop(_) | Jmp(_) => Some(x.0),
			_ => None
		}
	}).flatten().find_map_any(|x| {
		cpu(ops, Some(x)).ok()
	}).unwrap()
}
