use std::{env, error, fs, io};

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[derive(Debug, Clone)]
enum Node {
    Visible(i32),
    Invisible(i32),
}

fn part_1(input: &Vec<Vec<i32>>) -> u32 {
    let mut input = input
        .iter()
        .map(|line| {
            line.iter()
                .map(|num| Node::Invisible(*num))
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<_>>();

    let mut visible_trees = 0;
    // for the rows
    for line in &mut input {
        // right to left
        let mut max = -1;
        line.iter_mut().for_each(|node| match node {
            Node::Visible(height) => max = max.max(*height),
            Node::Invisible(height) => {
                if *height > max {
                    max = *height;
                    visible_trees += 1;
                    *node = Node::Visible(*height);
                }
            }
        });

        // left to right
        let mut max = -1;
        line.iter_mut().rev().for_each(|node| match node {
            Node::Visible(height) => max = max.max(*height),
            Node::Invisible(height) => {
                if *height > max {
                    max = *height;
                    visible_trees += 1;
                    *node = Node::Visible(*height);
                }
            }
        });
    }

    // for the columns
    for cid in 0..input[0].len() {
        // top to bottom
        let mut max = -1;
        for rid in 0..input.len() {
            let node = &mut input[rid][cid];
            match node {
                Node::Visible(height) => max = max.max(*height),
                Node::Invisible(height) => {
                    if *height > max {
                        max = *height;
                        visible_trees += 1;
                        *node = Node::Visible(*height);
                    }
                }
            }
        }

        // bottom to top
        let mut max = -1;
        for rid in (0..input.len()).rev() {
            let node = &mut input[rid][cid];
            match node {
                Node::Visible(height) => max = max.max(*height),
                Node::Invisible(height) => {
                    if *height > max {
                        max = *height;
                        visible_trees += 1;
                        *node = Node::Visible(*height);
                    }
                }
            }
        }
    }

    visible_trees
}

fn scenic_score(input: &Vec<Vec<i32>>, row_id: usize, col_id: usize) -> u32 {
    let target_height = input[row_id][col_id];
    let mut up = 0;
    let mut down = 0;
    let mut right = 0;
    let mut left = 0;
    for rid in (0..row_id).rev() {
        let height = input[rid][col_id];
        up += 1;
        if height >= target_height {
            break;
        }
    }

    for rid in (row_id + 1)..input.len() {
        let height = input[rid][col_id];
        down += 1;
        if height >= target_height {
            break;
        }
    }

    for cid in (0..col_id).rev() {
        let height = input[row_id][cid];
        left += 1;
        if height >= target_height {
            break;
        }
    }

    for cid in (col_id + 1)..input[0].len() {
        let height = input[row_id][cid];
        right += 1;
        if height >= target_height {
            break;
        }
    }

    up * down * right * left
}

fn part_2(input: &Vec<Vec<i32>>) -> u32 {
    let mut max = 0;
    for rid in 0..input.len() {
        for cid in 0..input[rid].len() {
            max = max.max(scenic_score(input, rid, cid))
        }
    }

    max
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let input = parse_input(&input);
    println!("part 1 solution: {}", part_1(&input));
    println!("part 2 solution: {}", part_2(&input));

    Ok(())
}
