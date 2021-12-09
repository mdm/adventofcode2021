use itertools::Itertools;
use std::{collections::HashMap, io::BufRead};

fn map_string(s: &str, permutation: &Vec<char>) -> String {
    let mut mapped = s
        .chars()
        .map(|char| {
            let index = "abcdefg".chars().position(|c| c == char).unwrap();
            permutation[index]
        })
        .collect::<Vec<_>>();
    mapped.sort();
    mapped.iter().collect::<String>()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let displays = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split(" | ");
            let signals = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| {
                    let mut chars = s.chars().collect::<Vec<_>>();
                    chars.sort();
                    chars.iter().collect::<String>()
                })
                .collect::<Vec<_>>();
            let output = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| {
                    let mut chars = s.chars().collect::<Vec<_>>();
                    chars.sort();
                    chars.iter().collect::<String>()
                })
                .collect::<Vec<_>>();
            (signals, output)
        })
        .collect::<Vec<_>>();

    let mut digits = HashMap::new();
    digits.insert("abcefg".to_string(), 0);
    digits.insert("cf".to_string(), 1);
    digits.insert("acdeg".to_string(), 2);
    digits.insert("acdfg".to_string(), 3);
    digits.insert("bcdf".to_string(), 4);
    digits.insert("abdfg".to_string(), 5);
    digits.insert("abdefg".to_string(), 6);
    digits.insert("acf".to_string(), 7);
    digits.insert("abcdefg".to_string(), 8);
    digits.insert("abcdfg".to_string(), 9);

    let mut part1 = 0;
    let mut part2 = 0;
    for display in displays {
        for permutation in "abcdefg".chars().permutations(7) {
            let valid = display
                .0
                .iter()
                .map(|signal| map_string(signal, &permutation))
                .all(|signal| digits.contains_key(&signal));

            if valid {
                let output_digits = display
                    .1
                    .iter()
                    .map(|output| {
                        let mapped = map_string(output, &permutation);
                        digits[&mapped]
                    })
                    .collect::<Vec<_>>();

                let mut output_value = 0;
                for digit in &output_digits {
                    if *digit == 1 || *digit == 4 || *digit == 7 || *digit == 8 {
                        part1 += 1;
                    }
                    output_value *= 10;
                    output_value += digit;
                }

                part2 += output_value;

                break;
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
