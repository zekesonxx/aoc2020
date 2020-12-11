
#[derive(PartialEq, Eq, Clone, Copy)]
enum Cell {
	Floor,
	Seat,
	OccupiedSeat
}

use Cell::*;

#[aoc_generator(day11)]
fn day11_gen(input: &str) -> Vec<Vec<Cell>> {
	let mut grid = vec![];
	for line in input.lines() {
		grid.push(line.chars().map(|c| match c {
				'.' => Some(Floor),
				'L' => Some(Seat),
				'#' => Some(OccupiedSeat),
				_ => unreachable!("malformed input")
			}).flatten().collect());
	}
	grid
}

	
fn at(grid: &Vec<Vec<Cell>>, y: usize, x: usize) -> Cell {
	if let Some(row) = grid.get(y) {
		if let Some(c) = row.get(x) {
			return *c;
		}
	}
	Floor
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Vec<Cell>>) -> usize {
	let mut grid = input.clone();
	loop {
		for row in grid.iter() {
			for c in row.iter() {
				print!("{}", match c {
					Floor => '.',
					Seat => 'L',
					OccupiedSeat => '#'
				});
			}
			println!();
		}
		println!();
		
		
		let lastgrid = grid.clone();
		let mut changed = false;
		
		for y in 0..grid.len() {
			for x in 0..grid[0].len() {
				if grid[y][x] == Floor {
					continue;
				}
				let mut adajcent = 0;
				if at(&lastgrid, y-1, x) == OccupiedSeat {
					adajcent += 1;
				}
				if at(&lastgrid, y-1, x-1) == OccupiedSeat {
					adajcent += 1;
				}
				if at(&lastgrid, y-1, x+1) == OccupiedSeat {
					adajcent += 1;
				}
				if at(&lastgrid, y, x+1) == OccupiedSeat {
					adajcent += 1;
				}
				if at(&lastgrid, y, x-1) == OccupiedSeat {
					adajcent += 1;
				}
				if at(&lastgrid, y+1, x) == OccupiedSeat {
					adajcent += 1;
				}
				if at(&lastgrid, y+1, x-1) == OccupiedSeat {
					adajcent += 1;
				}
				if at(&lastgrid, y+1, x+1) == OccupiedSeat {
					adajcent += 1;
				}
				if adajcent == 0 {
					grid[y][x] = OccupiedSeat;
				} else if adajcent >= 4 {
					grid[y][x] = Seat;
				}
				if lastgrid[y][x] != grid[y][x] {
					changed = true;
				}
			}
		}
		
		if !changed {
			break;
		}
	}
	grid.iter().map(|row| row.iter().map(|c| if *c == OccupiedSeat {1}else{0}).sum::<usize>()).sum()
}
