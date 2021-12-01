use std::{io::BufRead, usize};

fn count_increasing(measurements: &Vec<u32>) -> usize {
    measurements
        .iter()
        .skip(1)
        .zip(measurements.iter())
        .filter(|(a, b)| a > b)
        .count()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let measurements = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let part1 = count_increasing(&measurements);

    println!("{}", part1);

    let mut measurements2 = Vec::new();
    for i in 2..measurements.len() {
        let sum_of_three = measurements[i - 2] + measurements[i - 1] + measurements[i];
        measurements2.push(sum_of_three);
    }

    let part2 = count_increasing(&measurements2);

    println!("{}", part2);
}
