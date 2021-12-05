use std::{collections::HashMap, io::BufRead};

#[derive(Debug)]
struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

impl Line {
    fn from_string(s: String) -> Line {
        let points = s
            .split(" -> ")
            .map(|point| {
                point
                    .split(',')
                    .map(|coordinate| coordinate.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Line {
            start_x: points[0][0],
            start_y: points[0][1],
            end_x: points[1][0],
            end_y: points[1][1],
        }
    }
}

fn mark_line(line: &Line, grid: &mut HashMap<(i32, i32), i32>) {
    let step_x = if line.start_x > line.end_x { -1 } else { 1 };
    let step_y = if line.start_y > line.end_y { -1 } else { 1 };

    let length_x = (line.end_x - line.start_x).abs();
    let length_y = (line.end_y - line.start_y).abs();

    for y in 0..=length_y {
        for x in 0..=length_x {
            let on_vertical = line.start_x == line.end_x;
            let on_horizontal = line.start_y == line.end_y;
            let on_diagonal = x == y;

            if on_vertical || on_horizontal || on_diagonal {
                let x = line.start_x + x * step_x;
                let y = line.start_y + y * step_y;
                *grid.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| Line::from_string(line.unwrap()))
        .collect::<Vec<_>>();

    let mut grid1 = HashMap::new();
    for line in lines
        .iter()
        .filter(|line| line.start_x == line.end_x || line.start_y == line.end_y)
    {
        mark_line(line, &mut grid1);
    }

    let part1 = grid1.values().filter(|point| **point >= 2).count();
    println!("{}", part1);

    let mut grid2 = HashMap::new();
    for line in lines.iter() {
        mark_line(line, &mut grid2);
    }

    let part2 = grid2.values().filter(|point| **point >= 2).count();
    println!("{}", part2);
}
