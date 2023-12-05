use almanac::{get_seed_ranges, ParseErr};

use crate::almanac::Almanac;

mod almanac;
mod ranges;

fn part_1(input: &str) -> Result<u64, ParseErr> {
    let almanac: Almanac = input.parse()?;

    let mut map = almanac
        .maps
        .get("seed")
        .ok_or_else(|| ParseErr::MissingMap("seed".into()))?;

    let mut locations = almanac.seeds.clone();
    loop {
        locations.iter_mut().for_each(|seed| *seed = map.map(*seed));

        if map.to() == "location" {
            // we reached the destination
            break;
        }

        // move to the next map
        map = almanac
            .maps
            .get(map.to())
            .ok_or_else(|| ParseErr::MissingMap(map.to().into()))?;
    }

    Ok(locations.into_iter().min().unwrap())
}

fn part_2(input: &str) -> Result<u64, ParseErr> {
    let almanac: Almanac = input.parse()?;

    let mut map = almanac
        .maps
        .get("seed")
        .ok_or_else(|| ParseErr::MissingMap("seed".into()))?;

    let mut locations = get_seed_ranges(&almanac.seeds)?;
    loop {
        locations = locations
            .into_iter()
            .flat_map(|range| map.map_range(range))
            .collect();
        locations = ranges::compact_ranges(locations);

        if map.to() == "location" {
            // we reached the destination
            break;
        }

        // move to the next map
        map = almanac
            .maps
            .get(map.to())
            .ok_or_else(|| ParseErr::MissingMap(map.to().into()))?;
    }

    Ok(locations
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part_1 = {}", part_1(&input)?);
    println!("part_2 = {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT).unwrap(), 35);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 46);
    }
}
