use std::io::BufRead;

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn from_string(s: String) -> Command {
        let mut parts = s.split(' ');
        let command = parts.nth(0);
        let argument = parts.nth(0).unwrap().parse().unwrap();

        match command {
            Some("forward") => Command::Forward(argument),
            Some("down") => Command::Down(argument),
            Some("up") => Command::Up(argument),
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let commands = reader
        .lines()
        .map(|line| Command::from_string(line.unwrap()))
        .collect::<Vec<_>>();

    let mut position = 0 ;
    let mut depth = 0 ;
    for command in &commands {
        match command {
            Command::Forward(argument) => {
                position += argument;
            }
            Command::Down(argument) => {
                depth += argument;
            }
            Command::Up(argument) => {
                depth -= argument;
            }
        }
    }

    let part1 = position * depth;
    println!("{}", part1);

    position = 0 ;
    depth = 0 ;
    let mut aim = 0;
    for command in &commands {
        match command {
            Command::Forward(argument) => {
                position += argument;
                depth += argument * aim;
            }
            Command::Down(argument) => {
                aim += argument;
            }
            Command::Up(argument) => {
                aim -= argument;
            }
        }
    }

    let part2 = position * depth;
    println!("{}", part2);
}
