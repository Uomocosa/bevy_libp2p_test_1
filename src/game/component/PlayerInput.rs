use crate::p2p::protocol::PlayerInputData;
use bevy::prelude::*;

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
