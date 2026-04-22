use bevy::prelude::*;
use tracing::info;

use crate::p2p::plugin::P2PPlugin;
use crate::p2p::plugin::SwarmState;
use crate::p2p::swarm::P2PSwarm;
use crate::sync;
use crate::sync::network_state::NetworkState;

use crate::p2p::handler::P2PState;

impl Plugin for P2PPlugin {
    fn build(&self, app: &mut App) {
        let config = self.config.clone();

        let (swarm, event_receiver) = match P2PSwarm::new(config.clone()) {
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
                config: config.clone(),
            })
            .insert_resource(P2PState::new(config, local_peer_id))
            .add_event::<crate::p2p::config::P2PEvent>()
            .add_systems(FixedUpdate, crate::p2p::poll_network::poll_network)
            .add_systems(FixedUpdate, crate::p2p::log_peer_count::log_peer_count)
            .add_systems(FixedUpdate, sync::broadcast)
            .add_systems(FixedUpdate, sync::apply_remote_inputs);
    }
}
