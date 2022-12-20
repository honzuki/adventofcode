use std::env;
use std::fs;
use std::error::Error;

fn read_input() -> std::io::Result<String> {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(val) => val,
        None => {
            eprintln!("missing input file path");
            std::process::exit(1)
        },
    };

    return fs::read_to_string(file_path)
}

fn get_calories_list(input: &str) -> Vec<i32> {
    input.split("\n\n").map(
        |items| 
        items.lines().map(
            |item| item.parse::<i32>().unwrap()
        ).sum()
    ).collect()
}

fn part_one(input: &str) -> i32{
    get_calories_list(input).into_iter().max().unwrap()
}

fn part_two(input: &str) -> i32 {
    let mut calories_list = get_calories_list(input);
    calories_list.sort();
    calories_list.iter().rev().take(3).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;
    println!("Round 1 result: {}", part_one(&input));
    println!("Round 2 result: {}", part_two(&input));

    Ok(())
}
