use crate::p2p::protocol::NetworkMessage;

pub fn parse_message(data: &[u8]) -> Option<NetworkMessage> {
    bincode::deserialize(data).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let input = crate::p2p::protocol::PlayerInputData::from_bools(true, false, false, false);
        let data = crate::sync::create_message::create_player_input_message(42, input);

        let parsed = parse_message(&data);
        assert!(parsed.is_some(), "Should parse valid message");

        if let Some(NetworkMessage::PlayerInput { tick, .. }) = parsed {
            assert_eq!(tick, 42, "Tick should match");
        }
    }
}
