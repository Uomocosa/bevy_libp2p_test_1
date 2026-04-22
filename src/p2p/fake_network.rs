use bevy::prelude::*;
use libp2p::PeerId;

use crate::p2p::config::P2PEvent;
use crate::p2p::handler::P2PState;

#[derive(Resource)]
pub struct FakeNetwork {
    pub enabled: bool,
}

impl Default for FakeNetwork {
    fn default() -> Self {
        Self { enabled: true }
    }
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

#[allow(unused_variables, unused_mut, clippy::needless_return)]
pub fn simulate_discover_peer(
    fake_network: ResMut<FakeNetwork>,
    p2p_state: ResMut<P2PState>,
    events: MessageWriter<P2PEvent>,
) {
    if !fake_network.enabled {
        return;
    }
}

pub fn simulate_player_join(
    fake_network: ResMut<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: MessageWriter<P2PEvent>,
    peer_id: PeerId,
) {
    if !fake_network.enabled {
        return;
    }

    p2p_state.add_connected_peer(peer_id);
    events.write(P2PEvent::PlayerJoin(peer_id));
}

pub fn simulate_player_leave(
    fake_network: ResMut<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: MessageWriter<P2PEvent>,
    peer_id: PeerId,
) {
    if !fake_network.enabled {
        return;
    }

    p2p_state.remove_connected_peer(peer_id);
    events.write(P2PEvent::PlayerLeave(peer_id));
}

pub fn simulate_discovered_player(
    fake_network: ResMut<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: MessageWriter<P2PEvent>,
    peer_id: PeerId,
) {
    if !fake_network.enabled {
        return;
    }

    p2p_state.add_discovered_peer(peer_id);
    events.write(P2PEvent::DiscoveredPlayer(peer_id));
}

pub fn simulate_join_request(
    fake_network: ResMut<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: MessageWriter<P2PEvent>,
    peer_id: PeerId,
) {
    if !fake_network.enabled {
        return;
    }

    p2p_state.add_join_request(peer_id);
    events.write(P2PEvent::JoinRequest(peer_id));
}

pub fn trigger_fake_player_join(
    fake_network: Res<FakeNetwork>,
    mut p2p_state: ResMut<P2PState>,
    mut events: MessageWriter<P2PEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !fake_network.enabled {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyP) {
        let fake_peer = PeerId::random();
        p2p_state.add_connected_peer(fake_peer);
        events.write(P2PEvent::PlayerJoin(fake_peer));
        tracing::info!("FakeNetwork: Simulated player join: {}", fake_peer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_network_default_enabled() {
        let fake = FakeNetwork::default();
        eprintln!("fake.enabled = {:?}", fake.enabled);
        assert!(fake.enabled, "fake.enabled was false");
    }

    #[test]
    fn test_fake_network_disable() {
        let fake = FakeNetwork::default().disable();
        assert!(!fake.enabled);
    }

    #[test]
    fn test_trigger_fake_player_join() {
        let fake = FakeNetwork::new();
        let p2p_state = P2PState::new(crate::p2p::P2PConfig::default(), PeerId::random());
        let mut keyboard = ButtonInput::<KeyCode>::default();

        keyboard.press(KeyCode::KeyP);

        let mut world = World::new();
        world.insert_resource(fake);
        world.insert_resource(p2p_state);
        world.insert_resource(keyboard);

        let mut schedule = Schedule::default();
        schedule.add_systems(trigger_fake_player_join);
        schedule.run(&mut world);

        let p2p_state = world.resource::<P2PState>();
        assert!(
            !p2p_state.connected_peers.is_empty(),
            "Should have connected peer after trigger"
        );
    }
}
