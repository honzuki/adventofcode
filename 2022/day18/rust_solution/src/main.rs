use std::{
    collections::{hash_set::Iter, HashSet},
    env, error, fs, io,
    str::FromStr,
};

mod cube;
use cube::{Cube, CubeError};

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn parse_input(input: &str) -> Result<HashSet<Cube>, CubeError> {
    input
        .lines()
        .map(|line| Cube::from_str(line))
        .collect::<Result<HashSet<_>, _>>()
}

fn part_1(input: &HashSet<Cube>) -> usize {
    let mut unconnected_sides = 0;

    for cube in input {
        for connected_cube in cube.connected() {
            if !input.contains(&connected_cube) {
                unconnected_sides += 1;
            }
        }
    }

    unconnected_sides
}

fn get_max_cube(iterator: Iter<Cube>) -> Cube {
    let mut max = Cube::new(0, 0, 0);
    for cube in iterator {
        max = cube.max(&max);
    }
    max
}

// If it can not reach the bounding cube, it will return a list
// of all the "air" cubes it reached
fn bounded_bfs(
    start: &Cube,
    lower_bound: &Cube,
    upper_bound: &Cube,
    existing: &HashSet<Cube>,
) -> Option<HashSet<Cube>> {
    let mut visited = HashSet::new();

    let mut frontier = vec![*start];
    while let Some(cube) = frontier.pop() {
        if !cube.in_bounds(&lower_bound, &upper_bound) {
            // isn't trapped between lava droplets
            return None;
        }

        for connected_cube in cube.connected() {
            if !existing.contains(&connected_cube) && !visited.contains(&connected_cube) {
                visited.insert(connected_cube);
                frontier.push(connected_cube);
            }
        }
    }

    Some(visited)
}

fn part_2(input: &HashSet<Cube>) -> usize {
    let bounding_cube = get_max_cube(input.iter());

    let mut unconnected_sides = 0;

    let mut air_pockets = HashSet::new();
    let mut not_air_pocket = HashSet::new();

    for cube in input {
        for connected_cube in cube.connected() {
            if !input.contains(&connected_cube) && !air_pockets.contains(&connected_cube) {
                if not_air_pocket.contains(&connected_cube) {
                    // fast way out
                    unconnected_sides += 1;
                    continue;
                }

                // check for an air pocket
                match bounded_bfs(&connected_cube, &Cube::new(0, 0, 0), &bounding_cube, &input) {
                    None => {
                        unconnected_sides += 1;
                        not_air_pocket.insert(connected_cube);
                    }
                    Some(visited) => {
                        for cube in visited {
                            air_pockets.insert(cube);
                        }
                    }
                }
            }
        }
    }

    unconnected_sides
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;
    let input = parse_input(&input)?;

    println!("part 1 result: {}", part_1(&input));
    println!("part 2 result: {}", part_2(&input));

    Ok(())
}
