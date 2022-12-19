use std::{env, error, fs, io, str::FromStr};

mod factory;
use factory::blueprint::Blueprint;

fn read_input() -> io::Result<String> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn parse_input(input: &str) -> Result<Vec<Blueprint>, String> {
    input
        .lines()
        .map(|line| Blueprint::from_str(line))
        .collect::<Result<Vec<_>, _>>()
}

fn part_1(input: &Vec<Blueprint>) -> usize {
    input
        .iter()
        .map(|blueprint| {
            let best_state = factory::dfs(factory::State::new(24), blueprint);
            factory::evaluate(&best_state, blueprint)
        })
        .sum()
}

fn part_2(input: &Vec<Blueprint>) -> usize {
    let mut mul = 1;

    for geode_count in input.iter().take(3).map(|blueprint| {
        let best_state = factory::dfs(factory::State::new(32), blueprint);
        best_state.geode_count()
    }) {
        mul *= geode_count;
    }

    mul
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let input = parse_input(&input)?;

    println!("part 1 result: {}", part_1(&input));
    println!("part 2 result: {}", part_2(&input));

    Ok(())
}
