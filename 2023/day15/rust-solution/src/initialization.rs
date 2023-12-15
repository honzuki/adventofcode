use std::{
    collections::{hash_map, HashMap},
    str::FromStr,
};

pub fn hash(data: &str) -> u8 {
    data.bytes().fold(0u16, |hash, byte| {
        ((hash + byte as u16) * 17).rem_euclid(256)
    }) as u8
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IBox {
    data: HashMap<String, (u32, u8)>,
    next_id: u32,
}

impl IBox {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            next_id: 0,
        }
    }

    pub fn focusing_power(&self) -> u32 {
        let mut lens = self.data.values().copied().collect::<Vec<_>>();
        lens.sort();
        lens.into_iter()
            .enumerate()
            .map(|(idx, (_, focal))| (idx as u32 + 1) * focal as u32)
            .sum()
    }

    pub fn execute(&mut self, step: Step) {
        match step.method {
            Method::Remove => self.remove(&step.label),
            Method::Insert(focal) => self.insert(step.label, focal),
        }
    }

    pub fn insert(&mut self, label: String, focal: u8) {
        match self.data.entry(label) {
            hash_map::Entry::Occupied(mut entry) => {
                let id = entry.get().0;
                entry.insert((id, focal));
            }
            hash_map::Entry::Vacant(entry) => {
                let id = self.next_id;
                self.next_id += 1;

                entry.insert((id, focal));
            }
        }
    }

    pub fn remove(&mut self, label: &str) {
        self.data.remove(label);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StepErr {
    #[error("failed to parse the step due to unknown format")]
    UnknownStep,

    #[error("can not parse the focal value: {0}")]
    BadFocalValue(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Method {
    Insert(u8),
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    label: String,
    method: Method,
}

impl Step {
    pub fn ibox(&self) -> u8 {
        hash(&self.label)
    }
}

impl FromStr for Step {
    type Err = StepErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some((label, "")) = s.split_once('-') {
            return Ok(Self {
                label: label.into(),
                method: Method::Remove,
            });
        }

        if let Some((label, focal)) = s.split_once('=') {
            let focal: u8 = focal
                .parse()
                .map_err(|_| StepErr::BadFocalValue(focal.into()))?;

            return Ok(Self {
                label: label.into(),
                method: Method::Insert(focal),
            });
        }

        Err(StepErr::UnknownStep)
    }
}

#[cfg(test)]
mod tests {
    use super::{hash, IBox, Method, Step};

    #[test]
    fn test_hash() {
        let input = "rn=1";
        let expected_output = 30;

        assert_eq!(hash(input), expected_output);
    }

    #[test]
    fn parse_step() {
        let inputs = ["rn=1", "cm-", "qp=3"];
        let expected_outputs = [
            Step {
                label: "rn".into(),
                method: Method::Insert(1),
            },
            Step {
                label: "cm".into(),
                method: Method::Remove,
            },
            Step {
                label: "qp".into(),
                method: Method::Insert(3),
            },
        ];

        for (input, expected) in inputs.into_iter().zip(expected_outputs) {
            let output: Step = input.parse().unwrap();
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn test_ibox() {
        let mut ibox = IBox::new();
        ibox.insert("test".into(), 2);
        ibox.insert("test2".into(), 3);
        ibox.insert("test3".into(), 4);
        ibox.remove("test2");
        assert_eq!(ibox.focusing_power(), 2 + 8);

        ibox.remove("test4");
        ibox.remove("test2");
        assert_eq!(ibox.focusing_power(), 2 + 8);
    }
}
