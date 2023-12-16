use contraption::{Grid, GridErr};
use rayon::iter::{ParallelBridge, ParallelIterator};

mod contraption;

fn part_1(input: &str) -> Result<usize, GridErr> {
    let grid: Grid = input.parse()?;
    Ok(grid.calculate_energized((0, 0), contraption::Dir::Right))
}

fn part_2(input: &str) -> Result<usize, GridErr> {
    let grid: Grid = input.parse()?;

    let (rlen, clen) = grid.len();
    let all_possible_starts = (0..rlen)
        .flat_map(|ridx| {
            [
                ((ridx, 0), contraption::Dir::Right),
                ((ridx, clen - 1), contraption::Dir::Left),
            ]
        })
        .chain((0..clen).flat_map(|cidx| {
            [
                ((0, cidx), contraption::Dir::Down),
                ((rlen - 1, cidx), contraption::Dir::Up),
            ]
        }));

    Ok(all_possible_starts
        .par_bridge()
        .map(|(pos, dir)| grid.calculate_energized(pos, dir))
        .max()
        .unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|...."#;
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 46);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 51);
    }
}
