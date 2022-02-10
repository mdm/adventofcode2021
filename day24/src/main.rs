use std::io::BufRead;
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Operand {
    Register(usize),
    Constant(i64),
}

#[derive(Debug)]
enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

impl Instruction {
    fn from_string(str: &str) -> Instruction {
        let mut parts = str.split(' ');
        let operation = parts.next().unwrap();
        let operands = parts
            .map(|operand| match operand {
                "w" => Operand::Register(0),
                "x" => Operand::Register(1),
                "y" => Operand::Register(2),
                "z" => Operand::Register(3),
                _ => Operand::Constant(operand.parse().unwrap()),
            })
            .collect::<Vec<_>>();

        match operation {
            "inp" => Instruction::Inp(operands[0].clone()),
            "add" => Instruction::Add(operands[0].clone(), operands[1].clone()),
            "mul" => Instruction::Mul(operands[0].clone(), operands[1].clone()),
            "div" => Instruction::Div(operands[0].clone(), operands[1].clone()),
            "mod" => Instruction::Mod(operands[0].clone(), operands[1].clone()),
            "eql" => Instruction::Eql(operands[0].clone(), operands[1].clone()),
            _ => unreachable!(),
        }
    }
}

fn execute(
    program: &Vec<Instruction>,
    input: i64,
    mut registers: Vec<i64>,
    mut ip: usize,
) -> (Vec<i64>, usize) {
    loop {
        match &program[ip] {
            Instruction::Inp(dest) => {
                let dest = match dest {
                    Operand::Register(dest) => *dest,
                    Operand::Constant(_) => unreachable!(),
                };
                registers[dest] = input;
            }
            Instruction::Add(dest, src) => {
                let dest = match dest {
                    Operand::Register(dest) => *dest,
                    Operand::Constant(_) => unreachable!(),
                };
                let src = match src {
                    Operand::Register(src) => registers[*src],
                    Operand::Constant(value) => *value,
                };
                registers[dest] = registers[dest] + src;
            }
            Instruction::Mul(dest, src) => {
                let dest = match dest {
                    Operand::Register(dest) => *dest,
                    Operand::Constant(_) => unreachable!(),
                };
                let src = match src {
                    Operand::Register(src) => registers[*src],
                    Operand::Constant(value) => *value,
                };
                // dbg!(registers[dest], src);
                registers[dest] = registers[dest] * src;
            }
            Instruction::Div(dest, src) => {
                let dest = match dest {
                    Operand::Register(dest) => *dest,
                    Operand::Constant(_) => unreachable!(),
                };
                let src = match src {
                    Operand::Register(src) => registers[*src],
                    Operand::Constant(value) => *value,
                };
                registers[dest] = registers[dest] / src;
            }
            Instruction::Mod(dest, src) => {
                let dest = match dest {
                    Operand::Register(dest) => *dest,
                    Operand::Constant(_) => unreachable!(),
                };
                let src = match src {
                    Operand::Register(src) => registers[*src],
                    Operand::Constant(value) => *value,
                };
                registers[dest] = registers[dest] % src;
            }
            Instruction::Eql(dest, src) => {
                let dest = match dest {
                    Operand::Register(dest) => *dest,
                    Operand::Constant(_) => unreachable!(),
                };
                let src = match src {
                    Operand::Register(src) => registers[*src],
                    Operand::Constant(value) => *value,
                };
                registers[dest] = if registers[dest] == src { 1 } else { 0 };
            }
        }

        ip += 1;
        // dbg!(ip);

        if ip == program.len() {
            break;
        }

        if let Instruction::Inp(_) = program[ip] {
            break;
        }
    }

    (registers, ip)
}

fn find_model_number(snippets: &Vec<Vec<Instruction>>, depth: usize, register_z: i64, memo: &mut HashSet<(usize, i64)>, reverse: bool) -> Option<Vec<i64>> {
    if depth == 14 {
        if register_z == 0 {
            return Some(Vec::new());
        } else {
            return None;
        }
    }

    if memo.contains(&(depth, register_z)) {
        return None;
    }

    let mut digits = (1..=9).collect::<Vec<_>>();
    if reverse {
        digits.reverse();
    }

    for digit in digits {
        let registers = vec![0, 0, 0, register_z];
        let (new_registers, _ip) = execute(&snippets[depth], digit, registers, 0);

        let model_number = find_model_number(snippets, depth + 1, new_registers[3], memo, reverse);

        match model_number {
            Some(mut model_number) => {
                model_number.push(digit);
                return Some(model_number);
            }
            None => {
                memo.insert((depth + 1, new_registers[3]));
            }
        }
    }

    None
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let program = reader
        .lines()
        .map(|line| Instruction::from_string(&line.unwrap()))
        .collect::<Vec<_>>();

    let mut snippets = Vec::new();
    let mut snippet = Vec::new();
    for instruction in program {
        if let Instruction::Inp(_) = instruction {
            if !snippet.is_empty() {
                snippets.push(snippet);
                snippet = Vec::new();
            }
        }

        snippet.push(instruction);
    }
    snippets.push(snippet);


    let model_number = find_model_number(&snippets, 0, 0, &mut HashSet::new(), true).unwrap();
    let part1 = model_number.iter().rev().fold(0i64, |number, digit| number * 10 + digit);
    println!("{}", part1);

    let model_number = find_model_number(&snippets, 0, 0, &mut HashSet::new(), false).unwrap();
    let part2 = model_number.iter().rev().fold(0i64, |number, digit| number * 10 + digit);
    println!("{}", part2);
}
