use bevy::prelude::*;
use libp2p::PeerId;
use std::collections::HashMap;

use crate::p2p::protocol::PlayerInputData as RemoteInputData;

#[derive(Resource)]
pub struct RemoteInputBuffer {
    inputs: HashMap<PeerId, Vec<(u64, RemoteInputData)>>,
    max_size: usize,
}

impl Default for RemoteInputBuffer {
    fn default() -> Self {
        Self {
            inputs: HashMap::new(),
            max_size: 256,
        }
    }
}

impl RemoteInputBuffer {
    pub fn push(&mut self, peer_id: PeerId, tick: u64, input: RemoteInputData) {
        let peer_inputs = self.inputs.entry(peer_id).or_default();
        if peer_inputs.len() >= self.max_size {
            peer_inputs.remove(0);
        }
        peer_inputs.push((tick, input));
    }

    pub fn get(&self, peer_id: &PeerId, tick: u64) -> Option<RemoteInputData> {
        self.inputs.get(peer_id).and_then(|inputs| {
            inputs
                .iter()
                .find(|(t, _)| *t == tick)
                .map(|(_, input)| input.clone())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let mut buffer = RemoteInputBuffer::default();
        let peer_id = PeerId::random();
        let input = RemoteInputData::from_bools(true, false, false, false);

        buffer.push(peer_id, 1, input.clone());
        let retrieved = buffer.get(&peer_id, 1);

        assert!(retrieved.is_some(), "Should retrieve pushed input");
        assert!(retrieved.unwrap().left, "Left should be true");
    }
}
