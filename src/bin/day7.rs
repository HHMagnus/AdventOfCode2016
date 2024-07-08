use std::{collections::HashSet, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day7.txt").unwrap();

	let part1 = file.lines().into_iter().filter(|line| support_tls(line)).count();
	println!("Day 7 part 1: {}", part1);

	let part2 = file.lines().into_iter().filter(|line| support_ssl(line)).count();
	println!("Day 7 part 2: {}", part2);
}

fn support_tls(line: &str) -> bool {
	let mut inside = false;
	
	let mut it1 = line.chars();
	let mut it2 = line.chars();
	it2.next();
	let mut it3 = line.chars();
	it3.next();
	it3.next();
	let mut it4 = line.chars();
	it4.next();
	it4.next();
	it4.next();

	let mut result = false;

	while let Some(c4) = it4.next() {
		if let Some(c1) = it1.next() {
			if let Some(c2) = it2.next() {
				if let Some(c3) = it3.next() {
					if c1 == '[' || c2 == '[' || c3 == '[' || c4 == '[' {
						inside = true;
						continue;
					}
					if c1 == ']' || c2 == ']' || c3 == ']' || c4 == ']' {
						inside = false;
						continue;
					}

					if c1 == c4 && c2 == c3 && c1 != c2 {
						if inside {
							return false;
						}
						result = true;
					}
				}
			}
		}
	}
	
	result
}



fn support_ssl(line: &str) -> bool {
	let mut inside = false;
	
	let mut it1 = line.chars();
	let mut it2 = line.chars();
	it2.next();
	let mut it3 = line.chars();
	it3.next();
	it3.next();

	let mut insides = HashSet::new();
	let mut outsides = HashSet::new();

	while let Some(c3) = it3.next() {
		if let Some(c1) = it1.next() {
			if let Some(c2) = it2.next() {
				if c1 == '[' || c2 == '[' || c3 == '[' {
					inside = true;
					continue;
				}
				if c1 == ']' || c2 == ']' || c3 == ']' {
					inside = false;
					continue;
				}

				if c1 == c3 {
					if inside {
						insides.insert((c1, c2, c3));
					} else {
						outsides.insert((c1, c2, c3));
					}
				}
			}
		}
	}

	insides.iter().any(|(c1, c2, _)| outsides.iter().any(|(x1, x2, x3)| x2 == c1 && x1 == c2 && x3 == c2))
}