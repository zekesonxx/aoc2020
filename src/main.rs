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
		if !(passport.contains_key("byr") &&
		   passport.contains_key("iyr") &&
		   passport.contains_key("eyr") &&
		   passport.contains_key("hgt") &&
		   passport.contains_key("hcl") &&
		   passport.contains_key("ecl") &&
		   passport.contains_key("pid")) {
		   	continue;
		}
		let byrraw = passport.get("byr").unwrap();
		let byr = byrraw.parse::<usize>().unwrap();
		if byr < 1920 || byr > 2002 || byrraw.len() != 4 {
			println!("invalid {:?} due to byr", passport);
			continue;
		}
		let iyrraw = passport.get("iyr").unwrap();
		let iyr = iyrraw.parse::<usize>().unwrap();
		if iyr < 2010 || iyr > 2020 || iyrraw.len() != 4 {
			println!("invalid {:?} due to iyr", passport);
			continue;
		}
		let eyrraw = passport.get("eyr").unwrap();
		let eyr = eyrraw.parse::<usize>().unwrap();
		if eyr < 2020 || eyr > 2030 || eyrraw.len() != 4 {
			println!("invalid {:?} due to eyr", passport);
			continue;
		}

		let hgtraw = passport.get("hgt").unwrap();
		if hgtraw.len() != 5 && hgtraw.len() != 4 {	
			println!("invalid {:?} due to height", passport);
			continue;
		}
		let hgt = hgtraw[..hgtraw.len()-2].parse::<usize>().unwrap_or_default();
		let hgtunit = hgtraw.char_indices().rev().map(|(i, _)| i).nth(1).unwrap();
		let hgtunit = &hgtraw[hgtunit..];
		println!("grr {} {}", hgt, hgtunit);
		if (hgtunit == "cm" && (hgt < 150 || hgt > 193)) ||
		   (hgtunit == "in" && (hgt < 59 || hgt > 76)) {
			println!("invalid {:?} due to height", passport);
			continue;
		}

		let mut validhcl = true;
		let hcl = passport.get("hcl").unwrap();
		if hcl.len() != 7 {
			println!("invalid {:?} due to hcl", passport);
			continue;
		}
		for c in hcl.char_indices() {
			match c.1 {
				'#' => if c.0 != 0 { validhcl = false },
				'0'..='9' | 'a'..='f' => if c.0 == 0 { validhcl = false },
				_ => validhcl=false
			}
		}
		if !validhcl {
			println!("invalid {:?} due to hcl", passport);
			continue;
		}
		match *passport.get("ecl").unwrap() {
			"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
			_ => {
		   		println!("invalid {:?} due to ecl", passport);
				continue;
			}
		}
		let mut pid: Vec<char> = passport.get("pid").unwrap().chars().collect();
		pid.retain(|i| ('0'..='9').contains(i));
		if pid.len() != 9 {
			println!("invalid {:?} due to pid", passport);
			continue;
		}
		println!("valid {:?}", passport);
		valid += 1;
	}
	//lines.retain(|&x| x.len() != 0);
	println!("{} valid", valid);
	Ok(())
}
