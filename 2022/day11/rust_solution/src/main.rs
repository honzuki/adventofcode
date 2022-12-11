use std::{env, error, fs, io, str::FromStr};

mod monkey;

use monkey::GameManager;

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn part_1(input: &str) -> usize {
    let mut game_manager = GameManager::from_str(&input).unwrap();

    for _ in 0..20 {
        game_manager.execute_round();
    }

    game_manager.monkey_business_level()
}

fn part_2(input: &str) -> usize {
    let mut game_manager = GameManager::from_str(&input).unwrap();
    game_manager.set_relief(1);

    for _ in 0..10000 {
        game_manager.execute_round();
    }

    game_manager.monkey_business_level()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;

    println!("part 1 result: {}", part_1(&input));
    println!("part 2 result: {}", part_2(&input));

    Ok(())
}
