pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            count : 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            0..=5 => {
                self.count += 1;
                Some(self.count)
            },
            _ => None,
        }
    }
}