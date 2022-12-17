use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Tile {
    data: Vec<Vec<bool>>,
}

impl Tile {
    pub fn as_raw(&self) -> &[Vec<bool>] {
        &self.data
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .rev()
            .map(|line| {
                line.chars()
                    .map(|tile| match tile {
                        '#' => true,
                        _ => false,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Tile { data })
    }
}
