use crate::p2p::protocol::GossipTopic;
use libp2p::gossipsub::IdentTopic;

pub fn get_game_topic() -> IdentTopic {
    let topic = GossipTopic::new();
    topic.into()
}
