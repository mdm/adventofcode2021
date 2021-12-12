use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

fn count_paths(map: &HashMap<String, Vec<String>>, start: &str, mut visited: HashSet<String>, allow_repeat: bool) -> usize {
    if start == "end" {
        return 1;
    }

    visited.insert(start.to_string());

    let mut paths = 0;

    for neighbor in map[start].iter() {
        let is_big = neighbor.chars().next().unwrap().is_ascii_uppercase();
        if !visited.contains(neighbor) || is_big {
            paths += count_paths(map, neighbor, visited.clone(), allow_repeat);
        } else {
            if neighbor != "start" && allow_repeat {
                paths += count_paths(map, neighbor, visited.clone(), false);
            }
        }
    }

    paths
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut map = HashMap::new();

    for line in reader.lines().map(|line| line.unwrap()) {
        let caves = line
            .split('-')
            .map(|cave| cave.to_string())
            .collect::<Vec<_>>();

        map.entry(caves[0].clone()).or_insert(Vec::new()).push(caves[1].clone());
        map.entry(caves[1].clone()).or_insert(Vec::new()).push(caves[0].clone());
    }

    let part1 = count_paths(&map, "start", HashSet::new(), false);
    println!("{}", part1);

    let part2 = count_paths(&map, "start", HashSet::new(), true);
    println!("{}", part2);
}
