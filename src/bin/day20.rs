use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day20.txt").unwrap();

	let mut input = file.lines().map(|x| {
		let mut split = x.split("-");
		let x = split.next().unwrap().parse::<usize>().unwrap();
		let y = split.next().unwrap().parse::<usize>().unwrap();
		(x, y)
	}).collect::<Vec<_>>();

	input.sort();

	let mut part1 = 0;
	let mut part2 = 0;
	let mut curr = 0;
	for (x, y) in input {
		if x <= part1 && y >= part1 {
			part1 = y+1;
		}

		if x <= curr && y >= curr {
			curr = curr.max(y+1);
		}

		if x > curr {
			part2 += x - curr;
			curr = y + 1;
		}
	}

	println!("Day 20 part 1: {}", part1);
	println!("Day 20 part 2: {}", part2);
}