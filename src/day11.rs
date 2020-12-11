use std::convert::TryFrom;


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

fn search_at(grid: &Vec<Vec<Cell>>, cy: usize, cx: usize, searchy: isize, searchx: isize) -> usize {
	let mut distance = 1;
	let cy = isize::try_from(cy).unwrap();
	let cx = isize::try_from(cx).unwrap();
	let rows = isize::try_from(grid.len()).unwrap();
	let cols = isize::try_from(grid[0].len()).unwrap();
	let yrange = 0..rows;
	let xrange = 0..cols;
	loop {
		let y = cy + (searchy*distance);
		let x = cx + (searchx*distance);
		if !(yrange.contains(&y) && xrange.contains(&x)) {
			return 0;
		}
		
		if let Some(row) = grid.get(usize::try_from(y).unwrap_or(0)) {
			if let Some(c) = row.get(usize::try_from(x).unwrap_or(0)) {
				if *c == OccupiedSeat {
					return 1;
				} else if *c == Seat {
					return 0;
				}
				distance += 1;
			}
		}
	}
}

#[aoc(day11, part2)]
fn part2(input: &Vec<Vec<Cell>>) -> usize {
	let mut grid = input.clone();
	loop {
		let lastgrid = grid.clone();
		let mut changed = false;
		
		for y in 0..grid.len() {
			for x in 0..grid[0].len() {
				if grid[y][x] == Floor {
					continue;
				}
				let mut adajcent = 0;
				adajcent += search_at(&lastgrid, y, x, -1,  0);
				adajcent += search_at(&lastgrid, y, x, -1,  1);
				adajcent += search_at(&lastgrid, y, x, -1, -1);
				adajcent += search_at(&lastgrid, y, x,  0,  1);
				adajcent += search_at(&lastgrid, y, x,  0, -1);
				adajcent += search_at(&lastgrid, y, x,  1,  0);
				adajcent += search_at(&lastgrid, y, x,  1,  1);
				adajcent += search_at(&lastgrid, y, x,  1, -1);
				
				//println!("adajcent: {}", adajcent);
				
				if adajcent == 0 {
					grid[y][x] = OccupiedSeat;
				} else if adajcent >= 5 {
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
