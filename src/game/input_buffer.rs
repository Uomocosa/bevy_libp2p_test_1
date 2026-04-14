use crate::game::player::InputBuffer;
use crate::p2p::protocol::PlayerInputData;

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
