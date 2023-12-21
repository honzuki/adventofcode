use garden::{GardenErr, Map};

mod garden;

fn part_1(input: &str) -> Result<usize, GardenErr> {
    let map: Map = input.parse()?;

    Ok(map.can_reach(64))
}

fn part_2(input: &str) -> Result<usize, GardenErr> {
    let map: Map = input.parse()?;

    Ok(map.can_reach_wrap(5000))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}
