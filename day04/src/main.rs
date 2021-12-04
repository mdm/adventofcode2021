use std::io::BufRead;

fn winning(boards: &Vec<Vec<Vec<(usize, bool)>>>) -> Option<usize> {
    let winning_boards = boards.iter().enumerate().filter(|(_, board)| {
        let has_winning_row = board.iter().any(|row| row.iter().all(|number| number.1));

        let has_winning_column = (0..5)
            .map(|i| board.iter().map(|row| row[i]).collect::<Vec<_>>())
            .any(|column| column.iter().all(|number| number.1));

        has_winning_row || has_winning_column
    });

    winning_boards.map(|(i, _)| i).next()
}

fn sum_unmarked(board: &Vec<Vec<(usize, bool)>>) -> usize {
    let mut result = 0;
    for x in 0..5 {
        for y in 0..5 {
            if !board[y][x].1 {
                result += board[y][x].0;
            }
        }
    }

    result
}

#[allow(dead_code)]
fn print_board(board: &Vec<Vec<(usize, bool)>>) {
    for x in 0..5 {
        for y in 0..5 {
            if board[y][x].1 {
                print!(" X");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let mut lines = std::io::BufReader::new(file).lines();

    let numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = Vec::new();
    while let Some(_) = lines.next() {
        let mut board = Vec::new();
        for _ in 0..5 {
            let row = lines
                .next()
                .unwrap()
                .unwrap()
                .split(' ')
                .filter(|number| !number.is_empty())
                .map(|number| (number.parse::<usize>().unwrap(), false))
                .collect::<Vec<_>>();

            board.push(row);
        }

        boards.push(board);
    }

    let mut last_score = None;
    for number in numbers {
        for i in 0..boards.len() {
            for x in 0..5 {
                for y in 0..5 {
                    if boards[i][y][x].0 == number {
                        boards[i][y][x].1 = true;
                    }
                }
            }
        }

        while let Some(i) = winning(&boards) {
            let score = sum_unmarked(&boards[i]) * number;

            if last_score.is_none() {
                let part1 = score;
                println!("{}", part1);
            }

            last_score = Some(score);
            boards.remove(i);
        }
    }

    if let Some(score) = last_score {
        let part2 = score;
        println!("{}", part2);
    }
}
