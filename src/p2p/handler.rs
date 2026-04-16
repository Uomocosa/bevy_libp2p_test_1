use libp2p::PeerId;

use crate::p2p::config::P2PConfig;

#[derive(Resource)]
pub struct P2PState {
    pub config: P2PConfig,
    pub local_peer_id: PeerId,
    pub connected_peers: Vec<PeerId>,
    pub discovered_peers: Vec<PeerId>,
    pub pending_join_requests: Vec<PeerId>,
}

impl P2PState {
    pub fn new(config: P2PConfig, local_peer_id: PeerId) -> Self {
        Self {
            config,
            local_peer_id,
            connected_peers: Vec::new(),
            discovered_peers: Vec::new(),
            pending_join_requests: Vec::new(),
        }
    }

    pub fn add_discovered_peer(&mut self, peer_id: PeerId) {
        if !self.discovered_peers.contains(&peer_id) {
            self.discovered_peers.push(peer_id);
        }
    }

    pub fn add_connected_peer(&mut self, peer_id: PeerId) {
        if !self.connected_peers.contains(&peer_id) {
            self.connected_peers.push(peer_id);
        }
    }

    pub fn remove_connected_peer(&mut self, peer_id: PeerId) {
        self.connected_peers.retain(|p| p != &peer_id);
    }

    pub fn add_join_request(&mut self, peer_id: PeerId) {
        if !self.pending_join_requests.contains(&peer_id) {
            self.pending_join_requests.push(peer_id);
        }
    }

    pub fn remove_join_request(&mut self, peer_id: PeerId) {
        self.pending_join_requests.retain(|p| p != &peer_id);
    }
}

pub struct OnDiscoveredPlayer;

pub struct OnJoinRequest;

pub struct OnPlayerJoin;

pub struct OnPlayerLeave;

pub struct OnNetworkMessage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2p_state_new() {
        let peer_id = PeerId::random();
        let state = P2PState::new(P2PConfig::default(), peer_id);

        assert_eq!(state.local_peer_id, peer_id);
        assert!(state.connected_peers.is_empty());
        assert!(state.discovered_peers.is_empty());
    }

    #[test]
    fn test_add_discovered_peer() {
        let peer_id = PeerId::random();
        let mut state = P2PState::new(P2PConfig::default(), PeerId::random());

        state.add_discovered_peer(peer_id);

        assert!(state.discovered_peers.contains(&peer_id));
    }

    #[test]
    fn test_add_connected_peer() {
        let peer_id = PeerId::random();
        let mut state = P2PState::new(P2PConfig::default(), PeerId::random());

        state.add_connected_peer(peer_id);

        assert!(state.connected_peers.contains(&peer_id));
    }

    #[test]
    fn test_remove_connected_peer() {
        let peer_id = PeerId::random();
        let mut state = P2PState::new(P2PConfig::default(), PeerId::random());

        state.add_connected_peer(peer_id);
        state.remove_connected_peer(peer_id);

        assert!(!state.connected_peers.contains(&peer_id));
    }
}
