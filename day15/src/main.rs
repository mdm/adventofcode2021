use std::{io::BufRead, collections::HashMap};

use priority_queue::PriorityQueue;

fn best_path_cost(risk_levels: &Vec<Vec<i32>>) -> i32 {
    let max_x = risk_levels[0].len() - 1;
    let max_y = risk_levels.len() - 1;

    let mut queue = PriorityQueue::new();
    queue.push((0, 0), -0);

    let mut visited = HashMap::new();
    visited.insert((0, 0), -0);

    while let Some((current, risk_so_far)) = queue.pop() {
        if current.0 == max_x && current.1 == max_y {
            return -risk_so_far;
        }

        if current.1 > 0 {
            let next = (current.0, current.1 - 1);
            if !visited.contains_key(&next) || visited[&next] < risk_so_far - risk_levels[next.1][next.0] {
                visited.insert(next, risk_so_far - risk_levels[next.1][next.0]);
                queue.push_increase(next, risk_so_far - risk_levels[next.1][next.0]);
            }
        }

        if current.0 < max_x {
            let next = (current.0 + 1, current.1);
            if !visited.contains_key(&next) || visited[&next] < risk_so_far - risk_levels[next.1][next.0] {
                visited.insert(next, risk_so_far - risk_levels[next.1][next.0]);
                queue.push_increase(next, risk_so_far - risk_levels[next.1][next.0]);
            }
        }

        if current.1 < max_y {
            let next = (current.0, current.1 + 1);
            if !visited.contains_key(&next) || visited[&next] < risk_so_far - risk_levels[next.1][next.0] {
                visited.insert(next, risk_so_far - risk_levels[next.1][next.0]);
                queue.push_increase(next, risk_so_far - risk_levels[next.1][next.0]);
            }
        }

        if current.0 > 0 {
            let next = (current.0 - 1, current.1);
            if !visited.contains_key(&next) || visited[&next] < risk_so_far - risk_levels[next.1][next.0] {
                visited.insert(next, risk_so_far - risk_levels[next.1][next.0]);
                queue.push_increase(next, risk_so_far - risk_levels[next.1][next.0]);
            }
        }
    }

    -1 // no path
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let risk_levels = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = best_path_cost(&risk_levels);
    println!("{}", part1);

    let offset_x = risk_levels[0].len();
    let offset_y = risk_levels.len();

    let mut risk_levels2 = Vec::new();
    for tile_y in 0..5 {
        for y in 0..offset_y {
            let mut row = Vec::new();
            for tile_x in 0..5 {
                for x in 0..offset_x {
                    row.push((risk_levels[y][x] - 1 + tile_x + tile_y) % 9 + 1)
                }
            }

            risk_levels2.push(row);
        }
    }

    let part2 = best_path_cost(&risk_levels2);
    println!("{}", part2);
}
