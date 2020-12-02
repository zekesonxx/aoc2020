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
			for j in &nums {
				if i+j+num == 2020 {
					println!("{a}+{b}+{c}=2020, {a}x{b}x{c}={abc}", 
						a=num, b=i, c=j, abc=num*i*j);
				}
			}
		}
	}
	Ok(())
}
