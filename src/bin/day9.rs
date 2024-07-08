use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day9.txt").unwrap();

	let mut it = file.chars();
	let mut length = 0;

	while let Some(next) = it.next() {
		if next == '(' {
			let mut start = String::new();
			while let Some(x) = it.next() {
				if x == 'x' {
					break;
				}
				start.push(x);
			}
			let mut end = String::new();
			while let Some(x) = it.next() {
				if x == ')' {
					break;
				}
				end.push(x);
			}
			let start = start.parse::<usize>().unwrap();
			let end = end.parse::<usize>().unwrap();
			length += start * end;
			for _ in 0..start {
				it.next();
			}
			continue;
		}
		length += 1;
	}

	println!("Day 9 part 1: {}", length);
	let part2 = part2(&file);
	println!("Day 9 part 2: {}", part2);
}

fn part2(file: &str) -> usize {
	let mut it = file.chars();
	let mut length = 0;

	while let Some(next) = it.next() {
		if next == '(' {
			let mut start = String::new();
			while let Some(x) = it.next() {
				if x == 'x' {
					break;
				}
				start.push(x);
			}
			let mut end = String::new();
			while let Some(x) = it.next() {
				if x == ')' {
					break;
				}
				end.push(x);
			}
			let start = start.parse::<usize>().unwrap();
			let end = end.parse::<usize>().unwrap();
			let mut compressed = String::new();
			for _ in 0..start {
				compressed.push(it.next().unwrap());
			}
			length += part2(&compressed) * end;
			continue;
		}
		length += 1;
	}

	return length;
}