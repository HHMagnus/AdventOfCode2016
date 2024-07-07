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

		let real_name = line.chars().into_iter().map(|x| decrypt(x, id as u32)).collect::<String>();
		if real_name.contains("north") {
			println!("Day 4 part 2: {}", id);
		}
	}

	println!("Day 4 part 1: {}", part1);
}

fn decrypt(c: char, id: u32) -> char {
	let shift = id % 26;
	if c == '-' { return ' ' }
	if !c.is_alphabetic() { return c}
	let base = c as u32 - 'a' as u32;
	let moved = (base + shift) % 26;
	let rebased = moved + 'a' as u32;
	char::from_u32(rebased).unwrap()
}