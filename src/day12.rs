
enum Direction {
	North,
	South,
	East,
	West
}
use Direction::*;

impl Direction {
	fn right(&self) -> Direction {
		match *self {
			North => East,
			East => South,
			South => West,
			West => North
		}
	}
	fn left(&self) -> Direction {
		match *self {
			North => West,
			West=> South,
			South => East,
			East => North
		}
	}
}

#[aoc(day12, part1)]
fn part1(input: &str) -> isize {
	let commands: Vec<(char, isize)> = input.split('\n')
	.map(|x| {
		(x.chars().next().unwrap(), x[1..x.len()].parse().unwrap())
	}).collect();
	let mut x = 0;
	let mut y = 0;
	let mut direction = East;
	for cmd in commands {
		match cmd {
			('N', v) => x += v,
			('S', v) => x -= v,
			('E', v) => y += v,
			('W', v) => y -= v,
			('F', v) => match direction {
				North => x += v,
				South => x -= v,
				East => y += v,
				West => y -= v,
				
			},
			('L', mut v) => while v > 0 {
				direction = direction.left();
				v -= 90;
			},
			('R', mut v) => while v > 0 {
				direction = direction.right();
				v -= 90;
			},
			_ => unreachable!("malformed input")
		}
		println!("cmd: {:?}, position: {} {} {} {}", cmd,
		y, if y > 0 { "east" } else {"west"},
		x, if x > 0 { "north" } else {"south"}
		);
	}
	x.abs()+y.abs()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> isize {
	let commands: Vec<(char, isize)> = input.split('\n')
	.map(|x| {
		(x.chars().next().unwrap(), x[1..x.len()].parse().unwrap())
	}).collect();
	let mut x = 0;
	let mut y = 0;
	let mut wx = 1;
	let mut wy = 10;
	for cmd in commands {
		match cmd {
			('N', v) => wx += v,
			('S', v) => wx -= v,
			('E', v) => wy += v,
			('W', v) => wy -= v,
			('F', v) => {
				x += v*wx;
				y += v*wy;
			},
			('L', mut v) => while v > 0 {
				let (owx, owy) = (wx, wy);
				wx = owy;
				wy = 0-owx;
				v -= 90;
			},
			('R', mut v) => while v > 0 {
				let (owx, owy) = (wx, wy);
				wx = 0-owy;
				wy = owx;
				v -= 90;
			},
			_ => unreachable!("malformed input")
		}
		println!("cmd: {:?}, position: {} {} {} {}, waypoint: {} {} {} {}", cmd,
		y, if y > 0 { "east" } else {"west"},
		x, if x > 0 { "north" } else {"south"},
		wy, if wy > 0 { "east" } else {"west"},
		wx, if wx > 0 { "north" } else {"south"},
		);
	}
	x.abs()+y.abs()
}
