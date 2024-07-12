use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day15.txt").unwrap();

	let mut discs = file.lines().map(|line| {
		let mut split = line.split(" positions; at time=0, it is at position ");
		let mut split2 = split.next().unwrap().split(" has ");
		split2.next();
		let x = split2.next().unwrap().parse::<usize>().unwrap();
		let y = split.next().unwrap().replace(".", "").parse::<usize>().unwrap();
		(x, y)
	}).collect::<Vec<_>>();


	let mut time = 0;
	
	while !discs.iter().enumerate().all(|(i, x)| (x.1 + i) % x.0 == 0) {
		discs = discs.into_iter().map(|(x, y)| (x, (y + 1) % x)).collect();
		time += 1;
	}

	println!("Day 15 part 1: {}", time-1);
}