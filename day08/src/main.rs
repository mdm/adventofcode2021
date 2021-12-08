use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

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
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let output = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            (signals, output)
        })
        .collect::<Vec<_>>();

    dbg!(&displays);

    let digits = vec![
        "abcefg",  // 0
        "cf",      // 1
        "acdeg",   // 2
        "acdfg",   // 3
        "bcdf",    // 4
        "abdfg",   // 5
        "abdefg",  // 6
        "acf",     // 7
        "abcdefg", // 8
        "abcdfg",  // 9
    ];

    let mut part1 = 0;
    for display in displays {
        let mut fixed_digits = HashMap::new();
        // while fixed_digits.len() < 10 {
        for _ in 0..1 {
            let mut filtered_candidates: HashMap<char, HashSet<char>> = HashMap::new();
            for signal in &display.0 {
                for digit in digits.iter().filter(|digit| digit.len() == signal.len()) {
                    for char in signal.chars() {
                        let candidates = digit.chars().filter(|char| !fixed_digits.contains_key(char)).collect::<HashSet<char>>();
                        if filtered_candidates.contains_key(&char) {
                            let new_candidates = filtered_candidates[&char]
                                .intersection(&candidates)
                                .copied()
                                .collect::<HashSet<char>>();
                            filtered_candidates.insert(char, new_candidates);
                        } else {
                            filtered_candidates.insert(char, candidates);
                        }
                    }
                }

                // dbg!(&filtered_candidates);
            }
            fixed_digits.insert('a', 'd');
            dbg!("final", &filtered_candidates);

        }
    }
}
