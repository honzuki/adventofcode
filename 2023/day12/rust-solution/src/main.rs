use rayon::{iter::ParallelIterator, str::ParallelString};
use springs::{Record, SpringErr};

mod springs;

fn part_1(input: &str) -> Result<usize, SpringErr> {
    input
        .trim()
        .par_lines()
        .map(|line| {
            line.parse::<Record>()
                .map(|record| record.count_arrangements())
        })
        .sum()
}

fn part_2(input: &str) -> Result<usize, SpringErr> {
    input
        .trim()
        .par_lines()
        .map(|line| {
            line.parse::<Record>().map(|mut record| {
                record.unfold(5);
                record.count_arrangements()
            })
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 21);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 525152);
    }
}
