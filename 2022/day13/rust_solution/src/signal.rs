use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Data {
    Pure(u32),
    List(Vec<Data>),
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Pure(v1), Self::Pure(v2)) => {
                if v1 < v2 {
                    std::cmp::Ordering::Less
                } else if v1 == v2 {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            }
            (Self::Pure(v1), Self::List(_)) => Self::List(vec![Data::Pure(*v1)]).cmp(other),
            (Self::List(_), Self::Pure(v2)) => self.cmp(&Data::List(vec![Data::Pure(*v2)])),
            (Self::List(l1), Self::List(l2)) => {
                for (d1, d2) in l1.iter().zip(l2) {
                    return match d1.cmp(d2) {
                        std::cmp::Ordering::Equal => continue,
                        value => value,
                    };
                }

                if l1.len() == l2.len() {
                    std::cmp::Ordering::Equal
                } else if l1.len() < l2.len() {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Packet {
    data: Data,
}

impl Packet {
    pub fn in_order(&self, other: &Packet) -> bool {
        match self.data.cmp(&other.data) {
            std::cmp::Ordering::Less => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PacketErr {
    InvalidPacket,
}

fn split_by_level(s: &str, delim: char) -> Vec<&str> {
    let mut result = Vec::new();

    let mut level = 0;
    let mut last_split = 0;
    for (idx, ch) in s.chars().enumerate() {
        if ch == '[' {
            level += 1;
        } else if ch == ']' {
            level -= 1;
        } else if level == 0 && ch == delim {
            result.push(&s[last_split..idx]);
            last_split = idx + 1;
        }
    }
    if last_split < s.len() {
        result.push(&s[last_split..s.len()]);
    }

    result
}

impl FromStr for Data {
    type Err = PacketErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Ok(Data::List(Vec::new()));
        }

        Ok(Data::List(
            split_by_level(s, ',')
                .iter()
                .map(|part| match part.parse::<u32>() {
                    Ok(value) => Ok(Data::Pure(value)),
                    Err(_) => Self::from_str(&part[1..part.len() - 1]),
                })
                .collect::<Result<Vec<Data>, _>>()?,
        ))
    }
}

impl FromStr for Packet {
    type Err = PacketErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Data::from_str(&s[1..s.len() - 1])? {
            Data::Pure(_) => Err(PacketErr::InvalidPacket),
            Data::List(data) => Ok(Packet {
                data: Data::List(data),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse_method() {
        println!("{:?}", Packet::from_str("[1,1,3,1,1]").unwrap());
        println!("{:?}", Packet::from_str("[[[]]]").unwrap());
        println!("{:?}", Packet::from_str("[[1],[2,3,4]]").unwrap());
    }

    #[test]
    fn check_equality() {
        let p1 = Packet::from_str("[1,1,3,1,1]").unwrap();
        let p2 = Packet::from_str("[1,1,3,1,1]").unwrap();
        assert_eq!(p1 == p2, true)
    }
}
