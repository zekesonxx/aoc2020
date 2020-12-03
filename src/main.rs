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
	min: u8,
	max: u8,
	passwd: String
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
	for line in &lines {
		println!("{:?}", passwd_line(line.as_bytes()).unwrap().1);
		println!("{}", line);
	}
	Ok(())
}
