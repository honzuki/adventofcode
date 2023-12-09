use oasis::{History, OasisErr};

mod oasis;

fn part_1(input: &str) -> Result<i32, OasisErr> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .parse::<History>()
                .and_then(|history| history.predict_next())
        })
        .sum::<Result<i32, _>>()
}

fn part_2(input: &str) -> Result<i32, OasisErr> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .parse::<History>()
                .and_then(|history| history.predict_prev())
        })
        .sum::<Result<i32, _>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 114);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 2);
    }
}
