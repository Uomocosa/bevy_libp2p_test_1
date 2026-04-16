use crate::p2p::protocol::{NetworkMessage, PlayerInputData};

pub fn create_player_input_message(tick: u64, input: PlayerInputData) -> Vec<u8> {
    let msg = NetworkMessage::PlayerInput { tick, input };
    bincode::serialize(&msg).expect("Failed to serialize message")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let input = PlayerInputData::from_bools(true, false, false, false);
        let data = create_player_input_message(1, input);

        assert!(!data.is_empty(), "Serialized data should not be empty");
    }
}
