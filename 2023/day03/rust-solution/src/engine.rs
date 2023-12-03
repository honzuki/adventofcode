use std::ops::Range;

use crate::helpers::{clamp_down, clamp_up};

#[derive(Debug, Clone, Default)]
pub struct Gear {
    adj_parts: Vec<PartData>,
}

impl Gear {
    pub fn adj_count(&self) -> usize {
        self.adj_parts.len()
    }

    pub fn ratio(&self) -> u32 {
        self.adj_parts.iter().fold(1u32, |m, part| part.number * m)
    }

    pub fn add_part(&mut self, part: PartData) {
        self.adj_parts.push(part);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartData {
    pub number: u32,
    pub range: Range<usize>,
}

/// Iterate over the parts in the scheme
pub fn iter_parts(scheme: &str) -> impl Iterator<Item = PartData> + '_ {
    PartIter::new(scheme)
}

struct PartIter<'a> {
    scheme: &'a str,
    base: usize,
    total_length: usize,
}

impl<'a> PartIter<'a> {
    fn new(scheme: &'a str) -> Self {
        Self {
            scheme,
            base: 0,
            total_length: scheme.len(),
        }
    }
}

impl<'a> Iterator for PartIter<'a> {
    type Item = PartData;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.scheme.chars();

        // for each ch in scheme
        loop {
            let ch = iter.next()?;

            // if it's a start of a number
            if let Some(mut number) = ch.to_digit(10) {
                let mut length = 1;

                // read the rest of the number
                loop {
                    let Some(ch) = iter.next() else {
                        break;
                    };

                    match ch.to_digit(10) {
                        Some(digit) => number = (number * 10) + digit,
                        None => break,
                    }

                    length += 1;
                }

                let start = clamp_down(self.base, 1);
                let end = clamp_up(self.base + length, 1, self.total_length);
                self.scheme = &self.scheme[length..];
                self.base += length;
                return Some(PartData {
                    number,
                    range: (start..end),
                });
            }

            self.scheme = &self.scheme[ch.len_utf8()..];
            self.base += ch.len_utf8();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{iter_parts, PartData};

    #[test]
    fn iterator_scheme_line() {
        let inputs = ["467..114..", "...*......", "..35..633."];
        let expected_outputs: &[&[PartData]] = &[
            &[
                PartData {
                    number: 467,
                    range: (0..4),
                },
                PartData {
                    number: 114,
                    range: (4..9),
                },
            ],
            &[],
            &[
                PartData {
                    number: 35,
                    range: (1..5),
                },
                PartData {
                    number: 633,
                    range: (5..10),
                },
            ],
        ];

        for (input, &expected) in inputs.into_iter().zip(expected_outputs.iter()) {
            let output = iter_parts(input).collect::<Vec<_>>();
            assert_eq!(output, expected);
        }
    }
}
