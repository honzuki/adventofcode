use race::{parse_record_sheet_correctly, ParseRecordErr};

use crate::race::parse_record_sheet;

mod race;

fn part_1(input: &str) -> Result<usize, ParseRecordErr> {
    let records = parse_record_sheet(input)?;
    let ways = records.iter().map(|record| {
        // for each possible split of the record into wait_time | travel time
        // calculate the distance and count the amount of ways that are bigger than the best record
        (0..record.time)
            .map(|wait| wait * (record.time - wait))
            .filter(|distance| *distance > record.distance)
            .count()
    });

    Ok(ways.product())
}

fn part_2(input: &str) -> Result<u64, ParseRecordErr> {
    let record = parse_record_sheet_correctly(input)?;

    // find the first best record
    let Some(first) = (0..record.time).find(|wait| (wait * (record.time - wait)) > record.distance)
    else {
        return Ok(0);
    };
    // find the last bast record
    let last = (0..record.time)
        .rev()
        .find(|wait| (wait * (record.time - wait)) > record.distance)
        .unwrap();

    Ok(last - first + 1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part_1: {}", part_1(&input)?);
    println!("part_2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(super::part_1(input).unwrap(), 288);
    }

    #[test]
    fn part_2() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(super::part_2(input).unwrap(), 71503);
    }
}
