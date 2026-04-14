use bevy::prelude::*;
use tracing::{debug, info};

use crate::p2p::plugin::SwarmState;
use crate::p2p::swarm::SwarmEventType;
use crate::sync::network_state::NetworkState;
use crate::sync::remote_input_buffer::RemoteInputBuffer;

pub fn poll_network_system(
    mut swarm_state: ResMut<SwarmState>,
    mut remote_buffer: ResMut<RemoteInputBuffer>,
    mut network_state: ResMut<NetworkState>,
) {
    while let Ok(event) = swarm_state.event_receiver.try_recv() {
        match event {
            SwarmEventType::PeerDiscovered(peer_id) => {
                info!("Peer discovered: {}", peer_id);
                if !network_state.connected_peers.contains(&peer_id) {
                    network_state.connected_peers.push(peer_id);
                }
            }
            SwarmEventType::PeerConnected(peer_id) => {
                debug!("Peer connected: {}", peer_id);
                if !network_state.connected_peers.contains(&peer_id) {
                    network_state.connected_peers.push(peer_id);
                }
            }
            SwarmEventType::PeerDisconnected(peer_id) => {
                debug!("Peer disconnected: {}", peer_id);
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
        if !network_state.connected_peers.contains(&peer) {
            network_state.connected_peers.push(peer);
        }
    }
}
