use std::io::BufRead;

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
struct OperatorPayload {
    operator: Operator,
    sub_packets: Vec<Packet>,
}

#[derive(Debug)]
enum Payload {
    LiteralValue(usize),
    Operator(OperatorPayload),
}

#[derive(Debug)]
struct Packet {
    version: usize,
    payload: Payload,
}

fn decode_binary(binary: &[usize]) -> usize {
    binary.iter().fold(0, |accu, digit| 2 * accu + digit)
}

fn decode_packet(packet: &[usize]) -> (Packet, usize) {
    let version = decode_binary(&packet[0..3]);
    let type_id = decode_binary(&packet[3..6]);

    let (payload, next) = match type_id {
        4 => {
            let mut start = 6;
            let mut value = 0;
            loop {
                value *= 16;
                value += decode_binary(&packet[(start + 1)..(start + 5)]);

                if packet[start] == 0 {
                    break;
                }

                start += 5;
            }

            (Payload::LiteralValue(value), start + 5)
        }
        _ => {
            let mut sub_packets = Vec::new();
            let mut start;
            if packet[6] == 0 {
                start = 22;
                let end = start + decode_binary(&packet[7..22]);
                while start < end {
                    let (sub_packet, decoded) = decode_packet(&packet[start..]);
                    sub_packets.push(sub_packet);
                    start += decoded;
                }
            } else {
                start = 18;
                let num = decode_binary(&packet[7..18]);
                for _ in 0..num {
                    let (sub_packet, decoded) = decode_packet(&packet[start..]);
                    sub_packets.push(sub_packet);
                    start += decoded;
                }
            }

            let operator = match type_id {
                0 => Operator::Sum,
                1 => Operator::Product,
                2 => Operator::Minimum,
                3 => Operator::Maximum,
                5 => Operator::GreaterThan,
                6 => Operator::LessThan,
                7 => Operator::EqualTo,
                _ => unreachable!(),
            };

            let payload = OperatorPayload {
                operator,
                sub_packets,
            };

            (Payload::Operator(payload), start)
        }
    };

    (Packet { version, payload }, next)
}

fn sum_versions_recursive(packet: &Packet) -> usize {
    match &packet.payload {
        Payload::LiteralValue(_) => packet.version,
        Payload::Operator(payload) => {
            packet.version
                + payload
                    .sub_packets
                    .iter()
                    .map(|sub_packet| sum_versions_recursive(sub_packet))
                    .sum::<usize>()
        }
    }
}

fn evaluate_recursive(packet: &Packet) -> usize {
    match &packet.payload {
        Payload::LiteralValue(value) => *value,
        Payload::Operator(payload) => {
            let values = payload
                .sub_packets
                .iter()
                .map(|sub_packet| evaluate_recursive(sub_packet))
                .collect::<Vec<_>>();
            match payload.operator {
                Operator::Sum => values.iter().sum(),
                Operator::Product => values.iter().product(),
                Operator::Minimum => values.into_iter().min().unwrap(),
                Operator::Maximum => values.into_iter().max().unwrap(),
                Operator::GreaterThan => {
                    if values[0] > values[1] {
                        1
                    } else {
                        0
                    }
                }
                Operator::LessThan => {
                    if values[0] < values[1] {
                        1
                    } else {
                        0
                    }
                }
                Operator::EqualTo => {
                    if values[0] == values[1] {
                        1
                    } else {
                        0
                    }
                }
            }
        }
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let line = std::io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let binary_message: Vec<usize> = line
        .chars()
        .map(|hexdigit| match hexdigit {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
        .concat();

    let (decoded_message, _) = decode_packet(&binary_message[0..binary_message.len()]);

    let part1 = sum_versions_recursive(&decoded_message);
    println!("{}", part1);

    let part2 = evaluate_recursive(&decoded_message);
    println!("{}", part2);
}
