use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Range(isize, isize);

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.1 < other.0 {
            Some(Ordering::Less)
        } else if self.0 > other.1 {
            Some(Ordering::Greater)
        } else if self.0 == other.0 && self.1 == other.1 {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl Range {
    pub fn new(start: isize, end: isize) -> Range {
        Range(start, end)
    }

    pub fn merge(self, other: Self) -> Self {
        Range(self.0.min(other.0), self.1.max(other.1))
    }

    pub fn size(&self) -> usize {
        self.0.abs_diff(self.1)
    }
}

#[derive(Debug)]
pub struct Ranges {
    data: Vec<Range>,
}

impl Ranges {
    pub fn new() -> Ranges {
        Ranges { data: Vec::new() }
    }

    pub fn add_range(&mut self, mut range: Range) {
        'main_loop: loop {
            for idx in 0..self.data.len() {
                match self.data[idx].partial_cmp(&range) {
                    Some(Ordering::Less) => continue,
                    Some(Ordering::Equal) => return,
                    Some(Ordering::Greater) => {
                        self.data.insert(idx, range);
                        return;
                    }
                    None => {
                        // we need to merge them
                        range = self.data.remove(idx).merge(range);
                        continue 'main_loop; // try adding it again
                    }
                }
            }

            self.data.push(range);
            return;
        }
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        for range in &self.data {
            size += range.size();
        }
        size
    }

    pub fn get_empty(&self, limit: usize) -> Option<usize> {
        if self.size() >= limit {
            None
        } else {
            if self.data[0].1 < limit as isize {
                Some((self.data[0].1 + 1) as usize)
            } else {
                Some((self.data[0].0 - 1) as usize)
            }
        }
    }
}
