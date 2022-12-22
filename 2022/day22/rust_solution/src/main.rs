use std::{env, error, fs, io, str::FromStr};

mod board;
mod helper;
mod moves;

use board::Board;
use moves::Movements;

fn read_input() -> io::Result<String> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn part_1(input: &str) -> Result<usize, &'static str> {
    let (board, moves) = input.split_once("\n\n").unwrap();

    let moves = Movements::from_str(moves.trim())?;
    let mut board = Board::from_str(board)?;
    for movement in moves.get_moves() {
        board.execute_movement(movement);
    }

    Ok(board.get_current_password())
}

fn part_2(input: &str) -> Result<usize, &'static str> {
    let (board, moves) = input.split_once("\n\n").unwrap();

    let moves = Movements::from_str(moves.trim())?;
    let mut board = Board::from_cubic_str(board)?;
    for movement in moves.get_moves() {
        board.execute_movement(movement);
    }

    Ok(board.get_current_password())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;

    println!("part 1 result: {}", part_1(&input)?);
    println!("part 2 result: {}", part_2(&input)?);

    Ok(())
}
