use std::{collections::HashMap, io::BufRead};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let mut lines = std::io::BufReader::new(file).lines();

    let enhancement_algorithm = lines.next().unwrap().unwrap();
    lines.next();

    let mut input_image = HashMap::new();
    for (y, line) in lines.enumerate() {
        for (x, pixel) in line.unwrap().char_indices() {
            input_image.insert((x as i32, y as i32), pixel);
        }
    }

    let offsets = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut output_image = HashMap::new();
    let mut default_pixel = '.';
    for i in 0..50 {
        let min_x = input_image.keys().map(|pixel| pixel.0).min().unwrap() - 1;
        let max_x = input_image.keys().map(|pixel| pixel.0).max().unwrap() + 1;
        let min_y = input_image.keys().map(|pixel| pixel.1).min().unwrap() - 1;
        let max_y = input_image.keys().map(|pixel| pixel.1).max().unwrap() + 1;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let binary_representation = offsets
                    .iter()
                    .map(|offset| {
                        let pixel = (x + offset.0, y + offset.1);

                        if input_image.contains_key(&pixel) {
                            input_image[&pixel]
                        } else {
                            default_pixel
                        }
                    })
                    .fold(0usize, |accu, pixel| {
                        let mut accu = accu * 2;

                        if pixel == '#' {
                            accu += 1;
                        }

                        accu
                    });

                let enhanced_pixel = enhancement_algorithm
                    .chars()
                    .nth(binary_representation)
                    .unwrap();

                output_image.insert((x, y), enhanced_pixel);
            }
        }

        input_image = output_image;
        output_image = HashMap::new();
        default_pixel = if default_pixel == '.' {
            enhancement_algorithm.chars().next().unwrap()
        } else {
            enhancement_algorithm.chars().last().unwrap()
        };

        if i == 1 {
            let part1 = input_image.values().filter(|pixel| **pixel == '#').count();
            println!("{}", part1);
        }    
    }

    let part1 = input_image.values().filter(|pixel| **pixel == '#').count();
    println!("{}", part1);
}
