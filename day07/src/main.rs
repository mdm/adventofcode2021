use std::io::BufRead;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let line = std::io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let crab_positions = line
        .split(',')
        .map(|position| position.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let min_position = *crab_positions.iter().min().unwrap();
    let max_position = *crab_positions.iter().max().unwrap();

    let mut min_fuel = None;
    for destination in min_position..=max_position {
        let fuel = crab_positions
            .iter()
            .map(|position| (destination as i32 - *position as i32).abs() as usize)
            .sum::<usize>();

        min_fuel = match min_fuel {
            Some(old_fuel) => {
                if fuel < old_fuel {
                    Some(fuel)
                } else {
                    Some(old_fuel)
                }
            }
            None => Some(fuel),
        };
    }

    let part1 = min_fuel.unwrap();
    println!("{}", part1);

    let mut min_fuel = None;
    for destination in min_position..=max_position {
        let fuel = crab_positions
            .iter()
            .map(|position| {
                let n = (destination as i32 - *position as i32).abs();
                (n * (n + 1) / 2) as usize
            })
            .sum::<usize>();

        min_fuel = match min_fuel {
            Some(old_fuel) => {
                if fuel < old_fuel {
                    Some(fuel)
                } else {
                    Some(old_fuel)
                }
            }
            None => Some(fuel),
        };
    }

    let part2 = min_fuel.unwrap();
    println!("{}", part2);
}
