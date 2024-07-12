use std::{collections::{HashSet, VecDeque}, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Type {
	Microchip,
	Generator
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Element {
	Polonium,
	Thulium,
	Promethium,
	Ruthenium,
	Cobalt,
}

fn main() {
	let file = read_to_string("input/day11.txt").unwrap();
	let floors = file.lines().map(|x| {
		let mut vec = Vec::new();
		if x.contains("polonium generator") {
			vec.push((Element::Polonium, Type::Generator))
		}
		if x.contains("polonium-compatible microchip") {
			vec.push((Element::Polonium, Type::Microchip))
		}
		if x.contains("thulium generator") {
			vec.push((Element::Thulium, Type::Generator))
		}
		if x.contains("thulium-compatible microchip") {
			vec.push((Element::Thulium, Type::Microchip))
		}
		if x.contains("promethium generator") {
			vec.push((Element::Promethium, Type::Generator))
		}
		if x.contains("promethium-compatible microchip") {
			vec.push((Element::Promethium, Type::Microchip))
		}
		if x.contains("ruthenium generator") {
			vec.push((Element::Ruthenium, Type::Generator))
		}
		if x.contains("ruthenium-compatible microchip") {
			vec.push((Element::Ruthenium, Type::Microchip))
		}
		if x.contains("cobalt generator") {
			vec.push((Element::Cobalt, Type::Generator))
		}
		if x.contains("cobalt-compatible microchip") {
			vec.push((Element::Cobalt, Type::Microchip))
		}
		vec
	}).collect::<Vec<_>>();
	
	let mut visited = HashSet::new();
	visited.insert((0, floors.clone()));
	let mut queue = VecDeque::new();
	queue.push_back((0, 1, floors));

	'bfs: while let Some((elevator, steps, next)) = queue.pop_front() {
		let nexts = mutate(elevator, &next).into_iter().filter(|x| valid(&x.1));
		for x in nexts {
			if visited.contains(&x) {
				continue;
			}
			visited.insert(x.clone());
			let (e, x) = x;
			if x[0].len() == 0 && x[1].len() == 0 && x[2].len() == 0 {
				println!("Day 11 part 1: {}", steps);
				break 'bfs;
			}
			queue.push_back((e, steps + 1, x));
		}
	}
}

fn mutate(floor: usize, floors: &Vec<Vec<(Element, Type)>>) -> Vec<(usize, Vec<Vec<(Element, Type)>>)> {
	let mut result = Vec::new();
	let floor_vec = &floors[floor];
	for i in 0..floor_vec.len() {
		let mut fs = floors.clone();
		let mut f = floor_vec.clone();
		let x = f.remove(i);
		fs[floor] = f;
		if floor > 0 {
			let mut fsb = fs.clone();
			fsb[floor-1].push(x);
			result.push((floor-1, fsb));
		}
		if floor < floors.len()-1 {
			fs[floor+1].push(x);
			result.push((floor+1,fs));
		}
	}

	for i in 0..floor_vec.len() {
		for j in i+1..floor_vec.len() {
			let mut fs = floors.clone();
			let mut f = floor_vec.clone();
			let x1 = f.remove(j);
			let x2 = f.remove(i);
			fs[floor] = f;
			if floor > 0 {
				let mut fsb = fs.clone();
				fsb[floor-1].push(x1);
				fsb[floor-1].push(x2);
				result.push((floor-1,fsb));
			}
			if floor < floors.len()-1 {
				fs[floor+1].push(x1);
				fs[floor+1].push(x2);
				result.push((floor+1, fs));
			}
		}
	}
	result
}


fn valid(floors: &Vec<Vec<(Element, Type)>>) -> bool {
	for floor in floors {
		let has_generator = floor.iter().any(|x| x.1 == Type::Generator);
		if !has_generator { continue; }

		for x in floor {
			if x.1 == Type::Microchip && !floor.iter().any(|y| y.0 == x.0 && y.1 == Type::Generator) {
				return false;
			}
		}
	}
	true
}