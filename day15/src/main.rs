use std::collections::HashMap;
use std::io::BufRead;

use priority_queue::PriorityQueue;


fn best_path_cost(risk_levels: &HashMap<(i32, i32), i32>) -> i32 {
    let max_x = *risk_levels.keys().map(|(x, _y)| x).max().unwrap();
    let max_y = *risk_levels.keys().map(|(_x, y)| y).max().unwrap();

    let mut queue = PriorityQueue::new();
    queue.push((0, 0), -0);

    while let Some((current, risk_so_far)) = queue.pop() {
        if current.0 == max_x && current.1 == max_y {
            return -risk_so_far;
        }

        let offsets = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
        for neighbor in offsets.iter().map(|offset| (current.0 + offset.0, current.1 + offset.1)) {
            if !risk_levels.contains_key(&neighbor) {
                continue; // off grid
            }

            queue.push_increase(neighbor, risk_so_far - risk_levels[&neighbor]);
        }
    }

    -1 // no path
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut risk_levels = HashMap::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, char) in line.chars().enumerate() {
            let risk_level = char.to_digit(10).unwrap() as i32;
            risk_levels.insert((x as i32, y as i32), risk_level);
        }
    }

    let part1 = best_path_cost(&risk_levels);
    println!("{}", part1);

    let offset_x = *risk_levels.keys().map(|(x, _y)| x).max().unwrap() + 1;
    let offset_y = *risk_levels.keys().map(|(_x, y)| y).max().unwrap() + 1;

    let mut risk_levels2 = HashMap::new();
    for y in 0..5 {        
        for x in 0..5 {
            for (position, risk_level) in &risk_levels {
                risk_levels2.insert((x * offset_x + position.0, y * offset_y + position.1), (*risk_level - 1 + x + y) % 9 + 1);
            }
        }   
    }

    let part2 = best_path_cost(&risk_levels2);
    println!("{}", part2);
}
