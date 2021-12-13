use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug)]
enum Fold {
    Left(i32),
    Up(i32),
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut dots = HashSet::new();
    let mut folds = Vec::new();

    let mut reading_dots = true;
    for line in reader.lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            reading_dots = false;
            continue;
        }

        if reading_dots {
            let mut coordinates = line.split(',').map(|coordinate| coordinate.parse::<i32>().unwrap());
            dots.insert((coordinates.next().unwrap(), coordinates.next().unwrap()));
        } else {
            let mut fold_spec = line.split(' ').last().unwrap().split('=');
            let axis = fold_spec.next().unwrap();
            let offset = fold_spec.next().unwrap().parse::<i32>().unwrap();

            let fold = match axis {
                "x" => Fold::Left(offset),
                "y" => Fold::Up(offset),
                _ => unreachable!(),
            };

            folds.push(fold);
        }
    }

    for (i, fold) in folds.iter().enumerate() {
        match fold {
            Fold::Left(offset) => {
                let (affected, unaffected): (HashSet<(i32, i32)>, HashSet<(i32, i32)>) = dots.iter().partition(|dot| dot.0 > *offset);
                dots = unaffected;
                for (x, y) in affected {
                    dots.insert((-(x - offset) + offset, y));
                }
            }
            Fold::Up(offset) => {
                let (affected, unaffected): (HashSet<(i32, i32)>, HashSet<(i32, i32)>) = dots.iter().partition(|dot| dot.1 > *offset);
                dots = unaffected;
                for (x, y) in affected {
                    dots.insert((x , -(y - offset) + offset));
                }
            }
        }

        if i == 0 {
            let part1 = dots.len();
            println!("{}", part1);
        }
    }

    let min_x = dots.iter().map(|dot| dot.0).min().unwrap();
    let max_x = dots.iter().map(|dot| dot.0).max().unwrap();
    let min_y = dots.iter().map(|dot| dot.1).min().unwrap();
    let max_y = dots.iter().map(|dot| dot.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
