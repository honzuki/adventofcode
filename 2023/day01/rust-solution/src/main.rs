use calibration::calibration;

mod calibration;

fn part_1(input: &str) -> u64 {
    input
        .trim()
        .lines()
        // map the lines to an iterator of the digits they contain
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.to_digit(10).map(|digit| digit as u64))
        })
        // map each iterator of digits to the first and last digits, combained
        .map(|mut digits| {
            let first = digits.next().unwrap(); // we can assume each line contains atleast 1 digit
            let second = digits.next_back().unwrap_or(first);

            first * 10 + second
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|line| calibration(line).unwrap() as u64)
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn part_1_example_input() {
        let input = r#"1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"#;

        assert_eq!(part_1(input), 142);
    }
}
