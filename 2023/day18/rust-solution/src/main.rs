use digger::{DiggerErr, Plan};

mod digger;
mod position;

fn part_1(input: &str) -> Result<usize, DiggerErr> {
    let plan: Plan = input.parse()?;

    Ok(plan.calculate_area())
}

fn part_2(input: &str) -> Result<usize, DiggerErr> {
    let plan = Plan::from_rgb(input)?;

    Ok(plan.calculate_area())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 62);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 952408144115);
    }
}
