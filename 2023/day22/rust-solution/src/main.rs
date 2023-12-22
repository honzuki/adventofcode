use snapshot::{SnapshotErr, Stack};

mod snapshot;

fn part_1(input: &str) -> Result<usize, SnapshotErr> {
    let stack: Stack = input.parse()?;
    let stack = stack.after_fall();

    Ok(stack.count_bricks_that_can_be_disintegrated())
}

fn part_2(input: &str) -> Result<usize, SnapshotErr> {
    let stack: Stack = input.parse()?;
    let stack = stack.after_fall();

    Ok(stack.count_brick_that_will_fall_after_disintegration())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 5);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 7);
    }
}
