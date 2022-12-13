use itertools::Itertools;
use std::{env, error, fs, io, str::FromStr};

mod signal;

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn parse_input(input: &str) -> Result<Vec<signal::Packet>, signal::PacketErr> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| signal::Packet::from_str(line))
        .collect::<Result<Vec<_>, _>>()
}

fn part_1(input: &[signal::Packet]) -> usize {
    input
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (p1, p2))| p1.in_order(p2))
        .map(|(idx, _)| idx + 1)
        .sum::<usize>()
}

fn part_2(input: &[signal::Packet]) -> usize {
    let mut packets: Vec<_> = input.clone().into();
    let p1 = signal::Packet::from_str("[[2]]").unwrap();
    let p2 = signal::Packet::from_str("[[6]]").unwrap();
    packets.push(p1.clone());
    packets.push(p2.clone());

    packets.sort();

    let key_parts = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| **packet == p1 || **packet == p2)
        .map(|(idx, _)| idx + 1)
        .collect::<Vec<_>>();

    key_parts[0] * key_parts[1]
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let input = parse_input(&input).unwrap();

    println!("part 1 result: {}", part_1(&input));
    println!("part 2 result: {}", part_2(&input));

    Ok(())
}
