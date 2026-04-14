use bevy::prelude::Resource;

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct Tick(pub u64);

pub const TICKS_PER_SECOND: u64 = 64;
pub const TICK_DURATION: f32 = 1.0 / TICKS_PER_SECOND as f32;
