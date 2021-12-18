use std::io::BufRead;

use nom::{IResult, branch::alt, character::complete::{char, digit1}};

#[derive(Debug, Clone)]
struct Pair {
    left: Box<Element>,
    right: Box<Element>,
}

#[derive(Debug, Clone)]
enum Element {
    Regular(u32),
    Snail(Pair),
}

fn boxed_element_from_str(s: &str) -> Box<Element> {
    Box::new(Element::Snail(parse_pair(s).unwrap().1))
}

fn add_elements(left: Box<Element>, right: Box<Element>) -> Box<Element> {
    let mut element = Box::new(Element::Snail(Pair { left, right }));

    loop {
        if let Some(_) = explode(&mut element, 4) {
            continue;
        }

        if split(&mut element) {
            continue;
        }

        break;
    }

    element
}

fn explode(element: &mut Box<Element>, depth: usize) -> Option<(u32, u32)> {
    match **element {
        Element::Regular(_) => None,
        Element::Snail(ref mut pair) => {
            if depth == 0 {
                let left = match *pair.left {
                    Element::Regular(value) => value,
                    Element::Snail(_) => unreachable!(),
                };
        
                let right = match *pair.right {
                    Element::Regular(value) => value,
                    Element::Snail(_) => unreachable!(),
                };
        
                **element = Element::Regular(0);
        
                return Some((left, right));
            }
        
            if let Some((left, right)) = explode(&mut pair.left, depth - 1) {
                add_to_leftmost(&mut pair.right, right);
                return Some((left, 0));
            }
        
            if let Some((left, right)) = explode(&mut pair.right, depth - 1) {
                add_to_rightmost(&mut pair.left, left);
                return Some((0, right));
            }
        
            None
        }
    }
}

fn split(element: &mut Box<Element>) -> bool {
    match **element {
        Element::Regular(old_value) => {
            if old_value >= 10 {
                let left_value = old_value / 2;
                let right_value = if old_value % 2 == 1 { old_value / 2 + 1 } else { old_value / 2 };

                let left = Box::new(Element::Regular(left_value));
                let right = Box::new(Element::Regular(right_value));

                **element = Element::Snail(Pair { left, right });

                return true;
            }

            false
        }
        Element::Snail(ref mut pair) => {
            if split(&mut pair.left) {
                return true;
            }

            split(&mut pair.right)
        }
    }
}

fn magnitude(element: &Box<Element>) -> u32 {
    match &**element {
        Element::Regular(value) => *value,
        Element::Snail(pair) => 3 * magnitude(&pair.left) + 2 * magnitude(&pair.right),
    }
}

fn add_to_leftmost(element: &mut Box<Element>, value: u32) {
    match **element {
        Element::Regular(ref mut old_value) => {
            *old_value += value;
        }
        Element::Snail(ref mut pair) => {
            add_to_leftmost(&mut pair.left, value);
        }
    }
}

fn add_to_rightmost(element: &mut Box<Element>, value: u32) {
    match **element {
        Element::Regular(ref mut old_value) => {
            *old_value += value;
        }
        Element::Snail(ref mut pair) => {
            add_to_rightmost(&mut pair.right, value);
        }
    }
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, _) = char('[')(input)?;
    let (input, left) = parse_element(input)?;
    let (input, _) = char(',')(input)?;
    let (input, right) = parse_element(input)?;
    let (input, _) = char(']')(input)?;

    let left = Box::new(left);
    let right = Box::new(right);

    Ok((input, Pair { left, right }))
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    alt((parse_regular, parse_snail))(input)
}

fn parse_regular(input: &str) -> IResult<&str, Element> {
    let (input, digits) = digit1(input)?;
    let value = digits.parse().unwrap();

    Ok((input, Element::Regular(value)))
}

fn parse_snail(input: &str) -> IResult<&str, Element> {
    let (input, pair) = parse_pair(input)?;

    Ok((input, Element::Snail(pair)))
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let snail_numbers = reader.lines().map(|line| boxed_element_from_str(&line.unwrap())).collect::<Vec<_>>();

    let mut sum = snail_numbers[0].clone();
    for i in 1..snail_numbers.len() {
        sum = add_elements(sum, snail_numbers[i].clone());
    }

    let part1 = magnitude(&sum);
    println!("{}", part1);

    let mut part2 = 0;
    for (i, a) in snail_numbers.iter().enumerate() {
        for (j, b) in snail_numbers.iter().enumerate() {
            if i == j {
                continue;
            }

            let magnitude = magnitude(&add_elements(a.clone(), b.clone()));

            if magnitude > part2 {
                part2 = magnitude;
            }
        }
    }

    println!("{}", part2);
}
