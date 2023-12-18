use digger::{DiggerErr, Plan};
use terrain::Terrain;

mod digger;
mod position;
mod rgb;
mod terrain;

fn part_1(input: &str) -> Result<usize, DiggerErr> {
    let plan: Plan = input.parse()?;
    let terrain: Terrain = plan.into();

    Ok(terrain.dig_interior())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
        assert_eq!(super::part_1(input).unwrap(), 62);
    }
}
