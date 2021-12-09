use std::{collections::{HashSet, VecDeque}, io::BufRead};

fn neighbors(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)>{
    let mut result = Vec::new();

    if y > 0 {
        result.push((x, y - 1));
    }

    if x < w - 1 {
        result.push((x + 1, y));
    }

    if y < h - 1 {
        result.push((x, y + 1));
    }

    if x > 0 {
        result.push((x - 1, y));
    }

    result
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let heights = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut low_points = Vec::new();
    for (y, row) in heights.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let low_point = neighbors(x, y, row.len(), heights.len())
                .into_iter()
                .map(|(neighbor_x, neighbor_y)| heights[neighbor_y][neighbor_x])
                .all(|neighbor_height| *height < neighbor_height);
            if low_point {
                part1 += height + 1;
                low_points.push((x, y));
            }
        }
    }

    println!("{}", part1);

    let mut basin_sizes = low_points.into_iter().map(|start| {
        let mut queue= VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        visited.insert(start);

        let mut basin_size = 0;
        while let Some(current) = queue.pop_front() {
            basin_size += 1;
            
            for neighbor in neighbors(current.0, current.1, heights[0].len(), heights.len()) {
                if heights[neighbor.1][neighbor.0] < 9 && heights[current.1][current.0] <= heights[neighbor.1][neighbor.0] && !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                }
            }
        }

        basin_size
    }).collect::<Vec<_>>();
    
    basin_sizes.sort();
    basin_sizes.reverse();
    let part2 = basin_sizes.iter().take(3).product::<u32>();

    println!("{}", part2);
}
