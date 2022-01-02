use std::{io::BufRead};

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

fn execute(program: &Vec<Instruction>, input: i64, mut registers: Vec<i64>, mut ip: usize) -> (Vec<i64>, usize) {
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
                dbg!(registers[dest], src);
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
        dbg!(ip);

        if let Instruction::Inp(_) = program[ip] {
            break;
        }
    }

    (registers, ip)
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let program = reader
        .lines()
        .map(|line| Instruction::from_string(&line.unwrap()))
        .collect::<Vec<_>>();
        
    let mut outputs1 = Vec::new();
    let mut outputs2 = Vec::new();
    for input1 in 1..=9 {
        let mut start = 0;
        let (registers, ip) = execute(&program, input1, vec![0i64; 4], start);
        outputs1.push(registers.clone());
        start = ip;

        for input2 in 1..=9 {
            let (registers, ip) = execute(&program, input2, outputs1[input1 as usize - 1].clone(), start);
            outputs2.push(registers.clone());
        }        
    }

    dbg!(&outputs1, outputs2);
}
