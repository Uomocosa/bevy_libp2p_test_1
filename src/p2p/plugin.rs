use bevy::prelude::*;
use libp2p::gossipsub::IdentTopic;
use libp2p::PeerId;
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::p2p::protocol::NetworkMessage;
use crate::p2p::swarm::{P2PSwarm, SwarmEventType};
use crate::sync::messages::parse_message;
use crate::sync::sync_system::RemoteInputBuffer;

pub struct P2PPlugin;

#[derive(Resource)]
pub struct SwarmState {
    pub swarm: P2PSwarm,
    pub local_peer_id: PeerId,
    event_receiver: mpsc::Receiver<SwarmEventType>,
}

impl Plugin for P2PPlugin {
    fn build(&self, app: &mut App) {
        let (swarm, event_receiver) = match P2PSwarm::new() {
            Ok((s, r)) => (s, r),
            Err(e) => {
                panic!("Failed to create P2P swarm: {}", e);
            }
        };

        let local_peer_id = swarm.local_peer_id;

        info!("P2P Plugin initialized with peer ID: {}", local_peer_id);

        app.insert_resource(SwarmState {
            swarm,
            local_peer_id,
            event_receiver,
        })
        .add_systems(FixedUpdate, (poll_network_system, log_peer_count_system));
    }
}

fn poll_network_system(
    mut swarm_state: ResMut<SwarmState>,
    mut remote_buffer: ResMut<RemoteInputBuffer>,
    mut network_state: ResMut<crate::sync::sync_system::NetworkState>,
) {
    while let Some(event) = swarm_state.event_receiver.try_recv().ok() {
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
                if let Some(msg) = parse_message(&data) {
                    handle_incoming_message(&mut remote_buffer, peer_id, msg);
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

fn handle_incoming_message(
    remote_buffer: &mut RemoteInputBuffer,
    peer_id: PeerId,
    msg: NetworkMessage,
) {
    match msg {
        NetworkMessage::PlayerInput { tick, input } => {
            debug!("Received player input from {} for tick {}", peer_id, tick);
            remote_buffer.push(peer_id, tick, input);
        }
        NetworkMessage::PlayerJoin { peer_id: id } => {
            info!("Player joined: {}", id);
        }
        NetworkMessage::PlayerLeave { peer_id: id } => {
            info!("Player left: {}", id);
        }
        NetworkMessage::Ping => {
            debug!("Received Ping from {}", peer_id);
        }
        NetworkMessage::Pong => {
            debug!("Received Pong from {}", peer_id);
        }
    }
}

fn log_peer_count_system(network_state: Res<crate::sync::sync_system::NetworkState>) {
    let count = network_state.connected_peers.len();
    if count > 0 {
        debug!("Connected peers: {}", count);
    }
}

pub fn get_game_topic() -> IdentTopic {
    IdentTopic::new("bevy_p2p_game")
}
