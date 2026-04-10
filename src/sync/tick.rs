use bevy::prelude::Resource;

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct Tick(pub u64);

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

pub const TICKS_PER_SECOND: u64 = 64;
pub const TICK_DURATION: f32 = 1.0 / TICKS_PER_SECOND as f32;
