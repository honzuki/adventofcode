use std::collections::HashMap;

use engine::{iter_parts, Gear};
use helpers::{clamp_down, clamp_up};

mod engine;
mod helpers;

fn part_1(input: &str) -> u32 {
    let input = input.trim();
    let scheme = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            let rows = clamp_down(r, 1)..clamp_up(r + 1, 1, scheme.len());

            iter_parts(line.trim())
                .filter(|part| {
                    part.range.clone().any(|c| {
                        rows.clone()
                            .any(|r| !scheme[r][c].is_ascii_digit() && scheme[r][c] != '.')
                    })
                })
                .map(|part| part.number)
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    let input = input.trim();
    let scheme = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut adj_map: HashMap<(usize, usize), Gear> = Default::default();
    for (r, line) in input.lines().enumerate() {
        let rows = clamp_down(r, 1)..clamp_up(r + 1, 1, scheme.len());

        // for each part, find all adjacent gears, and record the part as adjacent to them
        for part in iter_parts(line.trim()) {
            for r in rows.clone() {
                for c in part.range.clone() {
                    if scheme[r][c] == '*' {
                        adj_map.entry((r, c)).or_default().add_part(part.clone())
                    }
                }
            }
        }
    }

    adj_map
        .values()
        .filter(|gear| gear.adj_count() == 2)
        .map(|gear| gear.ratio())
        .sum::<u32>()
}

fn main() -> std::io::Result<()> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(super::part_1(input), 4361);
    }

    #[test]
    fn part_2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(super::part_2(input), 467835);
    }
}
