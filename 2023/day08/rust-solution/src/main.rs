use map::parse_input;

use crate::map::Move;

mod map;

fn part_1(input: &str) -> Result<usize, String> {
    let mut steps = 0;

    let (network, mut pattern) = parse_input(input)?;

    let mut current = network
        .get("AAA")
        .ok_or("network is missing node AAA".to_string())?;
    while current.name != "ZZZ" {
        let next = match pattern.next().unwrap() {
            Move::Left => current.left.as_str(),
            Move::Right => current.right.as_str(),
        };

        current = network
            .get(next)
            .ok_or_else(|| format!("the network is missing node: {}", next))?;

        steps += 1;
    }

    Ok(steps)
}

fn part_2(input: &str) -> Result<usize, String> {
    let (network, pattern) = parse_input(input)?;

    let nodes = network
        .values()
        .filter(|node| node.name.ends_with('A'))
        .collect::<Vec<_>>();

    let cycles = nodes
        .into_iter()
        .map(|mut current| {
            let mut steps = 0usize;
            let mut pattern = pattern.clone();
            while !current.name.ends_with('Z') {
                let next = match pattern.next().unwrap() {
                    Move::Left => current.left.as_str(),
                    Move::Right => current.right.as_str(),
                };

                current = network
                    .get(next)
                    .ok_or_else(|| format!("the network is missing node: {}", next))?;

                steps += 1;
            }

            Ok::<_, String>(steps)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(cycles.into_iter().fold(1, num::integer::lcm))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part_1 = {}", part_1(&input)?);
    println!("part_2 = {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_1() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(super::part_1(input).unwrap(), 6);
    }

    #[test]
    fn part_2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(super::part_2(input).unwrap(), 6);
    }
}
