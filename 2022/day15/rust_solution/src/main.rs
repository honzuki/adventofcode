use std::{env, error, fs, io, str::FromStr};

mod scan;
use scan::{Position, Scan};

mod ranges;
use ranges::{Range, Ranges};

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn parse_input(input: &str) -> Result<Vec<Scan>, String> {
    input
        .lines()
        .map(|line| Scan::from_str(line))
        .collect::<Result<_, _>>()
}

fn part_1(input: &Vec<Scan>, target_row: isize) -> usize {
    let mut ranges = Ranges::new();

    for scan in input {
        let dist_from_target = scan.sensor.y_dist(target_row);
        let dist_from_beacon = scan.sensor.manhattan_distance(&scan.beacon);

        let range_size = dist_from_beacon as isize - dist_from_target as isize;
        if range_size < 0 {
            // this sensor can not tell us anything about the target row
            continue;
        }

        ranges.add_range(Range::new(
            scan.sensor.get_x() - range_size,
            scan.sensor.get_x() + range_size,
        ));
    }

    ranges.size()
}

fn part_2(input: &Vec<Scan>, max_x: usize, max_y: usize) -> Option<usize> {
    for target_row in 0..=max_y {
        let mut ranges = Ranges::new();
        for scan in input {
            let dist_from_target = scan.sensor.y_dist(target_row as isize);
            let dist_from_beacon = scan.sensor.manhattan_distance(&scan.beacon);

            let range_size = dist_from_beacon as isize - dist_from_target as isize;
            if range_size < 0 {
                // this sensor can not tell us anything about the target row
                continue;
            }

            ranges.add_range(Range::new(
                (scan.sensor.get_x() - range_size)
                    .max(0)
                    .min(max_x as isize),
                (scan.sensor.get_x() + range_size)
                    .max(0)
                    .min(max_x as isize),
            ));
        }

        match ranges.get_empty(max_x) {
            None => continue,
            Some(x) => return Some(x * max_x + target_row),
        }
    }

    None
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let input = parse_input(&input)?;
    println!("part 1 result: {}", part_1(&input, 2000000));
    println!(
        "part 2 result: {:?}",
        part_2(&input, 4000000, 4000000).unwrap()
    );

    Ok(())
}
