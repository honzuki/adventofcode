use emulator::{Emulator, EmulatorErr, ExecuteResult};

mod emulator;
mod modules;

fn part_1(input: &str) -> Result<usize, EmulatorErr> {
    let mut emulator: Emulator = input.parse()?;
    let mut result = ExecuteResult::default();
    for _ in 0..1000 {
        result += emulator.click();
    }

    Ok(result.low * result.high)
}

fn part_2(input: &str) -> Result<usize, EmulatorErr> {
    let emulator: Emulator = input.parse()?;

    let cycle_lengths = emulator
        .get_inputs("rx")?
        .iter()
        .flat_map(|input| emulator.get_inputs(input).unwrap())
        .map(|input| emulator.clone().find_cycle_on_observer(input, true))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(cycle_lengths.into_iter().fold(1, num::integer::lcm))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
        assert_eq!(super::part_1(input).unwrap(), 32000000);
    }
}
