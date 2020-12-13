

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
	let mut lines = input.split('\n');
	let earliest = lines.next().unwrap().parse::<usize>().unwrap();
	let line2 = lines.next().unwrap();
	let busses: Vec<usize> = line2.split(',').map(|x| x.parse::<usize>()).flatten().collect();
	
	println!("{}", earliest);
	println!("{:?}", busses);
	let mut shortest = usize::MAX;
	let mut shortestbus = 0;
	for bus in busses {
		println!("bus #{}", bus);
		let mut iter = earliest/bus;
		while iter*bus <= earliest {
			iter += 1;
		}
		println!("iter={}, iter*bus={}", iter, iter*bus);
		if iter*bus < shortest {
			println!("new shortest: {}", iter*bus);
			shortestbus = bus;
			shortest = iter*bus;
		}
	}
	shortestbus*(shortest-earliest)
}
