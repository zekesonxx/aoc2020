use std::collections::HashMap;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
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
	passports.push(temp);
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
	valid
}


#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
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
	passports.push(temp);
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
			continue;
		}
		let iyrraw = passport.get("iyr").unwrap();
		let iyr = iyrraw.parse::<usize>().unwrap();
		if iyr < 2010 || iyr > 2020 || iyrraw.len() != 4 {
			continue;
		}
		let eyrraw = passport.get("eyr").unwrap();
		let eyr = eyrraw.parse::<usize>().unwrap();
		if eyr < 2020 || eyr > 2030 || eyrraw.len() != 4 {
			continue;
		}

		let hgtraw = passport.get("hgt").unwrap();
		if hgtraw.len() != 5 && hgtraw.len() != 4 {	
			continue;
		}
		let hgt = hgtraw[..hgtraw.len()-2].parse::<usize>().unwrap_or_default();
		let hgtunit = hgtraw.char_indices().rev().map(|(i, _)| i).nth(1).unwrap();
		let hgtunit = &hgtraw[hgtunit..];
		if (hgtunit == "cm" && (hgt < 150 || hgt > 193)) ||
		   (hgtunit == "in" && (hgt < 59 || hgt > 76)) {
			continue;
		}

		let mut validhcl = true;
		let hcl = passport.get("hcl").unwrap();
		if hcl.len() != 7 {
		}
		for c in hcl.char_indices() {
			match c.1 {
				'#' => if c.0 != 0 { validhcl = false },
				'0'..='9' | 'a'..='f' => if c.0 == 0 { validhcl = false },
				_ => validhcl=false
			}
		}
		if !validhcl {
			continue;
		}
		match *passport.get("ecl").unwrap() {
			"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
			_ => {
				continue;
			}
		}
		let mut pid: Vec<char> = passport.get("pid").unwrap().chars().collect();
		pid.retain(|i| ('0'..='9').contains(i));
		if pid.len() != 9 {
			continue;
		}
		valid += 1;
	}
	valid
}


#[cfg(test)]
mod tests {
	use super::*;

	static EXAMPLE1: &'static str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

	static EXAMPLE2: &'static str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
	lazy_static! {
		static ref INPUT: &'static str = {
			include_str!("../input/2020/day4.txt").trim_end_matches('\n')
		};
	}

	#[test]
	fn part_1() {
		assert_eq!(part1(EXAMPLE1), 2);
	}
	
	#[test]
	fn part_2() {
		assert_eq!(part2(EXAMPLE2), 4);
	}

	#[test]
	fn real_answers() {
		assert_eq!(part1(&INPUT), 235);
		assert_eq!(part2(&INPUT), 194);
	}
}
