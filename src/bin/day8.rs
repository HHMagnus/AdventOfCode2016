use std::fs::read_to_string;

#[derive(Debug)]
enum Day8 {
	Rect(usize, usize),
	Row(usize, usize),
	Column(usize, usize),
}

fn main() {
	let file = read_to_string("input/day8.txt").unwrap();

	let lines = file.lines().map(|x| {
		if x.starts_with("rect") {
			let mut s = x.split("x");
			let a = s.next().unwrap().replace("rect ", "").parse::<usize>().unwrap();
			let b = s.next().unwrap().parse::<usize>().unwrap();
			Day8::Rect(a, b)
		} else if x.starts_with("rotate row") {
			let mut s = x.split(" by ");
			let a = s.next().unwrap().replace("rotate row y=", "").parse::<usize>().unwrap();
			let b = s.next().unwrap().parse::<usize>().unwrap();
			Day8::Row(a, b)
		} else if x.starts_with("rotate column") {
			let mut s = x.split(" by ");
			let a = s.next().unwrap().replace("rotate column x=", "").parse::<usize>().unwrap();
			let b = s.next().unwrap().parse::<usize>().unwrap();
			Day8::Column(a, b)
		} else {
			unreachable!("Unknown code: {}", x);
		}
	}).collect::<Vec<_>>();
	
	let mut map = Vec::new();

	for _ in 0..6 {
		let mut y = Vec::new();
		for _ in 0..50 {
			y.push(false);
		}
		map.push(y);
	}

	for inst in lines {
		match inst {
			Day8::Rect(a, b) => {
				for x in 0..a {
					for y in 0..b {
						map[y][x] = true;
					}
				}
			},
			Day8::Row(a, b) => {
				let x = map.get_mut(a).unwrap();
				x.rotate_right(b);
			},
			Day8::Column(a, b) => {
				let mut vec = Vec::new();
				for y in 0..6 {
					vec.push(map[y][a]);
				}
				vec.rotate_right(b);
				for y in 0..6 {
					map[y][a] = vec[y];
				}
			},
		}
	}

	let part1 = map.iter().flat_map(|x| x.iter()).filter(|&&y| y).count();
	println!("Day 8 part 1: {}", part1);
}