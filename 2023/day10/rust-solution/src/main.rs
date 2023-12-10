use std::collections::HashSet;

use sketch::{Pipe, Point, SketchErr};

use crate::sketch::{Sketch, Tile};

mod sketch;

fn part_1(input: &str) -> Result<Option<usize>, SketchErr> {
    let sketch: Sketch = input.parse()?;
    let start = sketch.start().ok_or(SketchErr::NoStart)?;

    // Get all points coming out of the start
    let mut frontier = start
        .all_dirs()
        .into_iter()
        .flatten()
        .map(|position| (1, start, position))
        .collect::<Vec<_>>();
    let mut visited = HashSet::from([start]);

    while !frontier.is_empty() {
        let mut new = vec![];
        while let Some((steps, prev, current)) = frontier.pop() {
            let pipe = match sketch.tile(current) {
                Some(Tile::Pipe(pipe)) => pipe,
                _ => continue, // ignore non-pipes
            };

            let ends = pipe.ends(current);

            if !ends.contains(&Some(prev)) {
                // never go to a pipe that doesn't go back to you
                continue;
            }

            if !visited.insert(current) {
                return Ok(Some(steps));
            }

            let next = match pipe.ends(current).iter().find(|&&pos| pos != Some(prev)) {
                Some(Some(next)) => *next,
                _ => continue,
            };

            new.push((steps + 1, current, next));
        }

        frontier = new;
    }

    Ok(None)
}

fn part_2(input: &str) -> Result<Option<usize>, SketchErr> {
    let sketch: Sketch = input.parse()?;
    let Some(lp) = get_loop(&sketch)? else {
        return Ok(None);
    };

    let start_goes_north = sketch
        .start()
        .unwrap()
        .north()
        .map(|position| {
            lp.contains(&position)
                && matches!(
                    sketch.tile(position),
                    Some(Tile::Pipe(
                        Pipe::SouthEast | Pipe::SouthWest | Pipe::Vertical
                    ))
                )
        })
        .unwrap_or(false);

    let enclosed = sketch
        .tiles
        .iter()
        .enumerate()
        .map(|(rdx, row)| {
            let mut count = 0;

            row.iter()
                .enumerate()
                .filter(|(cdx, col)| {
                    if lp.contains(&Point::new(*cdx, rdx)) {
                        if matches!(
                            col,
                            Tile::Pipe(Pipe::NorthEast | Pipe::NorthWest | Pipe::Vertical)
                        ) || (start_goes_north && matches!(col, Tile::Start))
                        {
                            count += 1;
                        }

                        false
                    } else {
                        count % 2 == 1
                    }
                })
                .count()
        })
        .sum();

    Ok(Some(enclosed))
}

fn get_loop(sketch: &Sketch) -> Result<Option<HashSet<Point>>, SketchErr> {
    let start = sketch.start().ok_or(SketchErr::NoStart)?;

    // get all tiles that are connected back to the start
    let connected_to_start = start.all_dirs().into_iter().flatten().collect::<Vec<_>>();

    'start_loop: for point in connected_to_start {
        let mut path = vec![start];
        let mut prev = start;
        let mut current = point;
        while current != start {
            let pipe = match sketch.tile(current) {
                Some(Tile::Pipe(pipe)) => pipe,
                _ => continue 'start_loop,
            };

            let ends = pipe.ends(current);
            if !ends.contains(&Some(prev)) {
                // never go to a pipe that doesn't go back to you
                continue 'start_loop;
            };

            let next = match ends.into_iter().find(|&pos| pos != Some(prev)) {
                Some(Some(next)) => next,
                _ => continue 'start_loop,
            };

            path.push(current);
            prev = current;
            current = next;
        }

        return Ok(Some(path.into_iter().collect()));
    }

    Ok(None)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {:?}", part_1(&input)?);
    println!("part 2: {:?}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        assert_eq!(super::part_1(input).unwrap().unwrap(), 8);
    }

    #[test]
    fn part_2_simple_maps() {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        assert_eq!(super::part_2(input).unwrap().unwrap(), 4);

        let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;
        assert_eq!(super::part_2(input).unwrap().unwrap(), 8);
    }

    #[test]
    fn part_2_complex_map() {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
        assert_eq!(super::part_2(input).unwrap().unwrap(), 10);
    }
}
