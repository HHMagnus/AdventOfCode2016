use std::{collections::HashMap, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day6.txt").unwrap();

	let mut fq = Vec::new();
	for line in file.lines() {
		while fq.len() < line.len() {
			fq.push(HashMap::new());
		}

		for (i, c) in line.chars().enumerate() {
			let map = fq.get_mut(i).unwrap();

			*map.entry(c).or_insert(0) += 1;
		}
	}

	let part1 = fq.iter().map(|x| x.iter().max_by_key(|&(_, j)| j).unwrap().0).collect::<String>();
	println!("Day 6 part 1: {}", part1);
	let part2 = fq.iter().map(|x| x.iter().min_by_key(|&(_, j)| j).unwrap().0).collect::<String>();
	println!("Day 6 part 2: {}", part2);
}