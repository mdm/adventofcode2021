use std::{collections::HashMap, io::BufRead};

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

    let mut part1 = 0;
    let mut part2 = 0;
    for display in displays {
        let mut signal_to_digit = HashMap::new();
        let mut digit_to_signal = HashMap::new();

        for signal in &display.0 {
            match signal.len() {
                2 => {
                    signal_to_digit.insert(signal, 1);
                    digit_to_signal.insert(1, signal);
                }
                4 => {
                    signal_to_digit.insert(signal, 4);
                    digit_to_signal.insert(4, signal);
                }
                3 => {
                    signal_to_digit.insert(signal, 7);
                    digit_to_signal.insert(7, signal);
                }
                7 => {
                    signal_to_digit.insert(signal, 8);
                    digit_to_signal.insert(8, signal);
                }
                _ => {}
            }
        }

        for output in &display.1 {
            if signal_to_digit.contains_key(output) {
                part1 += 1;
            }
        }

        for signal in &display.0 {
            if signal.len() != 5 {
                continue;
            }

            let is_three = digit_to_signal[&1]
                .chars()
                .all(|char| signal.contains(char));
            if is_three {
                signal_to_digit.insert(signal, 3);
                digit_to_signal.insert(3, signal);
            }
        }

        for signal in &display.0 {
            if signal.len() != 6 {
                continue;
            }

            let is_nine = digit_to_signal[&3]
                .chars()
                .all(|char| signal.contains(char));
            if is_nine {
                signal_to_digit.insert(signal, 9);
                digit_to_signal.insert(9, signal);
            }
        }

        for signal in &display.0 {
            if signal.len() != 6 {
                continue;
            }

            let maybe_zero = digit_to_signal[&1]
                .chars()
                .all(|char| signal.contains(char));
            if maybe_zero && !signal_to_digit.contains_key(signal) {
                signal_to_digit.insert(signal, 0);
                digit_to_signal.insert(0, signal);
            }
        }

        for signal in &display.0 {
            if signal.len() != 6 {
                continue;
            }

            if !signal_to_digit.contains_key(signal) {
                signal_to_digit.insert(signal, 6);
                digit_to_signal.insert(6, signal);
            }
        }

        for signal in &display.0 {
            if signal.len() != 5 {
                continue;
            }

            let is_five = signal
                .chars()
                .all(|char| digit_to_signal[&6].contains(char));
            if is_five {
                signal_to_digit.insert(signal, 5);
                digit_to_signal.insert(5, signal);
            }
        }

        for signal in &display.0 {
            if signal.len() != 5 {
                continue;
            }

            if !signal_to_digit.contains_key(signal) {
                signal_to_digit.insert(signal, 2);
                digit_to_signal.insert(2, signal);
            }
        }

        let mut value = 0;
        for output in display.1 {
            value *= 10;
            value += signal_to_digit[&output];
        }

        part2 += value;
    }

    println!("{}", part1);
    println!("{}", part2);
}
