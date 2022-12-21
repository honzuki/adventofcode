use std::{env, error, fs, io};

mod calculator;
use calculator::{build_numbers_map, find_controlled_value};

fn read_input() -> io::Result<String> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn part_1(input: &str) -> Result<i64, &'static str> {
    let (mut numbers, constraints) = build_numbers_map(input)?;
    let mut root = numbers.remove("root").unwrap();
    root.resolve_dep_list(&mut numbers, &constraints);
    Ok(root.get_value().unwrap())
}

fn part_2(input: &str) -> Result<i64, &'static str> {
    let (mut numbers, constraints) = build_numbers_map(input)?;
    numbers.get_mut("root").unwrap().set_op("=")?;

    Ok(find_controlled_value("root", "humn", numbers, &constraints))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;

    println!("part 1 result: {}", part_1(&input)?);
    println!("part 2 result: {}", part_2(&input)?);

    Ok(())
}
