use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input);
	}
	let nums: Vec<usize> = input.split('\n').map(|x| x.parse().unwrap_or(0)).collect();
	for num in &nums {
		for i in &nums {
			if i+num == 2020 {
				println!("{a}+{b}=2020, {a}x{b}={ab}", a=num, b=i, ab=num*i);
			}
		}
	}
	Ok(())
}
