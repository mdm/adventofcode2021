use std::{collections::HashSet, io::BufRead};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let reboot_steps = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let re = regex::Regex::new(r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$").unwrap();
            let captures = re.captures(&line).unwrap();

            let on = captures.get(1).unwrap().as_str() == "on";
            let min_x = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let max_x = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let min_y = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let max_y = captures.get(5).unwrap().as_str().parse::<i64>().unwrap();
            let min_z = captures.get(6).unwrap().as_str().parse::<i64>().unwrap();
            let max_z = captures.get(7).unwrap().as_str().parse::<i64>().unwrap();

            (on, (min_x, max_x), (min_y, max_y), (min_z, max_z))        
        }).collect::<Vec<_>>();

    let mut on_cubes = HashSet::new();
    for (on, (min_x, max_x), (min_y, max_y), (min_z, max_z)) in reboot_steps {
        let min_x = min_x.max(-50);
        let max_x = max_x.min(50);
        let min_y = min_y.max(-50);
        let max_y = max_y.min(50);
        let min_z = min_z.max(-50);
        let max_z = max_z.min(50);

        for z in min_z..=max_z {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if on {
                        on_cubes.insert((x, y, z));
                    } else {
                        on_cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    dbg!(on_cubes.len());

    let part1 = on_cubes.iter().filter(|(x, y, z)| {
        *x >= -50 && *x <= 50 && *y >= -50 && *y <= 50 && *z >= -50 && *z <= 50
    }).count();
    println!("{}", part1);
}
