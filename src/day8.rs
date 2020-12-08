#[derive(Debug, Clone, Copy)]
enum Instruction {
	Nop,
	Acc(isize),
	Jmp(isize)
}

use std::convert::TryFrom;

#[aoc_generator(day8)]
fn day_gen(input: &str) -> Vec<Instruction> {
	input.split('\n').map(|line| {
		let line: Vec<&str> = line.split(' ').collect();
		let cmd = line[0];
		let i = line[1].parse::<isize>().unwrap();
		match cmd {
			"nop" => Instruction::Nop,
			"acc" => Instruction::Acc(i),
			"jmp" => Instruction::Jmp(i),
			_ => unreachable!("invalid input")
		}
	}).collect()
}

#[aoc(day8, part1)]
fn part1(ops: &[Instruction]) -> isize {
	let mut visited = vec![false; ops.len()];
	let mut ptr = 0;
	let mut acc = 0;
	while visited[ptr] != true {
		visited[ptr] = true;
		println!("{:?} ptr={} acc={}", ops[ptr], ptr, acc);
		use Instruction::*;
		match ops[ptr] {
			Nop => { ptr += 1; },
			Acc(x) => { acc += x; ptr += 1; },
			Jmp(x) => ptr = usize::try_from(isize::try_from(ptr).unwrap() + x).unwrap()
		}
		println!("ptr={} acc={}", ptr, acc);
	}
	acc
}
