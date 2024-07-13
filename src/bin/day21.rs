use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug, Clone, Copy)]
enum Instruction {
	SwapPostion(usize, usize),
	SwapLetter(char, char),
	RotateLeft(usize),
	RotateRight(usize),
	RotateLetter(char),
	Reverse(usize, usize),
	Move(usize, usize),
}

fn main() {
    let file = read_to_string("input/day21.txt").unwrap();

	let mut instructions = file.lines().map(|line| {
		if line.contains("swap position") {
			let mut split = line.split(" with position ");
			let x = split.next().unwrap().replace("swap position ", "").parse::<usize>().unwrap();
			let y = split.next().unwrap().parse::<usize>().unwrap();
			Instruction::SwapPostion(x, y)
		} else if line.contains("swap letter") {
			let mut split = line.split(" with letter ");
			let x = split.next().unwrap().replace("swap letter ", "").chars().next().unwrap();
			let y = split.next().unwrap().chars().next().unwrap();
			Instruction::SwapLetter(x, y)
		} else if line.contains("rotate left") {
			let x = line.replace("rotate left ", "").replace(" step", "").replace("s", "").parse().unwrap();
			Instruction::RotateLeft(x)
		} else if line.contains("rotate right") {
			let x = line.replace("rotate right ", "").replace(" step", "").replace("s", "").parse().unwrap();
			Instruction::RotateRight(x)
		} else if line.contains("rotate based on") {
			let x = line.replace("rotate based on position of letter ", "").chars().next().unwrap();
			Instruction::RotateLetter(x)
		} else if line.contains("reverse positions") {
			let mut split = line.split(" through ");
			let x = split.next().unwrap().replace("reverse positions ", "").parse::<usize>().unwrap();
			let y = split.next().unwrap().parse::<usize>().unwrap();
			Instruction::Reverse(x, y)
		} else if line.contains("move position") {
			let mut split = line.split(" to position ");
			let x = split.next().unwrap().replace("move position ", "").parse::<usize>().unwrap();
			let y = split.next().unwrap().parse::<usize>().unwrap();
			Instruction::Move(x, y)
		} else {
			unreachable!("Unknown line '{}'", line);
		}
	}).collect::<Vec<_>>();

	let mut input = "abcdefgh".chars().collect::<VecDeque<_>>();

	for &inst in &instructions {
		match inst {
			Instruction::SwapPostion(x, y) => {
				let c1 = input[x];
				let c2 = input[y];
				input[x] = c2;
				input[y] = c1;
			},
			Instruction::SwapLetter(x, y) => {
				let i = input.iter().enumerate().find(|&(_, &c)| c == x).unwrap().0;
				let j = input.iter().enumerate().find(|&(_, &c)| c == y).unwrap().0;
				input[i] = y;
				input[j] = x;
			},
			Instruction::RotateLeft(r) => {
				for _ in 0..r {
					let c = input.pop_front().unwrap();
					input.push_back(c);
				}
			},
			Instruction::RotateRight(r) => {
				for _ in 0..r {
					let c = input.pop_back().unwrap();
					input.push_front(c);
				}
			},
			Instruction::RotateLetter(x) => {
				let i = input.iter().enumerate().find(|&(_, &c)| c == x).unwrap().0;
				let mut total = i + 1;
				if i >= 4 {
					total += 1;
				}
				for _ in 0..total {
					let c = input.pop_back().unwrap();
					input.push_front(c);
				}
			},
			Instruction::Reverse(x, y) => {
				let mut new = Vec::new();
				for i in x..=y {
					new.push(input[i]);
				}
				for i in x..=y {
					input[i] = new.pop().unwrap();
				}
			},
			Instruction::Move(x, y) => {
				let c = input.remove(x).unwrap();
				input.insert(y, c);
			},
		}
	}

	println!("Day 21 part 1: {}", input.into_iter().collect::<String>());

	instructions.reverse();

	let mut input = "fbgdceah".chars().collect::<VecDeque<_>>();

	for inst in instructions {
		match inst {
			Instruction::SwapPostion(x, y) => {
				let c1 = input[x];
				let c2 = input[y];
				input[x] = c2;
				input[y] = c1;
			},
			Instruction::SwapLetter(x, y) => {
				let i = input.iter().enumerate().find(|&(_, &c)| c == x).unwrap().0;
				let j = input.iter().enumerate().find(|&(_, &c)| c == y).unwrap().0;
				input[i] = y;
				input[j] = x;
			},
			Instruction::RotateLeft(r) => {
				for _ in 0..r {
					let c = input.pop_back().unwrap();
					input.push_front(c);
				}
			},
			Instruction::RotateRight(r) => {
				for _ in 0..r {
					let c = input.pop_front().unwrap();
					input.push_back(c);
				}
			},
			Instruction::RotateLetter(x) => {
				let i = input.iter().enumerate().find(|&(_, &c)| c == x).unwrap().0;
				let r = i / 2 + if i % 2 == 1 || i == 0  { 1 } else { 5 };
				for _ in 0..r {
					let c = input.pop_front().unwrap();
					input.push_back(c);
				}
			},
			Instruction::Reverse(x, y) => {
				let mut new = Vec::new();
				for i in x..=y {
					new.push(input[i]);
				}
				for i in x..=y {
					input[i] = new.pop().unwrap();
				}
			},
			Instruction::Move(x, y) => {
				let c = input.remove(y).unwrap();
				input.insert(x, c);
			},
		}
	}

	println!("Day 21 part 2: {}", input.into_iter().collect::<String>());
}