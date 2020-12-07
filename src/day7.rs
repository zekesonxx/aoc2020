use nom::IResult;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::sequence::*;
use nom::multi::*;
use nom::combinator::*;
use nom::character::*;
use nom::branch::*;

use std::collections::HashMap;

use rayon::prelude::*;

fn decimal(input: &str) -> IResult<&str, usize> {
  let (input, u) = recognize(
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


#[aoc(day7, part1, orig)]
fn part1(input: &str) -> usize {
	let mut bags: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
	for line in input.split('\n') {
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
	shiny
}

#[aoc(day7, part1, orig_par)]
fn part1_par(input: &str) -> usize {
	let mut bags: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
	for line in input.split('\n') {
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
	bags.keys().par_bridge().map(|&bag| if has_shiny(&bags, bag) {
			1usize
		} else { 0 }).sum::<usize>()
	
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
	let mut bags: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
	for line in input.split('\n') {
		let k = bag_line(line).unwrap().1;
		bags.insert(k.0, k.1);
	}
	fn bag_len(bags: &HashMap<&str, Vec<(usize, &str)>>, x: &str) -> usize {
		let mut size = 1;
		let k = bags.get(x).unwrap();
		for sub in k {
			size += sub.0*bag_len(bags, sub.1);
		}
		size
	}
	bag_len(&bags, "shiny gold")-1
}

#[cfg(test)]
mod tests {
	use super::*;

	static EXAMPLE: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
	lazy_static! {
		static ref INPUT: &'static str = {
			include_str!("../input/2020/day7.txt").trim_end_matches('\n')
		};
	}

	#[test]
	fn part_1() {
		assert_eq!(part1(&EXAMPLE), 4);
	}
	
	#[test]
	fn part_2() {
		assert_eq!(part2(&EXAMPLE), 32);
	}

	#[test]
	fn real_answers() {
		assert_eq!(part1(&INPUT), 119);
		assert_eq!(part2(&INPUT), 155802);
	}
}
