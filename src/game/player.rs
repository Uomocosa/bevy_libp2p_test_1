use bevy::prelude::*;
use libp2p::PeerId;

use crate::p2p::protocol::PlayerInputData;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub peer_id: PeerId,
    pub is_local: bool,
}

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Component, Debug, Clone)]
pub struct PlayerInput {
    pub input: PlayerInputData,
}

impl PlayerInput {
    pub fn new() -> Self {
        Self {
            input: PlayerInputData::default(),
        }
    }

    pub fn set(&mut self, input: PlayerInputData) {
        self.input = input;
    }
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component, Debug, Clone)]
pub struct InputBuffer {
    pub inputs: Vec<(u64, PlayerInputData)>,
    pub max_size: usize,
}

impl InputBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            inputs: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, tick: u64, input: PlayerInputData) {
        if self.inputs.len() >= self.max_size {
            self.inputs.remove(0);
        }
        self.inputs.push((tick, input));
    }

    pub fn get(&self, tick: u64) -> Option<PlayerInputData> {
        self.inputs
            .iter()
            .find(|(t, _)| *t == tick)
            .map(|(_, input)| input.clone())
    }

    pub fn clear_before(&mut self, tick: u64) {
        self.inputs.retain(|(t, _)| *t >= tick);
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new(128)
    }
}
