use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day18.txt").unwrap();
	let mut rows = Vec::new();
	rows.push(file.chars().collect::<VecDeque<_>>());
	for i in 1..400000 {
		let mut prev_row = rows[i-1].clone();
		prev_row.push_front('.');
		prev_row.push_back('.');

		let row = prev_row.into_iter().collect::<Vec<_>>();
		let row = row.into_iter().as_slice().windows(3).map(|x| if is_trap(x) { '^' } else { '.' }).collect::<VecDeque<_>>();
		rows.push(row);
	}

	let part1 = rows[0..40].iter().flat_map(|x| x).filter(|&&x| x == '.').count();
	println!("Day 18 part 1: {}", part1);
	let part2 = rows.iter().flat_map(|x| x).filter(|&&x| x == '.').count();
	println!("Day 18 part 2: {}", part2);
}

fn is_trap(x: &[char]) -> bool {
	(x[0] == '^' && x[1] == '^' && x[2] == '.')
	|| (x[0] == '.' && x[1] == '^' && x[2] == '^')
	|| (x[0] == '.' && x[1] == '.' && x[2] == '^')
	|| (x[0] == '^' && x[1] == '.' && x[2] == '.')
}