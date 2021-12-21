use std::{io::BufRead, collections::HashMap};


fn dirac_wins_recursive(player: usize, rolls_remaining: usize, positions: Vec<i64>, scores: Vec<i64>, memo: &mut HashMap<(usize, usize, i64, i64, i64, i64), (i64, i64)>) -> (i64, i64) {
    let state = (player, rolls_remaining, positions[0], positions[1], scores[0], scores[1]);
    if memo.contains_key(&state) {
        return memo[&state];
    }

    let mut total_wins = (0, 0);
    for dice_roll in 0..3 {
        let mut new_scores = scores.clone();
        let mut new_positions = positions.clone();

        new_positions[player] += dice_roll + 1;
        new_positions[player] %= 10;

        let wins_from_here = if rolls_remaining > 1 {
            dirac_wins_recursive(player, rolls_remaining - 1, new_positions, new_scores, memo)
        } else {
            new_scores[player] -= new_positions[player] + 1;

            if new_scores[player] <= 0 {
                (1 - player as i64, player as i64)
            } else {
                dirac_wins_recursive(1 - player, 3, new_positions, new_scores, memo)
            }
        };

        total_wins = (total_wins.0 + wins_from_here.0, total_wins.1 + wins_from_here.1);
    }

    memo.insert(state, total_wins);
    total_wins
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let starting_positions = reader.lines().map(|line| {
        line.unwrap().split(' ').last().unwrap().parse::<i64>().unwrap() - 1
    }).collect::<Vec<_>>();

    let mut positions = starting_positions.clone();
    let mut scores = vec![0, 0];
    let mut dice_roll = 0;
    let mut times_rolled = 0;
    'game: loop {
        for player in 0..2 {
            for _roll in 0..3 {
                positions[player] += dice_roll + 1;
                positions[player] %= 10;
                dice_roll += 1;
                dice_roll %= 100;
                times_rolled += 1;
            }

            scores[player] += positions[player] + 1;

            if scores[player] >= 1000 {
                let part1 = scores[1 - player] * times_rolled;
                println!("{}", part1);

                break 'game;
            }
        }
    }

    let wins = dirac_wins_recursive(0, 3, starting_positions, vec![21, 21], &mut HashMap::new());

    let part2 = wins.0.max(wins.1);
    println!("{}", part2);
}
