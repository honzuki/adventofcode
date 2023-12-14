use std::collections::HashMap;

use prd::{Platform, PrdErr};

mod prd;

fn part_1(input: &str) -> Result<usize, PrdErr> {
    let mut platform: Platform = input.parse()?;
    platform.tilt_north();

    Ok(platform.load_on_north_support_beam())
}

fn part_2(input: &str) -> Result<usize, PrdErr> {
    let mut platform: Platform = input.parse()?;

    let mut visited = HashMap::new();
    for idx in 0.. {
        if let Some(pidx) = visited.insert(platform.clone(), idx) {
            let cycle_length = idx - pidx;
            let goal = 1000000000;
            let after_cycle = (goal - pidx) % cycle_length;

            for _ in 0..after_cycle {
                platform.tilt_cycle();
            }

            return Ok(platform.load_on_north_support_beam());
        }

        platform.tilt_cycle();
    }

    unreachable!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 136);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 64);
    }
}
