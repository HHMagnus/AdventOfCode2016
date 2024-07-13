use std::{collections::HashMap, fs::read_to_string};

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
}

fn main() {
    let file = read_to_string("input/day23.txt").unwrap();

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
            } else {
                unreachable!("Unknown instruction {}", x)
            }
        })
        .collect::<Vec<_>>();

    let mut registers = HashMap::new();
	registers.insert('a', 7);
    solve(&instructions, &mut registers);
    let part1 = registers.get(&'a').unwrap();
    println!("Day 23 part 1: {}", part1);
	
    let mut registers = HashMap::new();
	registers.insert('a', 12);
    solve(&instructions, &mut registers);
    let part2 = registers.get(&'a').unwrap();
    println!("Day 23 part 2: {}", part2);
}

fn solve(instructions: &Vec<Instruction>, registers: &mut HashMap<char, i64>) {
    let mut toggled = Vec::new();
    for _ in 0..instructions.len() {
        toggled.push(false);
    }
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
        }
        i += 1;
    }
}
