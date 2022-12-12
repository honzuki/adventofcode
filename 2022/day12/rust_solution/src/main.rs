use std::{env, error, fs, io, vec};

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

type Grid = Vec<Vec<Node>>;

#[derive(Copy, Clone, Debug)]
enum Node {
    Active(char),
    Visited,
}

fn parse_input(input: &str) -> Vec<Vec<Node>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| Node::Active(ch)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn find_in_grid(grid: &Grid, target: char) -> Option<(usize, usize)> {
    for (r, line) in grid.iter().enumerate() {
        for (c, ch) in line.iter().enumerate() {
            if let Node::Active(ch) = ch {
                if *ch == target {
                    return Some((r, c));
                }
            }
        }
    }

    None
}

fn ch_to_ind(ch: char) -> usize {
    if ch == 'S' {
        'a' as usize
    } else if ch == 'E' {
        'z' as usize
    } else {
        ch as usize
    }
}

fn part_1(mut grid: Grid) -> Option<usize> {
    static DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let start = find_in_grid(&grid, 'S')?;

    let mut frontier: Vec<(usize, usize)> = vec![start];

    let mut length = 0;
    while frontier.len() > 0 {
        length += 1;

        let mut next_frontier = Vec::new();
        while let Some((r, c)) = frontier.pop() {
            let ch = match grid[r][c] {
                Node::Active(ch) => ch,
                Node::Visited => continue,
            };

            let from_val = ch_to_ind(ch);
            for dir in DIRS {
                let new_r = r as isize + dir.0;
                let new_c = c as isize + dir.1;

                if new_r < 0
                    || new_c < 0
                    || new_r as usize >= grid.len()
                    || new_c as usize >= grid[new_r as usize].len()
                {
                    continue;
                }

                match grid[new_r as usize][new_c as usize] {
                    Node::Active(ch) => {
                        let to_val = ch_to_ind(ch);

                        if to_val > (from_val + 1) {
                            continue;
                        } else if ch == 'E' {
                            return Some(length);
                        }

                        next_frontier.push((new_r as usize, new_c as usize));
                    }
                    Node::Visited => {
                        continue;
                    }
                }
            }

            grid[r][c] = Node::Visited;
        }

        frontier = next_frontier;
    }

    None
}

fn part_2(mut grid: Grid) -> Option<usize> {
    static DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let start = find_in_grid(&grid, 'E')?;

    let mut frontier: Vec<(usize, usize)> = vec![start];

    let mut length = 0;
    while frontier.len() > 0 {
        length += 1;

        let mut next_frontier = Vec::new();
        while let Some((r, c)) = frontier.pop() {
            let ch = match grid[r][c] {
                Node::Active(ch) => ch,
                Node::Visited => continue,
            };

            let from_val = ch_to_ind(ch);
            for dir in DIRS {
                let new_r = r as isize + dir.0;
                let new_c = c as isize + dir.1;

                if new_r < 0
                    || new_c < 0
                    || new_r as usize >= grid.len()
                    || new_c as usize >= grid[new_r as usize].len()
                {
                    continue;
                }

                match grid[new_r as usize][new_c as usize] {
                    Node::Active(ch) => {
                        let to_val = ch_to_ind(ch);

                        if from_val > (to_val + 1) {
                            continue;
                        } else if ch == 'E' || ch == 'a' {
                            return Some(length);
                        }

                        next_frontier.push((new_r as usize, new_c as usize));
                    }
                    Node::Visited => {
                        continue;
                    }
                }
            }

            grid[r][c] = Node::Visited;
        }

        frontier = next_frontier;
    }

    None
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let input = parse_input(&input);
    println!("part 1 result: {}", part_1(input.clone()).unwrap());
    println!("part 2 result: {}", part_2(input.clone()).unwrap());

    Ok(())
}
