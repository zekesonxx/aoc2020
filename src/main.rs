extern crate nom;

//use nom::IResult;
//use nom::bytes::complete::{take, take_while1};
//use nom::character::complete::char;
//use nom::character::{is_digit, is_alphanumeric};
//use nom::sequence::tuple;

use std::fs::File;
use std::io::prelude::*;
//use std::str;
use std::env;

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
			print!("{}", self.c(x, y));
			if self.c(x, y) == '#' {
				trees += 1;
			}
			y += down;
			x += right;
		}
		println!();
		trees
	}
}


fn main() -> std::io::Result<()> {
	let argv: Vec<String> = env::args().collect();
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}
	let mut lines: Vec<&str> = input.split('\n').collect();
	lines.retain(|&x| x.len() != 0);
	let map = Map::new(lines);
	let down = argv.get(1).unwrap_or(&"".to_string()).parse().unwrap_or(0);
	let right = argv.get(2).unwrap_or(&"".to_string()).parse().unwrap_or(0);
	println!("{:?}, w={}, h={}", map, map.width(), map.height());
	println!("{} down and {} right hits {} trees", down, right, map.trees_encountered(down, right));
	Ok(())
}
