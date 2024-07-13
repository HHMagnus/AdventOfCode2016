use std::{collections::{BTreeMap, HashSet}, fs::read_to_string};

#[derive(Debug, Clone, Copy)]
enum RegVal {
    Reg(char),
    Val(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(RegVal, RegVal),
    Inc(RegVal),
    Dec(RegVal),
    Jnz(RegVal, RegVal),
    Tgl(RegVal),
    Out(RegVal),
}

fn main() {
    let file = read_to_string("input/day25.txt").unwrap();

    let instructions = file
        .lines()
        .map(|x| {
            let mut split = x.split(" ");
            split.next();
            if x.contains("cpy") {
                let x = split.next().unwrap();
                let x = x
                    .parse::<i64>()
                    .map(|y| RegVal::Val(y))
                    .unwrap_or(RegVal::Reg(x.chars().next().unwrap()));
                let y = split.next().unwrap();
                let y: RegVal = y
                    .parse::<i64>()
                    .map(|y| RegVal::Val(y))
                    .unwrap_or(RegVal::Reg(y.chars().next().unwrap()));
                Instruction::Cpy(x, y)
            } else if x.contains("inc") {
                let x = split.next().unwrap();
                let x = x
                    .parse::<i64>()
                    .map(|y| RegVal::Val(y))
                    .unwrap_or(RegVal::Reg(x.chars().next().unwrap()));
                Instruction::Inc(x)
            } else if x.contains("dec") {
                let x = split.next().unwrap();
                let x = x
                    .parse::<i64>()
                    .map(|y| RegVal::Val(y))
                    .unwrap_or(RegVal::Reg(x.chars().next().unwrap()));
                Instruction::Dec(x)
            } else if x.contains("jnz") {
                let x = split.next().unwrap();
                let x = x
                    .parse::<i64>()
                    .map(|y| RegVal::Val(y))
                    .unwrap_or(RegVal::Reg(x.chars().next().unwrap()));
                let y = split.next().unwrap();
                let y: RegVal = y
                    .parse::<i64>()
                    .map(|y| RegVal::Val(y))
                    .unwrap_or(RegVal::Reg(y.chars().next().unwrap()));
                Instruction::Jnz(x, y)
            } else if x.contains("tgl") {
                let x = split.next().unwrap();
                let x = x
                    .parse::<i64>()
                    .map(|y| RegVal::Val(y))
                    .unwrap_or(RegVal::Reg(x.chars().next().unwrap()));
                Instruction::Tgl(x)
            } else if x.contains("out") {
                let x = split.next().unwrap();
                let x = x
                    .parse::<i64>()
                    .map(|y| RegVal::Val(y))
                    .unwrap_or(RegVal::Reg(x.chars().next().unwrap()));
                Instruction::Out(x)
            } else {
                unreachable!("Unknown instruction {}", x)
            }
        })
        .collect::<Vec<_>>();

    let mut day25 = 0;
    while !solve(&instructions, day25) {
        day25 += 1;
    }

    println!("Day 25: {}", day25);
}

fn solve(instructions: &Vec<Instruction>, input: i64) -> bool {
	let mut visited = HashSet::new();
    let mut registers = BTreeMap::new();
    registers.insert('a', input);
    let mut toggled = Vec::new();
    for _ in 0..instructions.len() {
        toggled.push(false);
    }
    let mut prev = false;
    let mut i = 0 as i64;
    while i >= 0 && i < instructions.len() as i64 {
        let mut inst = instructions[i as usize];
        if toggled[i as usize] {
            inst = match inst {
                Instruction::Cpy(x, y) => Instruction::Jnz(x, y),
                Instruction::Inc(x) => Instruction::Dec(x),
                Instruction::Dec(x) => Instruction::Inc(x),
                Instruction::Jnz(x, y) => Instruction::Cpy(x, y),
                Instruction::Tgl(x) => Instruction::Inc(x),
                Instruction::Out(x) => Instruction::Inc(x),
            }
        }
        match inst {
            Instruction::Cpy(x, y) => {
                let val = match x {
                    RegVal::Reg(x) => *registers.get(&x).unwrap_or(&0),
                    RegVal::Val(x) => x,
                };
                let y = match y {
                    RegVal::Val(_) => {
                        i += 1;
                        continue;
                    }
                    RegVal::Reg(y) => y,
                };
                *registers.entry(y).or_insert(0) = val;
            }
            Instruction::Inc(x) => {
                let x = match x {
                    RegVal::Val(_) => {
                        i += 1;
                        continue;
                    }
                    RegVal::Reg(x) => x,
                };
                *registers.entry(x).or_insert(0) += 1;
            }
            Instruction::Dec(x) => {
                let x = match x {
                    RegVal::Val(_) => {
                        i += 1;
                        continue;
                    }
                    RegVal::Reg(x) => x,
                };
                *registers.entry(x).or_insert(0) -= 1;
            }
            Instruction::Jnz(x, y) => {
                let val = match x {
                    RegVal::Reg(x) => *registers.get(&x).unwrap_or(&0),
                    RegVal::Val(x) => x,
                };
                let y = match y {
                    RegVal::Reg(y) => *registers.get(&y).unwrap_or(&0),
                    RegVal::Val(y) => y,
                };
                if val != 0 {
                    i += y;
                    continue;
                }
            }
            Instruction::Tgl(x) => {
                let val = match x {
                    RegVal::Reg(x) => *registers.get(&x).unwrap_or(&0),
                    RegVal::Val(x) => x,
                };
                let t = i + val;
                if t >= 0 && t < instructions.len() as i64 {
                    toggled[t as usize] = !toggled[t as usize];
                }
            }
            Instruction::Out(x) => {
                let val = match x {
                    RegVal::Reg(x) => *registers.get(&x).unwrap_or(&0),
                    RegVal::Val(x) => x,
                };
				if val != 0 && val != 1 {
					return false;
				}
				if prev {
					if val != 1 {
						return false;
					}
					prev = false;
				} else {
					if val != 0 {
						return false;
					}
					prev = true;
				}

				let key = (prev, registers.clone(), i);
				if visited.contains(&key) {
					return true;
				}
				visited.insert(key);
            }
        }
        i += 1;
    }

    false
}
