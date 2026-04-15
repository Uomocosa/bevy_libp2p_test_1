use crate::sync::tick::Tick;

impl Tick {
    #[allow(clippy::should_implement_trait)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let mut tick = Tick::default();
        assert_eq!(tick.current(), 0, "Initial tick should be 0");

        let prev = tick.next();
        assert_eq!(prev, 0, "next() should return previous value");
        assert_eq!(tick.current(), 1, "Tick should increment after next()");

        tick.set(100);
        assert_eq!(tick.current(), 100, "set should update tick value");
    }
}
