use std::{collections::{BTreeSet, HashMap, HashSet, VecDeque}, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day24.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x, y), c))).collect::<HashMap<_, _>>();

	let nums = map.iter().filter_map(|(_, c)| if c.is_digit(10) { Some(c.to_digit(10).unwrap()) } else { None }).collect::<BTreeSet<_>>();

	let start = *map.iter().find(|(_, c)| c == &&'0').unwrap().0;

	let mut queue = VecDeque::new();
	queue.push_back((start, 0, vec![0]));
	let mut visited = HashSet::new();
	visited.insert((start, vec![0]));
	let mut part1 = None;

	while let Some((pos, steps, vec)) = queue.pop_front() {
		if vec.len() == nums.len() {
			if part1.is_none() {
				part1 = Some(steps);
			}
			if start == pos {
				println!("Day 24 part 1: {}", part1.unwrap());
				println!("Day 24 part 2: {}", steps);
				break;
			}
		}
		let neighbours = [
			(pos.0 - 1, pos.1),
			(pos.0 + 1, pos.1),
			(pos.0, pos.1 - 1),
			(pos.0, pos.1 + 1),
		].into_iter();

		for neighbour in neighbours {
			let c = map[&neighbour];
			if c == '#' {
				continue;
			}

			let mut new_vec = vec.clone();
			
			if c.is_digit(10) {
				let num = c.to_digit(10).unwrap();
				if !vec.contains(&num) {
					new_vec.push(num);
				}
			}

			let key = (neighbour, new_vec.clone());
			if visited.contains(&key) {
				continue;
			}
			visited.insert(key);

			queue.push_back((neighbour, steps+1, new_vec));
		}
	}
}