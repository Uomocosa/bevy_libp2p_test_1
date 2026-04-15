use bevy::prelude::*;
use tracing::debug;

use crate::sync::network_state::NetworkState;

pub fn log_peer_count(network_state: Res<NetworkState>) {
    let count = network_state.connected_peers.len();
    if count > 0 {
        debug!("Connected peers: {}", count);
    }
}
