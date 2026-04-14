use libp2p::PeerId;

use crate::p2p::protocol::NetworkMessage;
use crate::sync::remote_input_buffer::RemoteInputBuffer;

pub fn handle_incoming_message(
    remote_buffer: &mut RemoteInputBuffer,
    peer_id: PeerId,
    msg: NetworkMessage,
) {
    use tracing::debug;

    match msg {
        NetworkMessage::PlayerInput { tick, input } => {
            debug!("Received player input from {} for tick {}", peer_id, tick);
            remote_buffer.push(peer_id, tick, input);
        }
        NetworkMessage::PlayerJoin { peer_id: id } => {
            tracing::info!("Player joined: {}", id);
        }
        NetworkMessage::PlayerLeave { peer_id: id } => {
            tracing::info!("Player left: {}", id);
        }
        NetworkMessage::Ping => {
            debug!("Received Ping from {}", peer_id);
        }
        NetworkMessage::Pong => {
            debug!("Received Pong from {}", peer_id);
        }
    }
}
