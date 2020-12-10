use itertools::Itertools;

use rayon::prelude::*;

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
	let mut lastjolt = 0;
	for adapter in jolts {
		if skips.contains(&adapter) {
			continue;
		}
		let diff = adapter-lastjolt;
		if diff > 3 {
			return false;
		}
		lastjolt = *adapter;
	}
	true
}

#[aoc(day10, part2)]
fn part2(jolts: &[usize]) -> usize {
	let mut removable: Vec<usize> = vec![];
	let mut v = jolts.iter().peekable();
	let mut lastjolt = 0;
	while let Some(jolt) = v.next() {
		let diff = jolt-lastjolt;
		if let Some(peek) = v.peek() {
			if (*peek)-lastjolt <= 3 {
				removable.push(*jolt);
			}
		}
		lastjolt = *jolt;
	}
	println!("removable: {:?}", removable);
	let mut possible = 1;
	for i in 1..=removable.len() {
		let combos: Vec<Vec<&usize>> = removable.iter().combinations(i).collect();
		possible += combos.par_iter()
		.map(|x| if adapter_chain(jolts, x.as_slice()) {
			//println!("can skip {:?}", x);
			1 
		} else { 
			0
		}).sum::<usize>();
	}
	possible
}
