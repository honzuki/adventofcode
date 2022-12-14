use std::{env, error, fs, io, str::FromStr};

mod cavemap;
use cavemap::CaveMap;

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn emulate_sand_drop(cave_map: &mut CaveMap, floor: bool) -> usize {
    let mut count = 0;

    while cave_map.drop_sand(500, 0, floor) {
        count += 1;
    }

    count
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let cave_map = CaveMap::from_str(&input)?;
    println!(
        "part 1 result: {}",
        emulate_sand_drop(&mut (cave_map.clone()), false)
    );
    println!(
        "part 2 result: {}",
        emulate_sand_drop(&mut (cave_map.clone()), true)
    );

    Ok(())
}
