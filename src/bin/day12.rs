use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
enum RegVal {
	Reg(char),
	Val(i64),
}

#[derive(Debug)]
enum Instruction {
	Cpy(RegVal, char),
	Inc(char),
	Dec(char),
	Jnz(RegVal, i64)
}

fn main() {
    let file = read_to_string("input/day12.txt").unwrap();

	let instructions = file.lines().map(|x| {
		let mut split = x.split(" ");
		split.next();
		if x.contains("cpy") {
			let x = split.next().unwrap();
			let x = x.parse::<i64>().map(|y| RegVal::Val(y)).unwrap_or(RegVal::Reg(x.chars().next().unwrap()));
			let y = split.next().unwrap().chars().next().unwrap();
			Instruction::Cpy(x, y)
		} else if x.contains("inc") {
			let x = split.next().unwrap().chars().next().unwrap();
			Instruction::Inc(x)
		} else if x.contains("dec") {
			let x = split.next().unwrap().chars().next().unwrap();
			Instruction::Dec(x)
		} else if x.contains("jnz") {
			let x = split.next().unwrap();
			let x = x.parse::<i64>().map(|y| RegVal::Val(y)).unwrap_or(RegVal::Reg(x.chars().next().unwrap()));
			let y = split.next().unwrap().parse::<i64>().unwrap();
			Instruction::Jnz(x, y)
		} else {
			unreachable!("Unknown instruction {}", x)
		}
	}).collect::<Vec<_>>();

	let mut registers = HashMap::new();
	solve(&instructions, &mut registers);
	let part1 = registers.get(&'a').unwrap();
	println!("Day 12 part 1: {}", part1);

	let mut registers = HashMap::new();
	registers.insert('c', 1);
	solve(&instructions, &mut registers);
	let part1 = registers.get(&'a').unwrap();
	println!("Day 12 part 2: {}", part1);
}

fn solve(instructions: &Vec<Instruction>, registers: &mut HashMap<char, i64>) {
		let mut i = 0 as i64;
		while i >= 0 && i < instructions.len() as i64 {
				let inst = &instructions[i as usize];
				match inst {
					Instruction::Cpy(x, y) => {
						let val = match x {
								RegVal::Reg(x) => *registers.get(x).unwrap_or(&0),
								RegVal::Val(x) => *x,
							};
						*registers.entry(*y).or_insert(0) = val;
					},
					Instruction::Inc(x) => {
						*registers.entry(*x).or_insert(0) += 1;
					},
					Instruction::Dec(x) => {
						*registers.entry(*x).or_insert(0) -= 1;
					},
					Instruction::Jnz(x, y) => {
						let val = match x {
							RegVal::Reg(x) => *registers.get(x).unwrap_or(&0),
							RegVal::Val(x) => *x,
						};
						if val != 0 {
							i += *y;
							continue;
						}
					},
				}
				i += 1;
			}
	}