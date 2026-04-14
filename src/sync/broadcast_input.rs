use bevy::prelude::*;
use tracing::debug;

use crate::game::player::{Player, PlayerInput};
use crate::p2p::get_game_topic::get_game_topic;
use crate::p2p::plugin::SwarmState;
use crate::p2p::protocol::NetworkMessage;
use crate::sync::network_state::NetworkState;
use crate::sync::tick::Tick;

pub fn broadcast_input_system(
    mut swarm_state: ResMut<SwarmState>,
    network: Res<NetworkState>,
    tick: Res<Tick>,
    local_player_query: Query<(&Player, &PlayerInput)>,
) {
    let current_tick = tick.current();

    for (player, input) in &local_player_query {
        if !player.is_local {
            continue;
        }

        if input.input.is_zero() {
            continue;
        }

        let topic = get_game_topic();
        let message = NetworkMessage::PlayerInput {
            tick: current_tick,
            input: input.input.clone(),
        };

        swarm_state.swarm.publish(topic, message);

        debug!(
            "Broadcast from {} for tick {}: input={:?}",
            network.local_peer_id, current_tick, input.input
        );
    }
}
