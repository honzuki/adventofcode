use std::{env, error, fs, io, str::FromStr};

mod tetris;

static CHAMBER_SIZE: usize = 7;

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

static TILES: [&str; 5] = [
    "####",
    ".#.\n###\n.#.",
    "..#\n..#\n###",
    "#\n#\n#\n#",
    "##\n##",
];

fn get_tiles() -> Vec<tetris::tiles::Tile> {
    TILES
        .iter()
        .map(|raw_tile| tetris::tiles::Tile::from_str(raw_tile).unwrap())
        .collect::<Vec<_>>()
}

fn part_1(input: &str, count: usize) -> usize {
    let mut game_manager = tetris::GameManager::new(CHAMBER_SIZE, get_tiles(), &input);

    for _ in 0..count {
        while !game_manager.tick() {}
    }

    game_manager.get_height()
}

fn part_2(input: &str, count: usize) -> usize {
    let mut game_manager = tetris::GameManager::new(CHAMBER_SIZE, get_tiles(), &input);

    let (blocks_before_cycle, reminder) = game_manager.find_cycle().unwrap();
    let block_in_cycle = reminder - blocks_before_cycle;

    let height_before_cycle = part_1(input, blocks_before_cycle);
    let height_after_cycle = part_1(input, blocks_before_cycle + block_in_cycle);
    let cycle_height = height_after_cycle - height_before_cycle;

    let cycles_in_count = (count - blocks_before_cycle) / block_in_cycle;
    let reminder = (count - blocks_before_cycle) % block_in_cycle;

    let height_after_cycle_and_reminder =
        part_1(input, blocks_before_cycle + block_in_cycle + reminder);
    let reminder_height = height_after_cycle_and_reminder - height_after_cycle;

    height_before_cycle + (cycle_height * cycles_in_count) + reminder_height
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;

    println!("part 1 result: {}", part_1(&input, 2022));
    println!("part 2 result: {}", part_2(&input, 1000000000000));

    Ok(())
}
