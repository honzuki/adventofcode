use std::{env, error, fs, io};

mod snafu;
use snafu::SNAFU;

fn read_input() -> io::Result<String> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn part_1(input: &str) -> String {
    let result = input
        .lines()
        .map(|num| i64::from_snafu(num).unwrap())
        .sum::<i64>();

    result.to_snafu()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;

    println!("part 1 result: {}", part_1(&input));

    Ok(())
}
