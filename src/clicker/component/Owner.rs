use bevy::prelude::*;
use libp2p::PeerId;

use crate::p2p::handler::P2PState;

#[derive(Component, Debug, Clone)]
pub struct Owner(pub PeerId);

impl Owner {
    pub fn new(peer_id: PeerId) -> Self {
        Self(peer_id)
    }

    pub fn is_local(&self, p2p_state: &P2PState) -> bool {
        self.0 == p2p_state.local_peer_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_new() {
        let peer_id = PeerId::random();
        let owner = Owner::new(peer_id);
        assert_eq!(owner.0, peer_id);
    }
}
