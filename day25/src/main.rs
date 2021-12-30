use std::io::BufRead;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut seafloor = reader.lines().map(|line| {
        line.unwrap().chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut step = 0;
    loop {
        let mut moved = 0;
        step += 1;

        let mut new_seafloor = seafloor.clone();

        for y in 0..seafloor.len() {
            for x in 0..seafloor[0].len() {
                if seafloor[y][x] == '.' {
                    let origin = if x > 0 { x - 1 } else { seafloor[0].len() - 1 };
                    if seafloor[y][origin] == '>' {
                        new_seafloor[y][x] = '>';
                        new_seafloor[y][origin] = '.';
                        moved += 1;
                    }
                }
            }
        }

        seafloor = new_seafloor;
        new_seafloor = seafloor.clone();

        for y in 0..seafloor.len() {
            for x in 0..seafloor[0].len() {
                if seafloor[y][x] == '.' {
                    let origin = if y > 0 { y - 1 } else { seafloor.len() - 1 };
                    if seafloor[origin][x] == 'v' {
                        new_seafloor[y][x] = 'v';
                        new_seafloor[origin][x] = '.';
                        moved += 1;
                    }
                }
            }
        }

        seafloor = new_seafloor;

        // for y in 0..seafloor.len() {
        //     for x in 0..seafloor[0].len() {
        //         print!("{}", seafloor[y][x]);
        //     }
        //     println!();
        // }
        // println!();

        if moved == 0 {
            let part1 = step;
            println!("{}", part1);
            break;
        }
    }
}
