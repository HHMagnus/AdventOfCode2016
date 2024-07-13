use std::{collections::{BTreeMap, HashSet, VecDeque}, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day22.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();
	let lines = lines[2..].to_vec();

	let nodes = lines.into_iter().map(|x| {
		let mut split = x.split(" ").filter(|&x| x != "");
		let name = split.next().unwrap();
		let mut namesplit = name.split("-y");
		let x = namesplit.next().unwrap().replace("/dev/grid/node-x", "").parse::<usize>().unwrap();
		let y = namesplit.next().unwrap().parse::<usize>().unwrap();
		let size = split.next().unwrap().replace("T", "").parse::<usize>().unwrap();
		let used = split.next().unwrap().replace("T", "").parse::<usize>().unwrap();
		let avail = split.next().unwrap().replace("T", "").parse::<usize>().unwrap();
		let usep = split.next().unwrap().replace("%", "").parse::<usize>().unwrap();
		((x,y), size, used, avail, usep)
	}).collect::<Vec<_>>();
	
	let mut part1 = 0;

	for i in 0..nodes.len() {
		for j in i+1..nodes.len() {
			let node1 = nodes[i];
			let node2 = nodes[j];
			if (node1.2 > 0 && node1.2 <= node2.3) || node2.2 > 0 && node2.2 <= node1.3 {
				part1 += 1;
			}
		}
	}

	println!("Day 22 part 1: {}", part1);

	let maxx = nodes.iter().map(|x| x.0.0).max().unwrap();
	let mut queue = VecDeque::new();
	let nodemap = nodes.into_iter().map(|x| (x.0, (x.2, x.1))).collect::<BTreeMap<_,_>>();
	queue.push_back(((maxx, 0), 0, nodemap.clone()));
	let mut visited = HashSet::new();

	while let Some((g, steps, map)) = queue.pop_front() {
		if g.0 == 0 && g.1 == 0 {
			println!("Day 22 part 2: {}", steps);
			break;
		}

		let zero = map.iter().find(|x| x.1.0 == 0).unwrap();
		let zero_size = zero.1.1;
		let zero = *zero.0;
		
		let neighbours = [
			(zero.0 - 1, zero.1),
			(zero.0  + 1, zero.1),
			(zero.0 , zero.1 - 1),
			(zero.0 , zero.1 + 1)
		].into_iter().filter(|x| map.contains_key(x));
		
		for neighbour in neighbours {
			let next = map[&neighbour];
			if next.0 > zero_size {
				continue;
			}
			let mut newmap = map.clone();

			*newmap.get_mut(&neighbour).unwrap() = (0, next.1);
			*newmap.get_mut(&zero).unwrap() = (next.0, zero_size);

			let mut pos = g;
			if pos == neighbour {
				pos = zero;
			}

			let key = (neighbour, pos);
			if visited.contains(&key) {
				continue;
			}
			visited.insert(key);

			queue.push_back((pos, steps+1, newmap));
		}

		
	}
}