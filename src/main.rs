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

fn main() -> std::io::Result<()> {
	//let argv: Vec<String> = env::args().collect();
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}
	//let mut lines: Vec<&str> = input.split('\n').collect();
	let mut groups: Vec<Vec<char>> = vec![];
	let mut temp: Vec<char> = vec![];
	for line in input.split('\n') {
		if line.len() == 0 {
			groups.push(temp);
			temp = vec![];
		} else {
			for c in line.chars() {
				match temp.binary_search(&c) {
					Ok(x) => {},
					Err(x) => {temp.insert(x, c)}
				}
			}
		}
	}
	let total: usize = groups.iter().map(|x| x.len()).sum();
	println!("{} groups", total);
	//lines.retain(|&x| x.len() != 0);
	Ok(())
}
