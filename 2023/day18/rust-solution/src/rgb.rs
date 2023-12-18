use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum RgbErr {
    #[error("unknown rgb format")]
    UnknownFormat,

    #[error("expected 3 pairs of hexadecimal characters")]
    BadFormat,

    #[error("{0} is not an hexadecimal number")]
    ParseErr(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl FromStr for Rgb {
    type Err = RgbErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(("", rgb)) = s.split_once('#') else {
            return Err(RgbErr::UnknownFormat);
        };

        if rgb.len() != 6 {
            return Err(RgbErr::BadFormat);
        }

        let mut parts = [&rgb[0..2], &rgb[2..4], &rgb[4..6]]
            .into_iter()
            .map(|color| u8::from_str_radix(color, 16).map_err(|_| RgbErr::ParseErr(color.into())));

        let red = parts.next().unwrap()?;
        let green = parts.next().unwrap()?;
        let blue = parts.next().unwrap()?;

        Ok(Self { red, green, blue })
    }
}

#[cfg(test)]
mod tests {
    use crate::rgb::Rgb;

    #[test]
    fn parse_rgb() {
        let input = "#70c710";
        let expected_output = Rgb {
            red: 0x70,
            green: 0xc7,
            blue: 0x10,
        };
        let output: Rgb = input.parse().unwrap();

        assert_eq!(output, expected_output);
    }
}
