use std::{env, error, fs, io, str::FromStr};

mod elves_game;
use elves_game::GameManager;

fn read_input() -> io::Result<String> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn part_1(input: &str) -> Result<usize, &'static str> {
    let mut game_manager = GameManager::from_str(&input)?;

    for _ in 0..10 {
        game_manager.execute_round();
    }

    Ok(game_manager.containing_tiles())
}

fn part_2(input: &str) -> Result<usize, &'static str> {
    let mut game_manager = GameManager::from_str(&input)?;

    let mut rounds = 0;
    while game_manager.execute_round() > 0 {
        rounds += 1;
    }

    Ok(rounds + 1)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;

    println!("part 1 result: {}", part_1(&input)?);
    println!("part 2 result: {}", part_2(&input)?);

    Ok(())
}
