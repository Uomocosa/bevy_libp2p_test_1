use bevy::prelude::*;
use tracing::info;

use crate::p2p::plugin::P2PPlugin;
use crate::p2p::plugin::SwarmState;
use crate::p2p::swarm::P2PSwarm;
use crate::sync::network_state::NetworkState;

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

        app.init_resource::<NetworkState>()
            .insert_resource(SwarmState {
                swarm,
                local_peer_id,
                event_receiver,
            })
            .add_systems(FixedUpdate, crate::p2p::poll_network::poll_network_system)
            .add_systems(
                FixedUpdate,
                crate::p2p::log_peer_count::log_peer_count_system,
            )
            .add_systems(
                FixedUpdate,
                crate::sync::broadcast_input::broadcast_input_system,
            )
            .add_systems(
                FixedUpdate,
                crate::sync::apply_remote_inputs::apply_remote_inputs_system,
            );
    }
}
