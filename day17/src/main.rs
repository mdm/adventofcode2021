use std::{collections::HashSet, io::BufRead};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let line = std::io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let re = regex::Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
    let caps = re.captures(&line).unwrap();

    let target_min_x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let target_max_x = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let target_min_y = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let target_max_y = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();    

    let mut part1 = 0;
    let mut on_target = HashSet::new();
    for initial_velocity_x in 1..=target_max_x {
        for initial_velocity_y in target_min_y..=(-target_min_y) {
            let mut position_x = 0;
            let mut position_y = 0;
            let mut velocity_x = initial_velocity_x;
            let mut velocity_y = initial_velocity_y;

            while position_x <= target_max_x && position_y >= target_min_y {
                if position_x >= target_min_x && position_y <= target_max_y {
                    let max_y = initial_velocity_y * (initial_velocity_y + 1) / 2;
                    if max_y > part1 {
                        part1 = max_y;
                    }

                    on_target.insert((initial_velocity_x, initial_velocity_y));

                    break;
                }

                position_x += velocity_x;
                position_y += velocity_y;

                if velocity_x > 0 {
                    velocity_x -= 1;
                }

                velocity_y -= 1;
            }
        }
    }

    println!("{}", part1);

    let part2 = on_target.len();
    println!("{}", part2);
}
