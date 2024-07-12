use std::{collections::{HashSet, VecDeque}, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day13.txt").unwrap();

	let input = file.parse::<usize>().unwrap();

	let start = (1, 1);

	let mut queue = VecDeque::new();
	let mut visited = HashSet::new();
	visited.insert(start);
	queue.push_back((0, (0,0)));

	let end = (31, 39);

	let mut part2 = 0;

	while let Some((steps, (x, y))) = queue.pop_front() {
		if (x,y) == end {
			println!("Day 13 part 1: {}", steps);
			break;
		}

		if steps <= 50 {
			part2 += 1;
		}

		let neighbours = [
			(x - 1, y),
			(x + 1, y),
			(x, y - 1),
			(x, y + 1),
		].into_iter().filter(|&(x, y)| x >= 0 && y >= 0).filter(|&(x, y)| open(input, x, y));

		for neigh in neighbours {
			if visited.contains(&neigh) {
				continue;
			}
			visited.insert(neigh);
			queue.push_back((steps+1, neigh));
		}
	}

	println!("Day 13 part 2: {}", part2);
}

fn open(input: usize, x: i32, y: i32) -> bool {
	let mut v = x*x + 3*x + 2*x*y + y + y*y + input as i32;
	let mut count = 0;
	while v > 0 {
		if v & 1 > 0 {
			count += 1;
		}
		v >>= 1;
	}
	count % 2 == 0
}