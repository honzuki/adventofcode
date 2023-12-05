use std::ops::Range;

/// Merges together all the ranges that share boundaries in 'ranges'
pub fn compact_ranges<T>(mut ranges: Vec<Range<T>>) -> Vec<Range<T>>
where
    T: std::cmp::PartialOrd + std::cmp::Ord + Copy,
{
    // sort the ranges by their start position, and then end position
    ranges.sort_by_key(|range| (range.start, range.end));

    ranges.reverse(); // removing from the end is more efficient
    CompactRangesIter { ranges }.collect::<Vec<Range<T>>>()
}

// compacts the ranges in a sort list of ranges
// by merging all the ranges that share boundaries
struct CompactRangesIter<T> {
    ranges: Vec<Range<T>>,
}

impl<T> Iterator for CompactRangesIter<T>
where
    T: std::cmp::PartialOrd,
{
    type Item = Range<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.ranges.pop()?;

        while !self.ranges.is_empty() {
            if self.ranges[self.ranges.len() - 1].start >= current.end {
                // the next range starts a new range
                break;
            }

            // merge the ranges
            let next = self.ranges.pop().unwrap();
            current.end = next.end;
        }

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compact_ranges() {
        let input = vec![15u64..19, 18..36, 36..50, 40..99, 150..200];
        let expected_output = vec![15u64..36, 36..99, 150..200];

        assert_eq!(super::compact_ranges(input), expected_output);
    }
}
