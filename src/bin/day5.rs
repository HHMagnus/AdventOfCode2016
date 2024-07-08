use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day5.txt").unwrap();

	let mut part1 = String::new();

	let mut i = 0;
	while part1.len() < 8 {
		let x = md5::compute(format!("{}{}", file, i));
		if x.0[0] == 0 && x.0[1] == 0 && x.0[2] < 16 {
			part1.push_str(&format!("{:x}", x.0[2]));
		}
		i += 1;
	}
	
	println!("Day 5 part 1: {}", part1);
}