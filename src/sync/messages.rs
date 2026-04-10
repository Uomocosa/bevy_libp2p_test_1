use serde::{Deserialize, Serialize};

use crate::p2p::protocol::{NetworkMessage, PlayerInputData};

pub type GameMessage = NetworkMessage;

pub fn create_player_input_message(tick: u64, input: PlayerInputData) -> Vec<u8> {
    let msg = NetworkMessage::PlayerInput { tick, input };
    bincode::serialize(&msg).expect("Failed to serialize message")
}

pub fn parse_message(data: &[u8]) -> Option<NetworkMessage> {
    bincode::deserialize(data).ok()
}
