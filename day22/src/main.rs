use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Clone)]
struct Cuboid {
    id: usize,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

fn split(keep: &Cuboid, discard: &Cuboid) -> Vec<Cuboid> {
    // split "keep" into 9 parts and return those outside the intersection
    // visualize the case where "discard" is completely contained in "keep"

    let mut cuboids = Vec::new();

    let inner_min_x = keep.min_x.max(discard.min_x);
    let inner_max_x = keep.max_x.min(discard.max_x);
    let inner_min_y = keep.min_y.max(discard.min_y);
    let inner_max_y = keep.max_y.min(discard.max_y);
    let inner_min_z = keep.min_z.max(discard.min_z);
    let inner_max_z = keep.max_z.min(discard.max_z);

    if keep.min_x < inner_min_x && keep.min_y < inner_min_y && keep.min_z < inner_min_z {
        let cuboid = Cuboid {
            id: 1,
            min_x: keep.min_x,
            max_x: inner_min_x - 1,
            min_y: keep.min_y,
            max_y: inner_min_y - 1,
            min_z: keep.min_z,
            max_z: inner_min_z - 1,
        };

        cuboids.push(cuboid);
    }


    if keep.min_y < inner_min_y && keep.min_z < inner_min_z {
        let cuboid = Cuboid {
            id: 2,
            min_x: inner_min_x,
            max_x: inner_max_x,
            min_y: keep.min_y,
            max_y: inner_min_y - 1,
            min_z: keep.min_z,
            max_z: inner_min_z - 1,
        };

        cuboids.push(cuboid);
    }


    if keep.max_x > inner_max_x && keep.min_y < inner_min_y && keep.min_z < inner_min_z {
        let cuboid = Cuboid {
            id: 3,
            min_x: inner_max_x + 1,
            max_x: keep.max_x,
            min_y: keep.min_y,
            max_y: inner_min_y - 1,
            min_z: keep.min_z,
            max_z: inner_min_z - 1,
        };

        cuboids.push(cuboid);
    }

    unimplemented!()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let reboot_steps = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let re = regex::Regex::new(
                r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$",
            )
            .unwrap();
            let captures = re.captures(&line).unwrap();

            let on = captures.get(1).unwrap().as_str() == "on";
            let min_x = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let max_x = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let min_y = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let max_y = captures.get(5).unwrap().as_str().parse::<i64>().unwrap();
            let min_z = captures.get(6).unwrap().as_str().parse::<i64>().unwrap();
            let max_z = captures.get(7).unwrap().as_str().parse::<i64>().unwrap();

            (
                on,
                Cuboid {
                    id: 0,
                    min_x,
                    max_x,
                    min_y,
                    max_y,
                    min_z,
                    max_z,
                },
            )
        })
        .collect::<Vec<_>>();

    let mut on_cubes = HashSet::new();
    for (on, cuboid) in &reboot_steps {
        let min_x = cuboid.min_x.max(-50);
        let max_x = cuboid.max_x.min(50);
        let min_y = cuboid.min_y.max(-50);
        let max_y = cuboid.max_y.min(50);
        let min_z = cuboid.min_z.max(-50);
        let max_z = cuboid.max_z.min(50);

        for z in min_z..=max_z {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if *on {
                        on_cubes.insert((x, y, z));
                    } else {
                        on_cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    dbg!(on_cubes.len());

    let part1 = on_cubes
        .iter()
        .filter(|(x, y, z)| *x >= -50 && *x <= 50 && *y >= -50 && *y <= 50 && *z >= -50 && *z <= 50)
        .count();
    println!("{}", part1);

    let mut current_on_cuboids: Vec<Cuboid> = Vec::new();
    let mut next_on_cuboids: Vec<Cuboid> = Vec::new();
    for (on, new) in reboot_steps {
        let mut intersections = 0;

        for old in current_on_cuboids {
            let intersect_x = new.max_x >= old.min_x && old.max_x >= new.min_x;
            let intersect_y = new.max_y >= old.min_y && old.max_y >= new.min_y;
            let intersect_z = new.max_z >= old.min_z && old.max_z >= new.min_z;

            if intersect_x && intersect_y && intersect_z {
                intersections += 1;

                let old_contains_new_x = new.min_x >= old.min_x && new.max_x <= old.max_x;
                let old_contains_new_y = new.min_y >= old.min_y && new.max_y <= old.max_y;
                let old_contains_new_z = new.min_z >= old.min_z && new.max_z <= old.max_z;
                let old_contains_new = old_contains_new_x && old_contains_new_y && old_contains_new_z;

                let new_contains_old_x = old.min_x >= new.min_x && old.max_x <= new.max_x;
                let new_contains_old_y = old.min_y >= new.min_y && old.max_y <= new.max_y;
                let new_contains_old_z = old.min_z >= new.min_z && old.max_z <= new.max_z;
                let new_contains_old = new_contains_old_x && new_contains_old_y && new_contains_old_z;

                dbg!(old_contains_new, new_contains_old);
                println!();

                if on {
                    next_on_cuboids.extend(split(&new, &old));
                    next_on_cuboids.push(old);
                } else {
                    next_on_cuboids.extend(split(&old, &new));
                }
            }            
        }

        if intersections == 0 {
            next_on_cuboids.push(new);
        }

        current_on_cuboids = next_on_cuboids;
        next_on_cuboids = Vec::new();
    }

    let part2 = current_on_cuboids.iter().map(|cuboid| {
        let width = cuboid.max_x - cuboid.min_x + 1;
        let height = cuboid.max_y - cuboid.min_y + 1;
        let depth = cuboid.max_z - cuboid.min_z + 1;

        width * height * depth
    }).sum::<i64>();
    println!("{}", part1);
}
