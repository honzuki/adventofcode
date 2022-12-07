use std::{collections::HashMap, env, fs, io};

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

type FileSystem<'a> = HashMap<Vec<&'a str>, usize>;

fn parse_input(input: &str) -> FileSystem {
    let mut path: Vec<&str> = Vec::new();
    let mut dirs = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        match parts[0] {
            "$" => match parts[1] {
                "cd" => {
                    if parts.len() < 3 {
                        continue;
                    }
                    match parts[2] {
                        "/" => path = vec![],
                        ".." => {
                            path.pop();
                        }
                        _ => path.push(parts[2]),
                    };
                }
                _ => continue,
            },
            "dir" => continue,
            _ => {
                let size: usize = match parts[0].parse() {
                    Ok(size) => size,
                    Err(_) => continue,
                };

                for idx in 0..=path.len() {
                    dirs.entry(path[0..idx].to_vec())
                        .and_modify(|dir_size| *dir_size += size)
                        .or_insert(size);
                }
            }
        }
    }

    dirs
}

fn part_1(dirs: &FileSystem, size_threshold: usize) -> usize {
    dirs.iter()
        .filter_map(|(_, size)| {
            if *size <= size_threshold {
                Some(size)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(dirs: &FileSystem, disk_size: usize, needed_size: usize) -> usize {
    let used_space: usize = *dirs.get(&Vec::new()).unwrap();
    let free_space = disk_size - used_space;
    if needed_size <= free_space {
        // we already have enough free space
        return 0;
    }
    let to_free = needed_size - free_space;

    dirs.iter()
        .filter_map(|(_, size)| if *size >= to_free { Some(*size) } else { None })
        .min()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;
    let dirs = parse_input(&input);
    println!("part 1 result: {}", part_1(&dirs, 100000));
    println!("part 2 result: {}", part_2(&dirs, 70000000, 30000000));

    Ok(())
}
