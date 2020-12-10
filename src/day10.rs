
#[aoc(day10, part1)]
fn part1(input: &str) -> isize {
	let mut jolts: Vec<isize> = input.split('\n').map(|x| x.parse()).flatten().collect();
	jolts.sort();
	let highest = jolts.iter().max().unwrap();
	let device = highest+3;
	let mut lastjolt = 0;
	let mut diff1 = 0;
	let mut diff3 = 0;
	jolts.push(device);
	for adapter in jolts {
		let diff = adapter-lastjolt;
		match diff {
			1 => diff1 += 1,
			2 => {},
			3 => diff3 += 1,
			_ => {}
		}
		println!("jump {} to {} (diff {})", lastjolt, adapter, diff);
		lastjolt = adapter;
	}
	println!("{} 1 jumps, {} 3 jumps", diff1, diff3);
	diff1*diff3
}
