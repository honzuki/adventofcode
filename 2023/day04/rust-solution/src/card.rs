use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: u32,
    choose: Vec<u32>,
    winning: HashSet<u32>,
}

impl Card {
    /// Calculate the amount of points this card is worth
    pub fn worth(self) -> u32 {
        let power = self.winnig_count();

        if power == 0 {
            // won nothing
            return 0;
        }

        2u32.pow(power - 1)
    }

    /// Calculates the number of winning numbers you have
    pub fn winnig_count(mut self) -> u32 {
        self.choose
            .into_iter()
            .filter(|number| self.winning.remove(number))
            .count() as u32
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseCardErr {
    #[error("unknown format")]
    UnknownFormat,

    #[error("can not parse the game id")]
    BadGameId,

    #[error("expected a list of numbers")]
    ParseInt,
}

impl FromStr for Card {
    type Err = ParseCardErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, data) = s.split_once(':').ok_or(ParseCardErr::UnknownFormat)?;
        let id = id.trim().split_ascii_whitespace().collect::<Vec<_>>();
        let id = id
            .get(1)
            .and_then(|id| id.parse::<u32>().ok())
            .ok_or(ParseCardErr::BadGameId)?;

        let (winning, choose) = data.split_once('|').ok_or(ParseCardErr::UnknownFormat)?;
        fn from_number_list(list: &str) -> impl Iterator<Item = Result<u32, ParseCardErr>> + '_ {
            list.trim()
                .split_ascii_whitespace()
                .map(|num| num.parse::<u32>().map_err(|_| ParseCardErr::ParseInt))
        }

        let winning = from_number_list(winning).collect::<Result<HashSet<_>, _>>()?;
        let choose = from_number_list(choose).collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            id,
            choose,
            winning,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected_output = Card {
            id: 1,
            choose: vec![83, 86, 6, 31, 17, 9, 48, 53],
            winning: [41, 48, 83, 86, 17].into_iter().collect(),
        };

        let output: Card = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }
}
