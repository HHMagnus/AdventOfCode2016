use std::fs::read_to_string;

#[derive(Debug)]
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

	let instructions = file.lines().map(|line| {
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

	println!("{:?}", instructions);

	let input = "abcdefgh";
	
}