use bevy::prelude::*;

#[derive(Component)]
pub struct ClickCounter(pub u32);

impl ClickCounter {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let mut counter = ClickCounter(0);
        assert_eq!(counter.0, 0, "Initial value should be 0");

        counter.increment();
        assert_eq!(counter.0, 1, "Should increment to 1");

        counter.increment();
        assert_eq!(counter.0, 2, "Should increment to 2");
    }
}
