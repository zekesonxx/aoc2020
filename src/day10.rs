use itertools::Itertools;

#[aoc_generator(day10)]
fn day10_gen(input: &str) -> Vec<usize> {
	let mut jolts: Vec<usize> = input.split('\n').map(|x| x.parse()).flatten().collect();
	jolts.sort();
	let highest = jolts.iter().max().unwrap();
	let device = highest+3;
	jolts.push(device);
	jolts
}

#[aoc(day10, part1)]
fn part1(jolts: &[usize]) -> usize {
	let mut lastjolt = 0;
	let mut diff1 = 0;
	let mut diff3 = 0;
	for adapter in jolts {
		let diff = adapter-lastjolt;
		match diff {
			1 => diff1 += 1,
			2 => {},
			3 => diff3 += 1,
			_ => {}
		}
		//println!("jump {} to {} (diff {})", lastjolt, adapter, diff);
		lastjolt = *adapter;
	}
	//println!("{} 1 jumps, {} 3 jumps", diff1, diff3);
	diff1*diff3
}

fn adapter_chain(jolts: &[usize], skips: &[&usize]) -> bool {
	let mut lastjolt: usize = 0;
	let highest = **(skips.get(skips.len()-1).unwrap_or(&&usize::MAX));
	for adapter in jolts {
		if skips.binary_search(&adapter).is_ok() {
			continue;
		}
		let diff = adapter-lastjolt;
		if diff > 3 {
			return false;
		}
		if *adapter >= highest {
			return true;
		}
		lastjolt = *adapter;
	}
	true
}



#[aoc(day10, part2)]
fn part2_try2(jolts: &[usize]) -> usize {
	let mut pockets: Vec<Vec<usize>> = vec![];
	let mut pocket: Vec<usize> = vec![];
	let mut v = jolts.iter().peekable();
	let mut lastjolt = 0;
	while let Some(jolt) = v.next() {
		if let Some(peek) = v.peek() {
			if (*peek)-lastjolt <= 3 {
				pocket.push(*jolt);
			} else {
				if !pocket.is_empty() {
					//we just exited a pocket
					pockets.push(pocket);
					pocket = vec![];
				}
			}
		} 
		lastjolt = *jolt;
	}
	let mut combinations = 1;
	for pocket in &pockets {
		if pocket.len() == 1 {
			combinations *= 2
		} else {
			let mut total = 1;
			for i in 1..pocket.len()+1 {
				total += pocket.iter().combinations(i)
				.map(|x| {
					if adapter_chain(jolts, x.as_slice()) {
						1
					} else {
						0
					}
				}).sum::<usize>();
			}
			combinations *= total;
		}
	}
	combinations
}










