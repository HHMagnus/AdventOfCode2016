use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day16.txt").unwrap();

	let part1 = solve(&file, 272);
	println!("Day 16 part 1: {}", part1);
	let part2 = solve(&file, 35651584);
	println!("Day 16 part 2: {}", part2);
}

fn solve(input: &str, length: usize) -> String {
	let mut arr = input.chars().collect::<Vec<_>>();

	while arr.len() < length {
		let mut clone = arr.clone();
		let mut revclone = arr.clone();
		revclone.reverse();
		revclone = revclone.into_iter().map(|x| if x == '1' { '0' } else { '1' }).collect();

		let mut vec = Vec::new();
		vec.append(&mut clone);
		vec.push('0');
		vec.append(&mut revclone);
		
		arr = vec;
	}
	
	while arr.len() != length {
		arr.pop();
	}

	while arr.len() % 2 == 0 {
		arr = arr.chunks(2).map(|x| if x[0] == x[1] { '1' } else { '0' }).collect();
	}
	arr.into_iter().collect::<String>()
}