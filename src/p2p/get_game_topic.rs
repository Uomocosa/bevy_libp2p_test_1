use crate::p2p::protocol::GossipTopic;
use libp2p::gossipsub::IdentTopic;

pub fn get_game_topic() -> IdentTopic {
    let topic = GossipTopic::new();
    topic.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let topic = get_game_topic();
        let hash = topic.hash();
        let hash_str = hash.to_string();
        assert!(!hash_str.is_empty(), "Topic hash should not be empty");
    }
}
