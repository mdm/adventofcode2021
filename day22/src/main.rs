use std::{collections::HashSet, io::BufRead};

#[derive(Debug, Clone)]
struct Cuboid {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Cuboid {
    fn volume(&self) -> i64 {
        let width = self.max_x - self.min_x + 1;
        let height = self.max_y - self.min_y + 1;
        let depth = self.max_z - self.min_z + 1;

        width * height * depth
    }
}

fn split(keep: &Cuboid, discard: &Cuboid) -> Vec<Cuboid> {
    // split "keep" into 9 parts and return those outside the intersection
    // visualize the case where "discard" is completely contained in "keep"

    let mut cuboids = Vec::new();

    let inner = Cuboid {
        min_x: keep.min_x.max(discard.min_x),
        max_x: keep.max_x.min(discard.max_x),
        min_y: keep.min_y.max(discard.min_y),
        max_y: keep.max_y.min(discard.max_y),
        min_z: keep.min_z.max(discard.min_z),
        max_z: keep.max_z.min(discard.max_z),
    };

    // dbg!(&inner);

    for z in 0..3 {
        for y in 0..3 {
            for x in 0..3 {
                let min_x = match x {
                    0 => keep.min_x,
                    1 => inner.min_x,
                    2 => inner.max_x + 1,
                    _ => unreachable!(),
                };

                let max_x = match x {
                    0 => inner.min_x - 1,
                    1 => inner.max_x,
                    2 => keep.max_x,
                    _ => unreachable!(),
                };

                let min_y = match y {
                    0 => keep.min_y,
                    1 => inner.min_y,
                    2 => inner.max_y + 1,
                    _ => unreachable!(),
                };

                let max_y = match y {
                    0 => inner.min_y - 1,
                    1 => inner.max_y,
                    2 => keep.max_y,
                    _ => unreachable!(),
                };

                let min_z = match z {
                    0 => keep.min_z,
                    1 => inner.min_z,
                    2 => inner.max_z + 1,
                    _ => unreachable!(),
                };

                let max_z = match z {
                    0 => inner.min_z - 1,
                    1 => inner.max_z,
                    2 => keep.max_z,
                    _ => unreachable!(),
                };

                let cuboid = Cuboid {
                    min_x,
                    max_x,
                    min_y,
                    max_y,
                    min_z,
                    max_z,
                };

                cuboids.push(cuboid);
            }
        }
    }

    cuboids
        .into_iter()
        .filter(|cuboid| {
            let invalid = cuboid.min_x > cuboid.max_x
                || cuboid.min_y > cuboid.max_y
                || cuboid.min_z > cuboid.max_z;

            let intersect_x = cuboid.max_x >= discard.min_x && discard.max_x >= cuboid.min_x;
            let intersect_y = cuboid.max_y >= discard.min_y && discard.max_y >= cuboid.min_y;
            let intersect_z = cuboid.max_z >= discard.min_z && discard.max_z >= cuboid.min_z;

            let intersect = intersect_x && intersect_y && intersect_z;

            // dbg!(&cuboid, invalid, intersect);
            !invalid && !intersect
        })
        .collect()
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

    let part1 = on_cubes
        .iter()
        .filter(|(x, y, z)| *x >= -50 && *x <= 50 && *y >= -50 && *y <= 50 && *z >= -50 && *z <= 50)
        .count();
    println!("{}", part1);

    let mut current_on_cuboids: Vec<Cuboid> = Vec::new();
    let mut next_on_cuboids: Vec<Cuboid> = Vec::new();
    for (on, new) in reboot_steps {
        for old in current_on_cuboids {
            let intersect_x = new.max_x >= old.min_x && old.max_x >= new.min_x;
            let intersect_y = new.max_y >= old.min_y && old.max_y >= new.min_y;
            let intersect_z = new.max_z >= old.min_z && old.max_z >= new.min_z;

            if intersect_x && intersect_y && intersect_z {
                let parts = split(&old, &new);
                next_on_cuboids.extend(parts);
            } else {
                next_on_cuboids.push(old);
            }
        }

        if on {
            next_on_cuboids.push(new);
        }

        current_on_cuboids = next_on_cuboids;
        next_on_cuboids = Vec::new();
    }

    let part2 = current_on_cuboids
        .iter()
        .map(|cuboid| cuboid.volume())
        .sum::<i64>();
    println!("{}", part2);
}
