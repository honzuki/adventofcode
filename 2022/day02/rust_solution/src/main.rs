use std::env;
use std::error::Error;
use std::fs;
use std::io;

fn read_input() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");
    fs::read_to_string(input_path)
}

#[derive(PartialEq, Clone, Copy)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    fn map_choice(choice: &str) -> Result<Self, String> {
        match choice {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("can't map choice {}", choice)),
        }
    }

    fn get_value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_lose(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn get_win(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

enum RPSResult {
    Lose,
    Tie,
    Win,
}

impl RPSResult {
    fn get_result(opponent: &RPSChoice, player: &RPSChoice) -> Self {
        let to_lose = RPSChoice::get_lose(opponent);
        let to_win = RPSChoice::get_win(opponent);

        match player {
            _ if *player == to_lose => Self::Lose,
            _ if *player == to_win => Self::Win,
            _ => Self::Tie,
        }
    }

    fn map_result(raw_result: &str) -> Result<Self, String> {
        match raw_result {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(format!("can't map result {}", raw_result)),
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            Self::Lose => 0,
            Self::Tie => 3,
            Self::Win => 6,
        }
    }
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|game| {
            game.split(' ')
                .map(|choice| RPSChoice::map_choice(choice).unwrap())
                .collect::<Vec<RPSChoice>>()
        })
        .map(|game| {
            RPSResult::get_score(&RPSResult::get_result(&game[0], &game[1]))
                + RPSChoice::get_value(&game[1])
        })
        .sum::<i32>()
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|game| {
            let game: Vec<&str> = game.split(' ').collect();
            let opponent = RPSChoice::map_choice(game[0]).unwrap();
            let result = RPSResult::map_result(game[1]).unwrap();
            let choice = match result {
                RPSResult::Lose => RPSChoice::get_lose(&opponent),
                RPSResult::Tie => opponent.clone(),
                RPSResult::Win => RPSChoice::get_win(&opponent),
            };
            (opponent, choice)
        })
        .map(|(opponent, choice)| {
            RPSResult::get_score(&RPSResult::get_result(&opponent, &choice))
                + RPSChoice::get_value(&choice)
        })
        .sum::<i32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;
    println!("part 1 result: {}", part_1(&input));
    println!("part 2 result: {}", part_2(&input));

    Ok(())
}
