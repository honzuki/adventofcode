use poker::{Hand, ParseHandErr};

mod poker;

fn part_1(input: &str) -> Result<u32, ParseHandErr> {
    let mut hands = input
        .trim()
        .lines()
        .map(|hand| hand.parse::<Hand>())
        .collect::<Result<Vec<_>, _>>()?;

    hands.sort();
    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum())
}

fn part_2(input: &str) -> Result<u32, ParseHandErr> {
    let mut hands = input
        .trim()
        .lines()
        .map(|hand| {
            hand.parse::<Hand>().map(|mut hand| {
                hand.enable_joker_cards();
                hand
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    hands.sort();
    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part_1 = {}", part_1(&input)?);
    println!("part_2 = {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 6440);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 5905);
    }
}
