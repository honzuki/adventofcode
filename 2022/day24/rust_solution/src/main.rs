use std::{env, error, fs, io, str::FromStr};

mod mountain;
use mountain::Graph;

fn read_input() -> io::Result<String> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn part_1(graph: &Graph) -> Result<usize, &'static str> {
    match graph.search(mountain::State::start()) {
        Some(time) => Ok(time),
        None => Err("can not find a path"),
    }
}

fn part_2(graph: &Graph) -> Result<usize, &'static str> {
    match graph.search_circular(mountain::State::start()) {
        Some(time) => Ok(time),
        None => Err("can not find a path"),
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let graph = Graph::from_str(&input)?;

    println!("part 1 result: {}", part_1(&graph)?);

    println!("part 2 result: {}", part_2(&graph)?);

    Ok(())
}
