use std::{env, error, fs, io, str::FromStr};

mod cave;

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let graph = cave::Graph::from_str(&input)?;

    println!("part 1 result: {}", graph.find_best_strategy());

    println!("part 2 result: {}", graph.find_best_pair());

    Ok(())
}
