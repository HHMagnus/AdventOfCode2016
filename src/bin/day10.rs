use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
enum To {
	Bot(usize),
	Output(usize),
}

#[derive(Debug)]
enum Action {
	Value(usize, usize),
	Transfer(usize, To, To)
}

fn main() {
	let file = read_to_string("input/day10.txt").unwrap();

	let map = file.lines().map(|x| {
		if x.starts_with("value") {
			let mut split = x.split(" goes to bot ");
			let value = split.next().unwrap().replace("value ", "").parse::<usize>().unwrap();
			let bot = split.next().unwrap().parse::<usize>().unwrap();
			return Action::Value(value, bot);
		}

		let mut split = x.split(" gives low to ");
		let bot = split.next().unwrap().replace("bot ", "").parse::<usize>().unwrap();
		let mut split2 = split.next().unwrap().split(" and high to ");
		let o1 = split2.next().unwrap();
		let o2 = split2.next().unwrap();
		return Action::Transfer(bot, to(o1), to(o2));
	}).collect::<Vec<_>>();

	let mut values = map.iter().fold(HashMap::new(), |mut acc, x| match x {
		Action::Transfer(_, _, _) => acc,
		Action::Value(x, y) => {
			acc.entry(*y).or_insert(Vec::new()).push(*x);
			return acc;
		}
	});
	
	let botmap = map.into_iter().filter_map(|x| match x {
		Action::Value(_, _) => None,
		Action::Transfer(x, y, z) => Some((x, (y, z))),
	}).collect::<HashMap<_,_>>();
	
	let mut outputs = HashMap::new();

	while let Some(&next) = values.iter().find(|x| x.1.len() >= 2).map(|x| x.0) {
		let v = values.get_mut(&next).unwrap();
		v.sort();
		let high = v.pop().unwrap();
		v.reverse();
		let low = v.pop().unwrap();

		if low == 17 && high == 61 {
			println!("Day 10 part 1: {}", next);
		}

		let rule = &botmap[&next];

		match rule.0 {
			To::Bot(x) => {
				let v = values.entry(x).or_insert(Vec::new());
				v.push(low);
			}
			To::Output(x) => {
				let v = outputs.entry(x).or_insert(Vec::new());
				v.push(low);
			}
		}
		
		match rule.1 {
			To::Bot(x) => {
				let v = values.entry(x).or_insert(Vec::new());
				v.push(high);
			},
			To::Output(x) => {
				let v = outputs.entry(x).or_insert(Vec::new());
				v.push(high);
			}
		}
	}
	
	let part2 = outputs[&0][0] * outputs[&1][0] * outputs[&2][0];
	println!("Day 10 part 2: {}", part2);
}

fn to(x: &str) -> To {
	if x.starts_with("bot") {
		return To::Bot(x.replace("bot ", "").parse().unwrap())
	}
	return To::Output(x.replace("output ", "").parse().unwrap())
}