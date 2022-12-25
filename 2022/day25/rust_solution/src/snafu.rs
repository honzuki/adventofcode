#[derive(Debug, Clone, Copy)]
pub enum ConversionErr {
    UnknownFormat(char),
}

pub trait SNAFU {
    fn from_snafu(s: &str) -> Result<Self, ConversionErr>
    where
        Self: Sized;

    fn to_snafu(&self) -> String;
}

impl SNAFU for i64 {
    fn from_snafu(s: &str) -> Result<Self, ConversionErr> {
        let get_power = |p| 5i64.pow(p as u32);

        s.chars()
            .rev()
            .enumerate()
            .map(|(idx, ch)| match ch {
                '2' | '1' | '0' => Ok(ch.to_digit(10).unwrap() as Self * get_power(idx)),
                '-' => Ok(-1 * get_power(idx)),
                '=' => Ok(-2 * get_power(idx)),
                _ => Err(ConversionErr::UnknownFormat(ch)),
            })
            .sum::<Result<i64, _>>()
    }

    fn to_snafu(&self) -> String {
        let mut left = *self;

        let mut snafu = String::new();
        let mut rem = 0;
        while left > 0 {
            let current = left.rem_euclid(5);
            let digit = current + rem;

            rem = 0;
            match digit {
                0 => snafu.push('0'),
                1 => snafu.push('1'),
                2 => snafu.push('2'),
                3 => {
                    snafu.push('=');
                    rem = 1;
                }
                4 => {
                    snafu.push('-');
                    rem = 1;
                }
                5 => {
                    snafu.push('0');
                    rem = 1;
                }
                _ => {}
            }

            left /= 5;
        }

        if rem == 1 {
            snafu.push('1');
        }

        snafu.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_from_snafu() {
        let (to_convert, expected) = get_example_numbers();

        let result = to_convert
            .iter()
            .map(|&num| i64::from_snafu(num).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(result, expected)
    }

    #[test]
    fn check_to_snafu() {
        let (expected, to_convert) = get_example_numbers();

        let result = to_convert
            .iter()
            .map(|&num| num.to_snafu())
            .collect::<Vec<_>>();

        assert_eq!(result, expected)
    }

    fn get_example_numbers() -> (Vec<&'static str>, Vec<i64>) {
        let snafu_format = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;

        let i64_format = r#"1747
906
198
11
201
31
1257
32
353
107
7
3
37"#;

        (
            snafu_format.lines().collect::<Vec<&str>>(),
            i64_format
                .lines()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        )
    }
}
