use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use itertools::Itertools;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut unresolved_scanners = HashMap::new();
    let mut beacons = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains("scanner") {
            continue;
        }

        if line.is_empty() {
            unresolved_scanners.insert(unresolved_scanners.len(), beacons);
            beacons = Vec::new();

            continue;
        }

        let beacon = line
            .split(',')
            .map(|coordinate| coordinate.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        beacons.push(beacon);
    }
    unresolved_scanners.insert(unresolved_scanners.len(), beacons);

    let mut resolved_scanners = HashMap::new();
    let resolved_scanner = unresolved_scanners.remove(&0).unwrap();
    resolved_scanners.insert(0, resolved_scanner);

    let mut resolved_positions = Vec::new();

    while unresolved_scanners.len() > 0 {
        let mut newly_resolved_beacons = HashMap::new();
        for (_resolved_key, resolved_beacons) in &resolved_scanners {
            for (unresolved_key, unresolved_beacons) in &unresolved_scanners {
                for axis_mapping in (0..6).permutations(3) {
                    let mut invalid = false;
                    for i in 0..3 {
                        if axis_mapping.contains(&i) && axis_mapping.contains(&(i + 3)) {
                            invalid = true;
                            break;
                        }
                    }

                    if invalid {
                        continue;
                    }

                    let mapped_unresolved_beacons = unresolved_beacons
                        .iter()
                        .map(|beacon| {
                            (0..3)
                                .map(|i| {
                                    if axis_mapping[i] < 3 {
                                        beacon[axis_mapping[i]]
                                    } else {
                                        -beacon[axis_mapping[i] - 3]
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();

                    let mut candidate_positions = HashMap::new();
                    for resolved_beacon in resolved_beacons {
                        for unresolved_beacon in &mapped_unresolved_beacons {
                            let position = resolved_beacon
                                .iter()
                                .zip(unresolved_beacon.iter())
                                .map(|(r, u)| r - u)
                                .collect::<Vec<_>>();
                            *candidate_positions.entry(position).or_insert(0) += 1;
                        }
                    }

                    let best_position = candidate_positions
                        .into_iter()
                        .max_by(|(_, a), (_, b)| a.cmp(b))
                        .unwrap();
                    if best_position.1 >= 12 {
                        let mapped_resolved_beacons = unresolved_beacons
                            .iter()
                            .map(|beacon| {
                                (0..3)
                                    .map(|i| {
                                        if axis_mapping[i] < 3 {
                                            best_position.0[i] + beacon[axis_mapping[i]]
                                        } else {
                                            best_position.0[i] - beacon[axis_mapping[i] - 3]
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>();

                        resolved_positions.push(best_position.0);

                        newly_resolved_beacons.insert(*unresolved_key, mapped_resolved_beacons);
                        break;
                    }
                }
            }
        }

        for (key, beacons) in newly_resolved_beacons {
            unresolved_scanners.remove(&key);
            resolved_scanners.insert(key, beacons);
        }
    }

    let mut all_beacons = HashSet::new();
    for beacons in resolved_scanners.values() {
        all_beacons.extend(beacons);
    }

    let part1 = all_beacons.len();
    println!("{}", part1);

    let mut part2 = 0;
    for position_a in &resolved_positions {
        for position_b in &resolved_positions {
            let manhattan_distance = position_a
                .iter()
                .zip(position_b.iter())
                .fold(0, |accu, (a, b)| accu + (a - b).abs());

            part2 = part2.max(manhattan_distance);
        }
    }

    println!("{}", part2);
}
