use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day2.txt").unwrap();

	let part1 = file.lines().map(|x| find1(x.chars().collect())).map(|x| x.to_string()).collect::<String>();
	println!("Day 2 part 1: {}", part1);

	let part2 = file.lines().map(|x| find2(x.chars().collect())).collect::<String>();
	println!("Day 2 part 2: {}", part2);
}

fn find1(input: Vec<char>) -> i32 {
	let mut curr = (0,0);
	for c in input {
		let next: (i32, i32) = match c {
			'U' => (curr.0, curr.1 - 1),
			'D' => (curr.0, curr.1 + 1),
			'L' => (curr.0 - 1, curr.1),
			'R' => (curr.0 + 1, curr.1),
			x => unreachable!("Unknown code: {}", x),
		};
		if next.0.abs() > 1 || next.1.abs() > 1 {
			continue;
		}
		curr = next;
	}

	match curr {
		(-1, -1) => 1,
		( 0, -1) => 2,
		( 1, -1) => 3,
		(-1,  0) => 4,
		( 0,  0) => 5,
		( 1,  0) => 6,
		(-1,  1) => 7,
		( 0,  1) => 8,
		( 1,  1) => 9,
		x => unreachable!("Unknown {:?}", x),
	}
}

fn find2(input: Vec<char>) -> char {
	let mut curr = (0,0);
	for c in input {
		let next: (i32, i32) = match c {
			'U' => (curr.0, curr.1 - 1),
			'D' => (curr.0, curr.1 + 1),
			'L' => (curr.0 - 1, curr.1),
			'R' => (curr.0 + 1, curr.1),
			x => unreachable!("Unknown code: {}", x),
		};
		if next.0.abs() + next.1.abs() > 2 {
			continue;
		}
		curr = next;
	}

	match curr {
		(0,  -2) => '1',
		(-1, -1) => '2',
		( 0, -1) => '3',
		( 1, -1) => '4',
		(-2,  0) => '5',
		(-1,  0) => '6',
		( 0,  0) => '7',
		( 1,  0) => '8',
		( 2,  0) => '9',
		(-1,  1) => 'A',
		( 0,  1) => 'B',
		( 1,  1) => 'C',
		( 0,  2) => 'D',
		x => unreachable!("Unknown {:?}", x),
	}
}