const NAMED_NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Calculates the calibration value of a line
pub fn calibration(line: &str) -> Option<u8> {
    let first = ForwardIter::new(line).next()?;
    let last = BackwardIter::new(line).next()?;

    Some(first * 10 + last)
}

/// An iterator over a string that returns all the digits it contains
/// this iterator goes from left-to-right, so `eightwo` would be considered as `8`
pub struct ForwardIter<'a> {
    input: &'a str,
}

impl<'a> ForwardIter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> Iterator for ForwardIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.chars();

        // loop through all substrings until you find a match
        loop {
            // is the first char a digit?
            let first_char = chars.next()?;
            if first_char.is_ascii_digit() {
                // remove the char
                self.input = &self.input[first_char.len_utf8()..];

                return Some(first_char.to_digit(10).unwrap() as u8);
            }

            for (idx, &name) in NAMED_NUMS.iter().enumerate() {
                // we've found an exact match
                if name.len() <= self.input.len() && name == &self.input[..name.len()] {
                    // remove the name from the input
                    self.input = &self.input[name.len()..];

                    return Some(idx as u8 + 1);
                }
            }

            // move 1 char ahead
            self.input = &self.input[first_char.len_utf8()..];
        }
    }
}

/// An iterator over a string the returns all the digits it contains
/// this iterator goes from right-to-left, so 'eightwo' would be considered as '2'
pub struct BackwardIter<'a> {
    input: &'a str,
}

impl<'a> BackwardIter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> Iterator for BackwardIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.chars();

        // loop through all substrings until you find a match
        loop {
            // is the last char a digit?
            let last_char = chars.next_back()?;
            if last_char.is_ascii_digit() {
                // remove the char
                self.input = &self.input[..self.input.len() - last_char.len_utf8()];

                return Some(last_char.to_digit(10).unwrap() as u8);
            }

            for (idx, &name) in NAMED_NUMS.iter().enumerate() {
                // we've found an exact match
                if name.len() <= self.input.len()
                    && name == &self.input[self.input.len() - name.len()..]
                {
                    // remove the name from the input
                    self.input = &self.input[..self.input.len() - name.len()];

                    return Some(idx as u8 + 1);
                }
            }

            // move 1 char ahead
            self.input = &self.input[..self.input.len() - last_char.len_utf8()];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::calibration::BackwardIter;

    use super::ForwardIter;

    #[test]
    fn forward_test() {
        let inputs = [
            "two1nine",
            "eightwothreeeightwo",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let expected_outputs: &[&[u8]] = &[
            &[2, 1, 9],
            &[8, 3, 8],
            &[1, 2, 3],
            &[2, 3, 4],
            &[4, 9, 8, 7, 2],
            &[1, 2, 3, 4],
            &[7, 6],
        ];

        for (&input, &expected) in inputs.iter().zip(expected_outputs.iter()) {
            let output = ForwardIter::new(input).collect::<Vec<_>>();
            assert_eq!(&output, expected);
        }
    }

    #[test]
    fn backward_test() {
        let inputs = [
            "two1nine",
            "eightwothreeeightwo",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let expected_outputs: &[&[u8]] = &[
            &[9, 1, 2],
            &[2, 3, 2],
            &[3, 2, 1],
            &[4, 3, 1],
            &[2, 7, 8, 9, 4],
            &[4, 3, 2, 8],
            &[6, 7],
        ];

        for (&input, &expected) in inputs.iter().zip(expected_outputs.iter()) {
            let output = BackwardIter::new(input).collect::<Vec<_>>();
            assert_eq!(&output, expected);
        }
    }

    #[test]
    fn calibration_value() {
        let inputs = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let expected_outputs: &[u8] = &[29, 83, 13, 24, 42, 14, 76];

        for (&input, &expected) in inputs.iter().zip(expected_outputs.iter()) {
            assert_eq!(super::calibration(input), Some(expected));
        }
    }
}
