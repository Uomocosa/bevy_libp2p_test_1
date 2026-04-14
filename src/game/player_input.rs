use crate::game::player::PlayerInput;
use crate::p2p::protocol::PlayerInputData;

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
