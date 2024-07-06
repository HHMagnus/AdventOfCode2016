use std::{fs::read_to_string, vec};

fn main() {
	let file = read_to_string("input/day3.txt").unwrap();

	let input = file.lines().map(|cs| cs.split(" ").filter_map(|x| x.parse::<i32>().map(|x| Some(x)).unwrap_or(None)).collect::<Vec<_>>() ).collect::<Vec<_>>();

	let part1 = input.iter().filter(|x| valid(x)).count();
	println!("Day 3 part 1: {}", part1);

	let mut part2 = 0;
	for i in (0..input.len()).step_by(3) {
		let vec1 = vec![input[i][0], input[i+1][0], input[i+2][0]];
		let vec2 = vec![input[i][1], input[i+1][1], input[i+2][1]];
		let vec3 = vec![input[i][2], input[i+1][2], input[i+2][2]];
		if valid(&vec1) {
			part2 += 1;
		}
		if valid(&vec2) {
			part2 += 1;
		}
		if valid(&vec3) {
			part2 += 1;
		}
	}
	println!("Day 3 part 2: {}", part2);
}

fn valid(vec: &Vec<i32>) -> bool {
	vec[0] + vec[1] > vec[2] && vec[1] + vec[2] > vec[0] && vec[2] + vec[0] > vec[1]
}