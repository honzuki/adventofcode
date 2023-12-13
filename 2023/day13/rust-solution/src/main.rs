use rayon::iter::{ParallelBridge, ParallelIterator};
use reflection::{Pattern, PatternErr};

mod reflection;

fn find_reflection(input: &str, smudge: bool) -> Result<usize, PatternErr> {
    input
        .trim()
        .split("\n\n")
        .par_bridge()
        .map(|pattern| {
            pattern.parse::<Pattern>().map(|pattern| {
                pattern
                    .find_horizontal_reflection(smudge)
                    .map(|rows| (rows + 1) * 100)
                    .unwrap_or_else(|| {
                        pattern
                            .find_vertical_reflection(smudge)
                            .map(|cols| cols + 1)
                            .unwrap_or(0)
                    })
            })
        })
        .sum()
}

fn part_1(input: &str) -> Result<usize, PatternErr> {
    find_reflection(input, false)
}

fn part_2(input: &str) -> Result<usize, PatternErr> {
    find_reflection(input, true)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1 = {}", part_1(&input)?);
    println!("part 2 = {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 405);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 400);
    }
}
