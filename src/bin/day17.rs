use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day17.txt").unwrap();

	let mut queue = VecDeque::new();
	queue.push_back(((0,0), Vec::new()));
	
	while let Some(((x, y), path)) = queue.pop_front() {
		if x == 3 && y == 3 {
			println!("Day 17 part 1: {}", path.into_iter().collect::<String>());
			break;
		}

		let doors = doors(&file, &path);
		
		let neighs = [
			((x, y - 1), 'U'),
			((x, y + 1), 'D'),
			((x - 1, y), 'L'),
			((x + 1, y), 'R'),
		].into_iter().enumerate().filter(|&(i, _)| doors[i]).map(|(_, x)| x).filter(|&((x, y), _)| x >= 0 && y >= 0 && x <= 3 && y <= 3);

		for (pos, c) in neighs {
			let mut new_path = path.clone();
			new_path.push(c);
			queue.push_back((pos, new_path));
		}
	}
}

fn doors(input: &str, path: &Vec<char>) -> [bool; 4] {
	let hash = format!("{:?}", md5::compute(format!("{}{}", input, path.iter().collect::<String>())));
	let mut chars = hash.chars();
	[open(chars.next().unwrap()), open(chars.next().unwrap()), open(chars.next().unwrap()), open(chars.next().unwrap())]
}

fn open(c: char) -> bool {
	c == 'b' || c == 'c' || c == 'd' || c == 'e' || c == 'f'
}