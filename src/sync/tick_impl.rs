use crate::sync::tick::Tick;

impl Tick {
    pub fn next(&mut self) -> u64 {
        let current = self.0;
        self.0 = self.0.wrapping_add(1);
        current
    }

    pub fn current(&self) -> u64 {
        self.0
    }

    pub fn set(&mut self, tick: u64) {
        self.0 = tick;
    }
}
