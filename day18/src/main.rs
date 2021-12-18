use std::io::BufRead;

use nom::{IResult, branch::alt, character::complete::{char, digit1}, number::complete::u32, sequence::delimited, multi::many1};

#[derive(Debug)]
struct Pair {
    left: Box<Element>,
    right: Box<Element>,
}

#[derive(Debug)]
enum Element {
    Regular(u32),
    Snail(Pair),
}

impl Pair {
    fn from_str(s: &str) -> Pair {
        Pair::parse_pair(s).unwrap().1
    }

    fn parse_pair(input: &str) -> IResult<&str, Pair> {
        let (input, _) = char('[')(input)?;
        let (input, left) = Pair::parse_element(input)?;
        let (input, _) = char(',')(input)?;
        let (input, right) = Pair::parse_element(input)?;
        let (input, _) = char(']')(input)?;

        let left = Box::new(left);
        let right = Box::new(right);

        Ok((input, Pair { left, right }))
    }

    fn parse_element(input: &str) -> IResult<&str, Element> {
        alt((Pair::parse_regular, Pair::parse_snail))(input)
    }

    fn parse_regular(input: &str) -> IResult<&str, Element> {
        let (input, digits) = digit1(input)?;
        let value = digits.parse().unwrap();

        Ok((input, Element::Regular(value)))
    }

    fn parse_snail(input: &str) -> IResult<&str, Element> {
        let (input, pair) = Pair::parse_pair(input)?;

        Ok((input, Element::Snail(pair)))
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let snail_numbers = reader.lines().map(|line| Pair::from_str(&line.unwrap())).collect::<Vec<_>>();

    dbg!(&snail_numbers);
}
