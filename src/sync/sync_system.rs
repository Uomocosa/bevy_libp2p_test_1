use bevy::prelude::*;
use libp2p::PeerId;
use std::collections::HashMap;
use tracing::debug;

use crate::game::player::{Player, PlayerInput};
use crate::p2p::protocol::GossipTopic;
use crate::p2p::protocol::PlayerInputData as RemoteInputData;
use crate::sync::messages::create_player_input_message;
use crate::sync::tick::Tick;

#[derive(Resource)]
pub struct NetworkState {
    pub local_peer_id: PeerId,
    pub connected_peers: Vec<PeerId>,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            local_peer_id: PeerId::random(),
            connected_peers: Vec::new(),
        }
    }
}

pub fn broadcast_input_system(
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

        let topic = GossipTopic::new();
        let message = create_player_input_message(current_tick, input.input.clone());

        debug!(
            "Broadcast from {} for tick {}: {} bytes",
            network.local_peer_id,
            current_tick,
            message.len()
        );
    }
}

#[derive(Resource, Default)]
pub struct RemoteInputBuffer {
    inputs: HashMap<PeerId, Vec<(u64, RemoteInputData)>>,
    max_size: usize,
}

impl RemoteInputBuffer {
    pub fn push(&mut self, peer_id: PeerId, tick: u64, input: RemoteInputData) {
        let peer_inputs = self.inputs.entry(peer_id).or_insert_with(Vec::new);
        if peer_inputs.len() >= self.max_size {
            peer_inputs.remove(0);
        }
        peer_inputs.push((tick, input));
    }

    pub fn get(&self, peer_id: &PeerId, tick: u64) -> Option<RemoteInputData> {
        self.inputs.get(peer_id).and_then(|inputs| {
            inputs
                .iter()
                .find(|(t, _)| *t == tick)
                .map(|(_, input)| input.clone())
        })
    }
}

pub fn apply_remote_inputs_system(
    mut remote_buffer: ResMut<RemoteInputBuffer>,
    tick: Res<Tick>,
    mut players: Query<(&Player, &mut PlayerInput)>,
) {
    let current_tick = tick.current();

    for (player, mut input) in &mut players {
        if player.is_local {
            continue;
        }

        if let Some(remote_input) = remote_buffer.get(&player.peer_id, current_tick) {
            input.set(remote_input);
        }
    }
}
