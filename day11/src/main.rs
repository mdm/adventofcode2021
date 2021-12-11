use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut energy_levels = HashMap::new();

    for (y, line) in reader
        .lines()
        .enumerate()
        .map(|(y, line)| (y, line.unwrap()))
    {
        for (x, energy_level) in line
            .chars()
            .enumerate()
            .map(|(x, char)| (x, char.to_digit(10).unwrap()))
        {
            energy_levels.insert((x as i32, y as i32), energy_level);
        }
    }

    let mut part1 = 0;
    let mut step = 0;
    loop {
        let mut flashed_this_step = HashSet::new();
        for (_, energy_level) in energy_levels.iter_mut() {
            *energy_level += 1;
        }

        loop {
            let ready_to_flash = energy_levels
                .keys()
                .filter(|octopus| energy_levels[octopus] > 9)
                .copied()
                .collect::<Vec<_>>();

            let mut done = true;
            for octopus in ready_to_flash {
                if flashed_this_step.contains(&octopus) {
                    continue;
                }

                flashed_this_step.insert(octopus);

                if step < 100 {
                    part1 += 1;
                }

                done = false;

                let potential_neighbors = vec![
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ];
                let neighbors = potential_neighbors
                .iter()
                .map(|offset| (octopus.0 + offset.0, octopus.1 + offset.1));

                for neighbor in neighbors {
                    if !energy_levels.contains_key(&neighbor) {
                        continue;
                    }

                    if let Some(energy_level) = energy_levels.get_mut(&neighbor) {
                        *energy_level += 1;
                    }            
                }
            }

            if done {
                break;
            }
        }

        for (octopus, energy_level) in energy_levels.iter_mut() {
            if flashed_this_step.contains(octopus) {
                *energy_level = 0;                
            }
        }

        // for y in 0..10 {
        //     for x in 0..10 {
        //         print!("{}", energy_levels[&(x, y)]);
        //     }
        //     println!();
        // }
        // println!();

        step += 1;

        if flashed_this_step.len() == energy_levels.len() {
            let part2 = step;

            println!("{}", part1);
            println!("{}", part2);
            break;
        }
    }
}
