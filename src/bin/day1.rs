use std::{collections::HashSet, fs::read_to_string};

enum Direction {
	North,
	South,
	East,
	West,
}

impl Direction {
	fn right(&self) -> Direction {
		match self {
			Direction::North => Direction::East,
			Direction::South => Direction::West,
			Direction::East => Direction::South,
			Direction::West => Direction::North,
		}
	}

	fn left(&self) -> Direction {
		match self {
			Direction::North => Direction::West,
			Direction::South => Direction::East,
			Direction::East => Direction::North,
			Direction::West => Direction::South,
		}
	}

	fn translate(&self, pos: (i32, i32), moves: i32) -> (i32, i32) {
		match self {
			Direction::North => (pos.0, pos.1 - moves),
			Direction::South => (pos.0, pos.1 + moves),
			Direction::East => (pos.0 + moves, pos.1),
			Direction::West => (pos.0 - moves, pos.1),
		}
	}
}

fn main() {
	let file = read_to_string("input/day1.txt").unwrap();
	
	let mut curr = (0, 0);
	let mut dir = Direction::North;
	let mut locations = HashSet::new();
	let mut part2 = false;
	locations.insert(curr);
	for c in file.split(", ") {
		let moves = c[1..].parse::<i32>().unwrap();
		if c.starts_with("R") {
			dir = dir.right();
		} else {
			dir = dir.left();
		}
		for _ in 0..moves {
			curr = dir.translate(curr, 1);
			
			if !part2 && locations.contains(&curr) {
				println!("Day 1 part 2: {}", curr.0.abs() + curr.1.abs());
				part2 = true;
			}
			locations.insert(curr);
		}
	}

	let part1 = curr.0.abs() + curr.1.abs();
	println!("Day 1 part 1: {}", part1);
}