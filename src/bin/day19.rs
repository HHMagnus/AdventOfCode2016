use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day19.txt").unwrap();

	let elfs = file.parse::<usize>().unwrap();
	
	let mut elfs = (1..elfs+1).collect::<VecDeque<_>>();

	while elfs.len() > 1 {
		let uneven = elfs.len() % 2 == 1;
		elfs = elfs.into_iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, x)| x).collect::<VecDeque<_>>();
		if uneven && elfs.len() > 1 {
			elfs.pop_front();
		}
	}

	println!("Day 19 part 1: {}", elfs[0]);
}