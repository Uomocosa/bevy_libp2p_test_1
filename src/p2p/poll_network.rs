use bevy::prelude::*;
use tracing::{debug, info, warn};

use crate::p2p::config::P2PEvent;
use crate::p2p::handler::P2PState;
use crate::p2p::plugin::SwarmState;
use crate::p2p::swarm::SwarmEventType;
use crate::sync::network_state::NetworkState;
use crate::sync::remote_input_buffer::RemoteInputBuffer;

fn can_accept_player(current_count: usize, config: &crate::p2p::config::P2PConfig) -> bool {
    if let Some(max) = config.max_players {
        return current_count < max;
    }
    true
}

pub fn poll_network(
    mut swarm_state: ResMut<SwarmState>,
    mut remote_buffer: ResMut<RemoteInputBuffer>,
    mut network_state: ResMut<NetworkState>,
    mut p2p_state: ResMut<P2PState>,
    mut events: MessageWriter<P2PEvent>,
) {
    let auto_accept = swarm_state.config.auto_accept_join;
    let max_players = swarm_state.config.max_players;
    let can_accept = can_accept_player(p2p_state.connected_peers.len(), &swarm_state.config);

    while let Ok(event) = swarm_state.event_receiver.try_recv() {
        match event {
            SwarmEventType::PeerDiscovered(peer_id) => {
                info!("Peer discovered: {}", peer_id);
                if !p2p_state.discovered_peers.contains(&peer_id) {
                    p2p_state.add_discovered_peer(peer_id);
                    events.write(P2PEvent::DiscoveredPlayer(peer_id));
                }
                if !network_state.discovered_peers.contains(&peer_id) {
                    network_state.discovered_peers.push(peer_id);
                }
            }
            SwarmEventType::PeerConnected(peer_id) => {
                debug!("Peer connected: {}", peer_id);
                if !p2p_state.connected_peers.contains(&peer_id) {
if auto_accept && can_accept {
                        p2p_state.add_connected_peer(peer_id);
                        events.write(P2PEvent::PlayerJoin(peer_id));
} else if !auto_accept {
                        p2p_state.add_join_request(peer_id);
                        events.write(P2PEvent::JoinRequest(peer_id));
                    } else {
                        warn!(
                            "Max players ({:?}) reached, rejecting connection from {}",
                            max_players, peer_id
                        );
                    }
                }
                if !network_state.connected_peers.contains(&peer_id) {
                    network_state.connected_peers.push(peer_id);
                }
            }
            SwarmEventType::PeerDisconnected(peer_id) => {
                debug!("Peer disconnected: {}", peer_id);
                if p2p_state.connected_peers.contains(&peer_id) {
                    p2p_state.remove_connected_peer(peer_id);
                    events.write(P2PEvent::PlayerLeave(peer_id));
                }
                if p2p_state.pending_join_requests.contains(&peer_id) {
                    p2p_state.remove_join_request(peer_id);
                }
                network_state.connected_peers.retain(|p| *p != peer_id);
            }
            SwarmEventType::Message(peer_id, _topic, data) => {
                if let Some(msg) = crate::sync::messages::parse_message(&data) {
                    crate::p2p::handle_incoming_message::handle_incoming_message(
                        &mut remote_buffer,
                        peer_id,
                        msg,
                    );
                }
            }
            SwarmEventType::NewListenAddr(addr) => {
                info!("Listening on {}", addr);
            }
        }
    }

    let connected_peers = swarm_state.swarm.get_connected_peers();
    for peer in connected_peers {
        if !p2p_state.connected_peers.contains(&peer) {
            if auto_accept && can_accept {
                p2p_state.add_connected_peer(peer);
                events.write(P2PEvent::PlayerJoin(peer));
            } else if !auto_accept {
                p2p_state.add_join_request(peer);
                events.write(P2PEvent::JoinRequest(peer));
            } else {
                warn!(
                    "Max players ({:?}) reached, not accepting peer {}",
                    max_players, peer
                );
            }
        }
        if !network_state.connected_peers.contains(&peer) {
            network_state.connected_peers.push(peer);
        }
    }
}
