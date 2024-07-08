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

	let mut part2 = vec!['-', '-', '-', '-', '-', '-', '-', '-'];
	let mut hits = 0;
	
	let mut i = 0;
	while hits < 8 {
		let x = md5::compute(format!("{}{}", file, i));
		if x.0[0] == 0 && x.0[1] == 0 && x.0[2] < 16 {
			let pos = x.0[2] as usize;
			let val = format!("{:x}", x.0[3] >> 4);
			if pos < 8 && part2[pos] == '-' {
				part2[pos] = val.chars().next().unwrap();
				hits += 1;
			}
		}
		i += 1;
	}

	let part2 = part2.into_iter().collect::<String>();
	println!("Day 5 part 2: {}", part2);
}