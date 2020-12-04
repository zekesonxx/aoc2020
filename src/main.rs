//extern crate nom;

//use nom::IResult;
//use nom::bytes::complete::{take, take_while1};
//use nom::character::complete::char;
//use nom::character::{is_digit, is_alphanumeric};
//use nom::sequence::tuple;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
//use std::str;
use std::env;

fn main() -> std::io::Result<()> {
	let argv: Vec<String> = env::args().collect();
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}
	//let mut lines: Vec<&str> = input.split('\n').collect();
	let mut passports: Vec<HashMap<&str, &str>> = vec![];
	let mut temp: HashMap<&str, &str> = HashMap::new();
	for line in input.split('\n') {
		if line.len() == 0 {
			passports.push(temp);
			temp = HashMap::new();
		} else {
			for item in line.split(' ') {
				let item: Vec<&str> = item.split(':').collect();
				temp.insert(item[0], item[1]);
			}
		}
	}
	let mut valid = 0;
	for passport in passports.into_iter() {
		if passport.contains_key("byr") &&
		   passport.contains_key("iyr") &&
		   passport.contains_key("eyr") &&
		   passport.contains_key("hgt") &&
		   passport.contains_key("hcl") &&
		   passport.contains_key("ecl") &&
		   passport.contains_key("pid") {
		   	valid += 1
		   }
	}
	//lines.retain(|&x| x.len() != 0);
	println!("{} valid", valid);
	Ok(())
}
