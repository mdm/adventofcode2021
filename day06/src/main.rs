use std::{collections::HashMap, io::BufRead};

fn total_descendants(days_remaining: i32, memo: &mut HashMap<i32, u64>) -> u64 {
    if memo.contains_key(&days_remaining) {
        return memo[&days_remaining];
    }

    let mut descendants = 1;

    if days_remaining <= 0 {
        memo.insert(days_remaining, descendants);
        return descendants;
    }

    let mut tmp_days_remaining = days_remaining;
    descendants += 1;
    while tmp_days_remaining > 7 {
        descendants += total_descendants(tmp_days_remaining - 9, memo);
        tmp_days_remaining -= 7;
    }

    memo.insert(days_remaining, descendants);
    return descendants;
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let line = std::io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let original_fish_popoluation = line
        .split(',')
        .map(|timer| timer.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut fish_popoluation = original_fish_popoluation.clone();
    for _ in 0..80 {
        let new_fish = fish_popoluation.iter().filter(|timer| **timer == 0).count();

        fish_popoluation = fish_popoluation
            .iter()
            .map(|timer| if *timer == 0 { 6 } else { timer - 1 })
            .collect();

        for _ in 0..new_fish {
            fish_popoluation.push(8);
        }
    }

    let part1 = fish_popoluation.len();
    println!("{}", part1);

    let mut memo = HashMap::new();
    let part2 = original_fish_popoluation
        .iter()
        .map(|timer| total_descendants(256 - timer, &mut memo))
        .sum::<u64>();
    println!("{}", part2);
}
