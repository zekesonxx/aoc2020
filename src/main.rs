//extern crate nom;
extern crate rayon;

//use nom::IResult;
//use nom::bytes::complete::{take, take_while1};
//use nom::character::complete::char;
//use nom::character::{is_digit, is_alphanumeric};
//use nom::sequence::tuple;

//use rayon::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
//use std::str;
//use std::env;
//use std::convert::TryInto;

fn main() -> std::io::Result<()> {
	//let argv: Vec<String> = env::args().collect();
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}
	//let mut lines: Vec<&str> = input.split('\n').collect();
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
	println!("{} groups", total_groups);
	//lines.retain(|&x| x.len() != 0);
	Ok(())
}
