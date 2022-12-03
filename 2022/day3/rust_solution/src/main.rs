use itertools::Itertools;
use std::env;
use std::error::Error;
use std::fs;
use std::io;

fn read_input() -> io::Result<String> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

const MAP_SIZE: usize = ((('z' as u8) - ('a' as u8) + 1) * 2) as usize;

fn char_to_index(ch: char) -> usize {
    (if (ch as u8) < ('a' as u8) {
        ((ch as u8) - ('A' as u8)) + (('z' as u8) - ('a' as u8)) + 1
    } else {
        (ch as u8) - ('a' as u8)
    }) as usize
}

fn get_rucksack_score(rucksack: &str) -> u32 {
    let mut first_compartment = [0_u8; MAP_SIZE];
    rucksack
        .chars()
        .take(rucksack.len() / 2)
        .for_each(|item| first_compartment[char_to_index(item)] += 1);

    for item in rucksack.chars().rev().take(rucksack.len() / 2) {
        let index = char_to_index(item);
        if first_compartment[index] > 0 {
            return (index + 1) as u32;
        }
    }

    0
}

fn round_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| get_rucksack_score(line))
        .sum::<u32>()
}

fn find_group_badge(rucksacks: &Vec<&str>) -> char {
    let mut mutual_elements = [0_u8; MAP_SIZE];

    rucksacks[0]
        .chars()
        .for_each(|item| mutual_elements[char_to_index(item)] = 1);

    rucksacks[1].chars().for_each(|item| {
        let index = char_to_index(item);
        if mutual_elements[index] == 1 {
            mutual_elements[index] = 2;
        }
    });

    for item in rucksacks[2].chars() {
        if mutual_elements[char_to_index(item)] == 2 {
            return item;
        }
    }

    '?'
}

fn round_2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|lines| find_group_badge(&lines.collect::<Vec<_>>()))
        .map(|badge| char_to_index(badge) + 1)
        .sum::<usize>() as u32
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;
    println!("round 1 result: {}", round_1(&input));
    println!("round 2 result: {}", round_2(&input));

    Ok(())
}
