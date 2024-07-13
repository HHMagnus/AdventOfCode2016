use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day22.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();
	let lines = lines[2..].to_vec();

	let nodes = lines.into_iter().map(|x| {
		let mut split = x.split(" ").filter(|&x| x != "");
		let name = split.next().unwrap();
		let size = split.next().unwrap().replace("T", "").parse::<usize>().unwrap();
		let used = split.next().unwrap().replace("T", "").parse::<usize>().unwrap();
		let avail = split.next().unwrap().replace("T", "").parse::<usize>().unwrap();
		let usep = split.next().unwrap().replace("%", "").parse::<usize>().unwrap();
		(name, size, used, avail, usep)
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
}