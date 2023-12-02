use game::{ParseRecordErr, Record, Set};

mod game;

const MAX_SET: Set = Set {
    red: 12,
    green: 13,
    blue: 14,
};

fn part_1(input: &str) -> Result<u32, ParseRecordErr> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let record = match line.parse::<Record>() {
                Ok(record) => record,
                Err(err) => return Some(Err(err)),
            };

            if record.overflow(&MAX_SET) {
                None
            } else {
                Some(Ok(record.id))
            }
        })
        .sum::<Result<u32, _>>()
}

fn part_2(input: &str) -> Result<u64, ParseRecordErr> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Record>())
        .map(|line| line.map(|line| line.find_max_set().power()))
        .sum::<Result<u64, _>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let output = super::part_1(input).unwrap();

        assert_eq!(output, 8);
    }

    #[test]
    fn part_2() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let output = super::part_2(input).unwrap();

        assert_eq!(output, 2286);
    }
}
