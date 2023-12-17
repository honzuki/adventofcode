use city::{CityErr, Map};

mod city;

fn part_1(input: &str) -> Result<usize, CityErr> {
    let map: Map = input.parse()?;

    Ok(map.minimize_heat_loss(3, 0))
}

fn part_2(input: &str) -> Result<usize, CityErr> {
    let map: Map = input.parse()?;

    Ok(map.minimize_heat_loss(10, 4))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 102);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 94);
    }

    #[test]
    fn part_2_special() {
        let input = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;
        assert_eq!(super::part_2(input).unwrap(), 71);
    }
}
