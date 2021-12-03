use std::io::BufRead;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let numbers = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|char| char.to_digit(2).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let zeros = numbers.iter().fold(vec![0u32; numbers[0].len()], |mut accu, number| 
    {
        for i in 0..number.len() {
            if number[i] == 0 {
                accu[i] += 1;
            }
        }

        accu
    });

    let ones = numbers.iter().fold(vec![0u32; numbers[0].len()], |mut accu, number| 
    {
        for i in 0..number.len() {
            if number[i] == 1 {
                accu[i] += 1;
            }
        }

        accu
    });

    let (gamma_rate, epsilon_rate) = zeros.iter().zip(ones.iter()).fold((0u32, 0u32), |(old_gamma, old_epsilon), (zero, one)| {
        assert_ne!(zero, one);
        if one > zero {
            (2 * old_gamma + 1, 2 * old_epsilon)
        } else {
            (2 * old_gamma, 2 * old_epsilon + 1)
        }
    });

    let part1 = gamma_rate * epsilon_rate;
    println!("{}", part1);
}
