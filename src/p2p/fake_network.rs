use bevy::prelude::*;
use libp2p::PeerId;

use crate::p2p::config::P2PEvent;
use crate::p2p::handler::P2PState;

#[derive(Resource, Default)]
pub struct FakeNetwork {
    pub enabled: bool,
}

impl FakeNetwork {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}

pub fn simulate_discover_peer(
    mut fake_network: ResMut<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: EventWriter<P2PEvent>,
) {
    if !fake_network.enabled {
        return;
    }
}

pub fn simulate_player_join(
    mut fake_network: ResMut<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: EventWriter<P2PEvent>,
    peer_id: PeerId,
) {
    if !fake_network.enabled {
        return;
    }

    p2p_state.add_connected_peer(peer_id);
    events.send(P2PEvent::PlayerJoin(peer_id));
}

pub fn simulate_player_leave(
    mut fake_network: ResMut<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: EventWriter<P2PEvent>,
    peer_id: PeerId,
) {
    if !fake_network.enabled {
        return;
    }

    p2p_state.remove_connected_peer(peer_id);
    events.send(P2PEvent::PlayerLeave(peer_id));
}

pub fn simulate_discovered_player(
    mut fake_network: ResMut<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: EventWriter<P2PEvent>,
    peer_id: PeerId,
) {
    if !fake_network.enabled {
        return;
    }

    p2p_state.add_discovered_peer(peer_id);
    events.send(P2PEvent::DiscoveredPlayer(peer_id));
}

pub fn simulate_join_request(
    mut fake_network: ResMut<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: EventWriter<P2PEvent>,
    peer_id: PeerId,
) {
    if !fake_network.enabled {
        return;
    }

    p2p_state.add_join_request(peer_id);
    events.send(P2PEvent::JoinRequest(peer_id));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_network_default_enabled() {
        let fake = FakeNetwork::default();
        assert!(fake.enabled);
    }

    #[test]
    fn test_fake_network_disable() {
        let fake = FakeNetwork::default().disable();
        assert!(!fake.enabled);
    }
}
