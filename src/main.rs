//extern crate nom;
extern crate rayon;

//use nom::IResult;
//use nom::bytes::complete::{take, take_while1};
//use nom::character::complete::char;
//use nom::character::{is_digit, is_alphanumeric};
//use nom::sequence::tuple;

use rayon::prelude::*;


use std::fs::File;
use std::io::prelude::*;
//use std::str;
//use std::env;
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
			println!("min={}, max={}, half={}", min, max, half);
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

fn main() -> std::io::Result<()> {
	//let argv: Vec<String> = env::args().collect();
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}
	let mut lines: Vec<&str> = input.split('\n').collect();
	lines.retain(|&x| x.len() != 0);
	let passes: Vec<BinaryPass> = lines.par_iter().map(|x| BinaryPass::new(x)).flatten().collect();
	let highest = passes.par_iter().map(|x| x.seat_id()).max().unwrap();
	println!("highest: {}", highest);

	Ok(())
}
