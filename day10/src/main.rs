use std::io::BufRead;

const OPENING: &str = "([{<";
const CLOSING: &str = ")]}>";

fn complete_line(line: &str) -> Result<String, char> {
    let mut stack = Vec::new();
    for char in line.chars() {
        if let Some(pos) = CLOSING.find(char) {
            if let Some(top) = stack.last() {
                if *top == OPENING.as_bytes()[pos] as char {
                    stack.pop();
                } else {
                    return Err(char);
                }
            } else {
                return Err(char);
            }
        } else {
            stack.push(char);
        }
    }

    stack.reverse();
    Ok(stack.iter().collect::<String>())
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    let part1 = lines
        .iter()
        .map(|line| match complete_line(line) {
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            _ => 0,
        })
        .sum::<u32>();

    println!("{}", part1);

    let mut completion_scores = lines
        .iter()
        .filter_map(|line| match complete_line(line) {
            Ok(completion) => {
                let completion_score =
                completion
                .chars()
                .map(|char| match char {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                })
                .fold(0u64, |total_score, char_score| total_score * 5 + char_score);
                Some(completion_score)
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    completion_scores.sort();
    let part2 = completion_scores[completion_scores.len() / 2];

    println!("{}", part2);
}
