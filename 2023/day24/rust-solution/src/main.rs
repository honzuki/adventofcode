use hail::{HailErr, HailStones, Vec3};

mod hail;

fn part_1(input: &str) -> Result<usize, HailErr> {
    let hailstones: HailStones = input.parse()?;

    Ok(hailstones.predict_collisions(
        &Vec3::from_xy(200000000000000.0, 200000000000000.0),
        &Vec3::from_xy(400000000000000.0, 400000000000000.0),
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);

    Ok(())
}
