use std::{collections::HashMap, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day4.txt").unwrap();

	let mut part1 = 0;
	for line in file.lines() {
		let split = line.split("-").collect::<Vec<_>>();
		let idsplit = split.last().unwrap().split("[").collect::<Vec<_>>();
		let id = idsplit[0].parse::<i32>().unwrap();
		let check = idsplit[1].replace("]", "");

		let mut map = HashMap::new();
		for c in split[..split.len()-1].into_iter().flat_map(|cs| cs.chars().into_iter()) {
			*map.entry(c).or_insert(0) += 1;
		}

		let mut vector = map.into_iter().map(|(x, y)| (i32::MAX-y, x)).collect::<Vec<_>>();
		vector.sort();
		let value = vector[0..5].into_iter().map(|(_, y)| y).collect::<String>();

		if value == check {
			part1 += id;
		}
	}

	println!("Day 4 part 1: {}", part1);
}