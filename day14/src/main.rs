use std::collections::HashMap;
use std::io::BufRead;

fn get_counts_recursive(
    start: &str,
    steps_to_go: usize,
    memo: &mut HashMap<(char, char, usize), HashMap<char, usize>>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<char, usize> {
    if steps_to_go == 0 {
        let mut counts = HashMap::new();
        for char in start.chars() {
            *counts.entry(char).or_insert(0) += 1;
        }

        return counts;
    }

    let mut total_counts: HashMap<char, usize> = HashMap::new();
    for pair in start.chars().zip(start.chars().skip(1)) {
        if !memo.contains_key(&(pair.0, pair.1, steps_to_go - 1)) {
            let next_start = vec![pair.0, rules[&pair], pair.1]
                .into_iter()
                .collect::<String>();

            let counts = get_counts_recursive(&next_start, steps_to_go - 1, memo, rules);
            memo.insert((pair.0, pair.1, steps_to_go - 1), counts);
        }

        for (char, count) in &memo[&(pair.0, pair.1, steps_to_go - 1)] {
            *total_counts.entry(*char).or_insert(0) += count;
        }

        *total_counts.entry(pair.1).or_insert(0) -= 1;
    }

    let last = start.chars().last().unwrap();
    *total_counts.entry(last).or_insert(0) += 1;

    total_counts
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut lines = reader.lines();

    let start = lines.next().unwrap().unwrap();
    lines.next();

    let mut rules = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let mut split = line.split(" -> ");
        let lhs = split.next().unwrap().chars().collect::<Vec<_>>();
        let rhs = split.next().unwrap().chars().nth(0).unwrap();
        rules.insert((lhs[0], lhs[1]), rhs);
    }

    let mut polymer = start.clone();
    for _ in 0..10 {
        let mut new_polymer = Vec::new();
        for pair in polymer.chars().zip(polymer.chars().skip(1)) {
            new_polymer.push(pair.0);
            new_polymer.push(rules[&pair]);
        }

        new_polymer.push(polymer.chars().last().unwrap());
        polymer = new_polymer.iter().collect::<String>();
    }

    let mut counts = HashMap::new();
    for char in polymer.chars() {
        *counts.entry(char).or_insert(0) += 1;
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    let part1 = max - min;
    println!("{}", part1);

    let counts = get_counts_recursive(&start, 40, &mut HashMap::new(), &rules);

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    let part2 = max - min;
    println!("{}", part2);
}
