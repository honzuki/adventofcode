use crate::universe::Universe;

mod universe;

fn part_1(input: &str) -> Option<usize> {
    let universe = Universe::from_str(input, 2)?;

    Some(universe.minimum_path_sum())
}

fn part_2(input: &str) -> Option<usize> {
    let universe = Universe::from_str(input, 1000000)?;

    Some(universe.minimum_path_sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input).ok_or("empty graph")?);
    println!("part 2: {}", part_2(&input).ok_or("empty graph")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        assert_eq!(super::part_1(input).unwrap(), 374);
    }
}
