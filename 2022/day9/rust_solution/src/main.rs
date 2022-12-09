use std::{env, error, fs, io};

mod board;
mod command;

use board::Board;
use command::Cmd;

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn parse_input(input: String) -> Vec<Cmd> {
    input
        .lines()
        .map(|line| Cmd::parse_line(line).unwrap())
        .collect()
}

fn execute(input: &Vec<Cmd>, tail_len: usize) -> usize {
    let mut board = Board::new(tail_len);
    for cmd in input {
        board.execute(&cmd);
    }
    board.get_visited_size()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let input = parse_input(input);
    println!("part 1 result: {}", execute(&input, 1));
    println!("part 1 result: {}", execute(&input, 9));

    Ok(())
}
