use std::collections::HashMap;

use card::{Card, ParseCardErr};

mod card;

fn part_1(input: &str) -> Result<u32, ParseCardErr> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Card>().map(|card| card.worth()))
        .sum::<Result<_, _>>()
}

fn part_2(input: &str) -> Result<u32, ParseCardErr> {
    let cards = input
        .trim()
        .lines()
        .map(|line| line.parse::<Card>())
        .collect::<Result<Vec<_>, _>>()?;

    let Some(max_card_id) = cards.iter().map(|card| card.id).max() else {
        return Ok(0); // no cards
    };

    // card id -> amount
    let mut hand: HashMap<u32, u32> = HashMap::new();
    for card in cards {
        let count = hand.entry(card.id).or_default();

        // add the current card
        *count += 1;
        let count = *count; // end the borrow

        // add 'count' cards to all of the cards we won
        for idx in (card.id + 1)..(card.id + 1 + card.winnig_count()) {
            if idx > max_card_id {
                // reached the end
                break;
            }

            *hand.entry(idx).or_default() += count;
        }
    }

    Ok(hand.values().sum())
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
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(super::part_1(input).unwrap(), 13);
    }

    #[test]
    fn part_2() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(super::part_2(input).unwrap(), 30);
    }
}
