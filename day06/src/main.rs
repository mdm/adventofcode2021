use std::io::BufRead;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let line = std::io::BufReader::new(file).lines().next().unwrap().unwrap();

    let mut fish_popoluation = line.split(',')
        .map(|timer| timer.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..80 {
        let new_fish = fish_popoluation.iter().filter(|timer| **timer == 0).count();

        fish_popoluation = fish_popoluation.iter().map(|timer | if *timer == 0 { 6 } else { timer - 1 }).collect();
        
        for _ in 0..new_fish {
            fish_popoluation.push(8);
        }
    }

    let part1 = fish_popoluation.len();
    println!("{}", part1);
}
