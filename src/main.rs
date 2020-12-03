extern crate nom;

use nom::IResult;
use nom::bytes::complete::{take, take_while1};
use nom::character::complete::char;
use nom::character::{is_digit, is_alphanumeric};
use nom::sequence::tuple;

use std::fs::File;
use std::io::prelude::*;
use std::str;

#[derive(Clone, Debug)]
struct Passwd {
	check_char: char,
	min: usize,
	max: usize,
	passwd: String
}

impl Passwd {
	fn is_p1_valid(&self) -> bool {
		let mut count = 0;
		for c in self.passwd.chars() {
			if c == self.check_char {
				count += 1;
			}
		}
		count >= self.min && count <= self.max
	}
	fn is_p2_valid(&self) -> bool {
		let chars: Vec<char> = self.passwd.chars().collect();
		(chars[self.min-1] == self.check_char) ^ (chars[self.max-1] == self.check_char)
	}
}


//1-3 a: abcde
fn passwd_line(input: &[u8]) -> IResult<&[u8], Passwd> {
	let digit = take_while1(is_digit);
	let dash = char('-');
	let space = char(' ');
	let colon = char(':');
	let take1 = take(1usize);
	let password = take_while1(is_alphanumeric);
	let (input, (min, _, max, _, ch, _, _, passwd)) =
		tuple((digit, dash, take_while1(is_digit), char(' '), take1, colon, space, password))(input)?;
	let passwd = str::from_utf8(&passwd).unwrap_or("").to_string();
	Ok((input, Passwd {
		min: str::from_utf8(&min).unwrap_or("0").parse().unwrap(),
		max: str::from_utf8(&max).unwrap_or("0").parse().unwrap(),
		check_char: ch[0] as char,
		passwd: passwd
	}))
}

fn main() -> std::io::Result<()> {
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input);
	}
	let mut lines: Vec<&str> = input.split('\n').collect();
	lines.retain(|&x| x.len() != 0);
	let (mut p1valid, mut p1invalid) = (0,0);
	let (mut p2valid, mut p2invalid) = (0,0);
	for line in &lines {
		let passwd = passwd_line(line.as_bytes()).unwrap().1;
		if passwd.is_p1_valid() {
			p1valid += 1;
		} else {
			p1invalid += 1;
		}
		if passwd.is_p2_valid() {
			p2valid += 1;
		} else {
			p2invalid += 1;
		}
	}
	println!("part 1: {} valid, {} invalid", p1valid, p1invalid);
	println!("part 2: {} valid, {} invalid", p2valid, p2invalid);
	Ok(())
}
