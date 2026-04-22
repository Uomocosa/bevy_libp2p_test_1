use bevy::prelude::Resource;
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

    pub fn accept_peer(&mut self, peer_id: PeerId) -> bool {
        if let Some(max) = self.config.max_players {
            if self.connected_peers.len() >= max {
                return false;
            }
        }
        if self.pending_join_requests.contains(&peer_id) {
            self.pending_join_requests.retain(|p| p != &peer_id);
            if !self.connected_peers.contains(&peer_id) {
                self.connected_peers.push(peer_id);
            }
            return true;
        }
        false
    }

    pub fn reject_peer(&mut self, peer_id: PeerId) -> bool {
        if self.pending_join_requests.contains(&peer_id) {
            self.pending_join_requests.retain(|p| p != &peer_id);
            return true;
        }
        false
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

    #[test]
    fn test_accept_peer_with_pending_request() {
        let peer_id = PeerId::random();
        let config = P2PConfig::default().with_auto_accept(false);
        let mut state = P2PState::new(config, PeerId::random());

        state.add_join_request(peer_id);
        let result = state.accept_peer(peer_id);

        assert!(result, "accept_peer should succeed");
        assert!(state.connected_peers.contains(&peer_id), "Peer should be connected");
        assert!(
            !state.pending_join_requests.contains(&peer_id),
            "Join request should be removed"
        );
    }

    #[test]
    fn test_reject_peer_with_pending_request() {
        let peer_id = PeerId::random();
        let config = P2PConfig::default().with_auto_accept(false);
        let mut state = P2PState::new(config, PeerId::random());

        state.add_join_request(peer_id);
        let result = state.reject_peer(peer_id);

        assert!(result, "reject_peer should succeed");
        assert!(
            !state.pending_join_requests.contains(&peer_id),
            "Join request should be removed"
        );
        assert!(
            !state.connected_peers.contains(&peer_id),
            "Peer should not be connected"
        );
    }

    #[test]
    fn test_accept_peer_respects_max_players() {
        let peer_id1 = PeerId::random();
        let peer_id2 = PeerId::random();
        let config = P2PConfig::default().with_max_players(1).with_auto_accept(false);
        let mut state = P2PState::new(config, PeerId::random());

        state.add_join_request(peer_id1);
        let result1 = state.accept_peer(peer_id1);
        assert!(result1, "First peer should be accepted");

        state.add_join_request(peer_id2);
        let result2 = state.accept_peer(peer_id2);
        assert!(!result2, "Second peer should be rejected (max 1 player)");
    }
}
