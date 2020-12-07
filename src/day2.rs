use nom::IResult;
use nom::bytes::complete::{take, take_while1};
use nom::character::complete::char;
use nom::character::{is_digit, is_alphanumeric};
use nom::sequence::tuple;

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



#[aoc_generator(day2)]
fn day2_gen(input: &str) -> Vec<Passwd> {
	input.split('\n').map(|x| passwd_line(x.as_bytes()).unwrap().1).collect()
}

#[aoc(day2, part1)]
fn part1(passwds: &[Passwd]) -> usize {
	let mut valid = 0;
	for passwd in passwds {
		if passwd.is_p1_valid() {
			valid += 1;
		}
	}
	valid
}

#[aoc(day2, part2)]
fn part2(passwds: &[Passwd]) -> usize {
	let mut valid = 0;
	for passwd in passwds {
		if passwd.is_p2_valid() {
			valid += 1;
		}
	}
	valid
}

#[cfg(test)]
mod tests {
	use super::*;

	static EXAMPLE: &'static str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

	lazy_static! {
		static ref INPUT: &'static str = {
			include_str!("../input/2020/day2.txt").trim_end_matches('\n')
		};
	}

	#[test]
	fn part_1() {
		assert_eq!(part1(&day2_gen(EXAMPLE)), 2);
	}
	
	#[test]
	fn part_2() {
		assert_eq!(part2(&day2_gen(EXAMPLE)), 1);
	}

	#[test]
	fn real_answers() {
		let input = day2_gen(&INPUT);
		assert_eq!(part1(&input), 640);
		assert_eq!(part2(&input), 472);
	}
}
