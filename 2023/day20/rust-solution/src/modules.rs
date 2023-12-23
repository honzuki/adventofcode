use std::{collections::BTreeMap, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
    Unknown(Unknown),
}

impl Module {
    pub fn flipflop() -> Self {
        Self::FlipFlop(FlipFlop { status: false })
    }

    pub fn conjunction() -> Self {
        Self::Conjunction(Conjunction {
            mem: Default::default(),
            on: 0,
        })
    }

    pub fn broadcast() -> Self {
        Self::Broadcast(Broadcast {})
    }

    pub fn unknown() -> Self {
        Self::Unknown(Unknown {})
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlipFlop {
    status: bool,
}

impl FlipFlop {
    pub fn receive(&mut self, pulse: bool) -> Option<bool> {
        if pulse {
            return None;
        }

        self.status = !self.status;
        Some(self.status)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Conjunction {
    mem: BTreeMap<String, bool>,
    on: usize,
}

impl Conjunction {
    pub fn receive(&mut self, from: &str, pulse: bool) -> bool {
        match (pulse, self.mem.insert(from.into(), pulse)) {
            (true, Some(false)) => self.on += 1,
            (false, Some(true)) => self.on -= 1,
            _ => {}
        };

        self.on != self.mem.len()
    }

    pub fn connect_input(&mut self, from: &str) {
        self.mem.insert(from.into(), false);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Broadcast {}

impl Broadcast {
    pub fn receive(&mut self, pulse: bool) -> bool {
        pulse
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unknown {}

impl Unknown {
    #[tracing::instrument]
    pub fn receive(&mut self, name: &str, from: &str, pulse: bool) {}
}

#[cfg(test)]
mod tests {
    use super::{Conjunction, FlipFlop};

    #[test]
    fn test_flip_flop() {
        let mut ff = FlipFlop { status: false };

        assert!(ff.receive(true).is_none());
        assert_eq!(ff.receive(false), Some(true));
        assert!(ff.receive(true).is_none());
        assert_eq!(ff.receive(false), Some(false));
        assert_eq!(ff.receive(false), Some(true));
        assert_eq!(ff.receive(false), Some(false));
        assert!(ff.receive(true).is_none());
        assert_eq!(ff.receive(false), Some(true));
    }

    #[test]
    fn test_multi_input_conjunction() {
        let mut con = Conjunction {
            mem: Default::default(),
            on: 0,
        };

        con.connect_input("a");
        con.connect_input("b");
        con.connect_input("c");
        con.connect_input("d");

        assert!(con.receive("a", true));
        assert!(con.receive("a", true));
        assert!(con.receive("b", true));
        assert!(con.receive("c", true));
        assert!(con.receive("a", false));
        assert!(con.receive("d", true));
        assert!(!con.receive("a", true));
    }

    #[test]
    fn test_single_input_conjunction() {
        let mut con = Conjunction {
            mem: Default::default(),
            on: 0,
        };

        con.connect_input("a");

        assert!(!con.receive("a", true));
        assert!(con.receive("a", false));
        assert!(!con.receive("a", true));
        assert!(con.receive("a", false));
    }
}
