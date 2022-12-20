use regex::Regex;
use std::env;
use std::fs;
use std::str;
use std::str::FromStr;

use itertools::Itertools;

fn read_input() -> (String, String) {
    let args: Vec<_> = env::args().collect();
    let arrangement_path = args.get(1).expect("missing arrangement file path");
    let move_path = args.get(2).expect("missing re-arrangement file path");

    let arrangement = fs::read_to_string(arrangement_path).expect("can not read arrangement file");
    let rearrangement = fs::read_to_string(move_path).expect("can not read re-arrangement file");

    (arrangement, rearrangement)
}

struct Crates {
    list: Vec<Vec<char>>,
}

impl str::FromStr for Crates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crates: Vec<Vec<char>> = vec![];
        let parsed_lines = s
            .lines()
            .map(|line| {
                line.chars()
                    .chunks(4)
                    .into_iter()
                    .map(|chunk| {
                        let crate_id = chunk.collect::<Vec<char>>()[1];
                        if crate_id >= 'A' && crate_id <= 'Z' {
                            crate_id
                        } else {
                            '_'
                        }
                    })
                    .collect::<Vec<char>>()
            })
            .rev()
            .collect::<Vec<_>>();

        for _ in 1..=parsed_lines[0].len() {
            crates.push(vec![]);
        }

        for line in parsed_lines {
            for (idx, crate_id) in line.iter().enumerate() {
                if *crate_id != '_' {
                    crates[idx].push(*crate_id);
                }
            }
        }

        Ok(Crates { list: crates })
    }
}

impl Crates {
    fn crate_mover_9000_execute(&mut self, mv_instruction: &MoveInstruction) {
        for _ in 1..=mv_instruction.count {
            match self.list[mv_instruction.source].pop() {
                Some(crate_id) => self.list[mv_instruction.target].push(crate_id),
                None => return,
            }
        }
    }

    fn crate_mover_9001_execute(&mut self, mv_instruction: &MoveInstruction) {
        let mut temp = vec![];

        for _ in 1..=mv_instruction.count {
            match self.list[mv_instruction.source].pop() {
                Some(crate_id) => temp.push(crate_id),
                None => return,
            }
        }

        for crate_id in temp.iter().rev() {
            self.list[mv_instruction.target].push(*crate_id);
        }
    }

    fn get_top_crates_string(&self) -> String {
        let mut result = String::new();
        for crate_list in &self.list {
            match crate_list.last() {
                Some(crate_id) => result.push(*crate_id),
                None => continue,
            }
        }

        result
    }
}

struct MoveInstruction {
    count: usize,
    source: usize,
    target: usize,
}

impl FromStr for MoveInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        let cap = re.captures(s).unwrap();
        if cap.len() < 4 {
            return Err(String::from("invalid instruction"));
        }

        Ok(MoveInstruction {
            count: cap[1].parse().unwrap(),
            source: cap[2].parse::<usize>().unwrap() - 1,
            target: cap[3].parse::<usize>().unwrap() - 1,
        })
    }
}

fn part_1(arrangement: &str, rearrangement: &str) -> String {
    let mut crates = Crates::from_str(arrangement).unwrap();
    rearrangement
        .lines()
        .map(|line| MoveInstruction::from_str(line).unwrap())
        .for_each(|instruction| crates.crate_mover_9000_execute(&instruction));

    crates.get_top_crates_string()
}

fn part_2(arrangement: &str, rearrangement: &str) -> String {
    let mut crates = Crates::from_str(arrangement).unwrap();
    rearrangement
        .lines()
        .map(|line| MoveInstruction::from_str(line).unwrap())
        .for_each(|instruction| crates.crate_mover_9001_execute(&instruction));

    crates.get_top_crates_string()
}

fn main() {
    let (arrangement, rearrangement) = read_input();
    println!("{}", part_1(&arrangement, &rearrangement));
    println!("{}", part_2(&arrangement, &rearrangement));
}
