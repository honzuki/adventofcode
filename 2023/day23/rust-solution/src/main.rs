use hills::{HillsErr, Trail};

mod hills;

fn part_1(input: &str) -> Result<Option<usize>, HillsErr> {
    let trail: Trail = input.parse()?;
    Ok(trail.find_longest_slippery_path())
}

fn part_2(input: &str) -> Result<Option<usize>, HillsErr> {
    let trail: Trail = input.parse()?;

    Ok(trail.find_longest_path())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!(
        "part 1: {}",
        part_1(&input)?.ok_or("there is no path from start to end")?
    );
    println!(
        "part 2: {}",
        part_2(&input)?.ok_or("there is no path from start to end")?
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap().unwrap(), 94);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap().unwrap(), 154);
    }
}
