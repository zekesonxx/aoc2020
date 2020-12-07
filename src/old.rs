extern crate nom;
extern crate rayon;

use nom::IResult;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::sequence::*;
use nom::multi::*;
use nom::combinator::*;
use nom::character::*;
use nom::branch::*;

//use rayon::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
//use std::str;
//use std::env;
//use std::convert::TryInto;

fn decimal(input: &str) -> IResult<&str, usize> {
  let (input, (u)) = recognize(
    many1(
      terminated(one_of("0123456789"), many0(char('_')))
    )
  )(input)?;
  Ok((input, u.parse().unwrap()))
}

fn a_bag(input: &str) -> IResult<&str, (usize, &str)> {
	let (input, (q, _, name, _, _, _)) = tuple((
		decimal,
		multispace1,
		take_until(" bag"),
		alt((tag(" bags"), tag(" bag"))),
		alt((tag(","), tag("."))),
		multispace0
	))(input)?;
	//println!("{}", input);
	Ok((input, (q, name)))
}

fn bag_line(input: &str) -> IResult<&str, (&str, Vec<(usize, &str)>)> {
	let (input, (bagname, _)) = tuple((
		take_until(" bags"),
		tag(" bags contain ")
	))(input)?;
	//println!("{:?}", bagname);
	//println!("{:?}", input);
	if input == "no other bags." {
		Ok((input, (bagname, vec![])))
	} else {
		let (input, subbags) = many1(a_bag)(input)?;
		Ok((input, (bagname, subbags) ))
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
	let mut bags: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
	for line in lines {
		let k = bag_line(line).unwrap().1;
		bags.insert(k.0, k.1);
	}
	fn has_shiny(bags: &HashMap<&str, Vec<(usize, &str)>>, x: &str) -> bool {
		let k = bags.get(x).unwrap();
		for sub in k {
			if sub.1 == "shiny gold" {
				return true;
			} else if has_shiny(bags, sub.1) {
				return true;
			}
		}
		return false;
	}
	let mut shiny = 0;
	for bag in bags.keys() {
		if has_shiny(&bags, &bag) {
			shiny += 1;
		}
	}
	println!("shinys: {}", shiny);

	fn bag_len(bags: &HashMap<&str, Vec<(usize, &str)>>, x: &str) -> usize {
		let mut size = 1;
		let k = bags.get(x).unwrap();
		for sub in k {
			size += sub.0*bag_len(bags, sub.1);
		}
		size
	}
	println!("shiny contains {} bags", bag_len(&bags, "shiny gold")-1);

	Ok(())
}
