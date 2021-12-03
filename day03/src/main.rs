use std::io::BufRead;

fn count(numbers: &Vec<Vec<u32>>, target: u32) -> Vec<u32> {
    numbers
        .iter()
        .fold(vec![0u32; numbers[0].len()], |mut accu, number| {
            for i in 0..number.len() {
                if number[i] == target {
                    accu[i] += 1;
                }
            }

            accu
        })
}

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

    let zeros = count(&numbers, 0);
    let ones = count(&numbers, 1);

    let most_common = zeros
        .iter()
        .zip(ones.iter())
        .fold(vec![], |mut accu, (zero, one)| {
            assert_ne!(zero, one);
            if one > zero {
                accu.push(1u32);
            } else {
                accu.push(0u32);
            }

            accu
        });

    let (gamma_rate, epsilon_rate) =
        most_common
            .iter()
            .fold((0u32, 0u32), |(old_gamma, old_epsilon), bit| {
                if *bit == 1 {
                    (2 * old_gamma + 1, 2 * old_epsilon)
                } else {
                    (2 * old_gamma, 2 * old_epsilon + 1)
                }
            });

    let part1 = gamma_rate * epsilon_rate;
    println!("{}", part1);

    let mut candidates_oxygen_generator_rating = numbers.clone();
    let mut candidates_co2_scrubber_rating = numbers.clone();
    for i in 0..numbers[0].len() {
        if candidates_oxygen_generator_rating.len() > 1 {
            let zeros_for_oxygen_generator_rating = count(&candidates_oxygen_generator_rating, 0);
            let ones_for_oxygen_generator_rating = count(&candidates_oxygen_generator_rating, 1);
            let most_common_for_oxygen_generator_rating =
                if zeros_for_oxygen_generator_rating[i] > ones_for_oxygen_generator_rating[i] {
                    0
                } else {
                    1
                };

            candidates_oxygen_generator_rating = candidates_oxygen_generator_rating
                .into_iter()
                .filter(|number| number[i] == most_common_for_oxygen_generator_rating)
                .collect();
        }

        if candidates_co2_scrubber_rating.len() > 1 {
            let zeros_for_co2_scrubber_rating = count(&candidates_co2_scrubber_rating, 0);
            let ones_for_co2_scrubber_rating = count(&candidates_co2_scrubber_rating, 1);
            let least_common_for_co2_scrubber_rating =
                if zeros_for_co2_scrubber_rating[i] > ones_for_co2_scrubber_rating[i] {
                    1
                } else {
                    0
                };

            candidates_co2_scrubber_rating = candidates_co2_scrubber_rating
                .into_iter()
                .filter(|number| number[i] == least_common_for_co2_scrubber_rating)
                .collect();
        }
    }

    let oxygen_generator_rating = candidates_oxygen_generator_rating[0]
        .iter()
        .fold(0u32, |accu, bit| 2 * accu + bit);

    let co2_scrubber_rating = candidates_co2_scrubber_rating[0]
        .iter()
        .fold(0u32, |accu, bit| 2 * accu + bit);

    let part2 = oxygen_generator_rating * co2_scrubber_rating;
    println!("{}", part2);
}
