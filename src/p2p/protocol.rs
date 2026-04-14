use libp2p::gossipsub::IdentTopic;
use serde::{Deserialize, Serialize};

pub const GAME_TOPIC_STR: &str = "bevy_p2p_game";

#[derive(Clone, Debug)]
pub struct GossipTopic(IdentTopic);

impl GossipTopic {
    pub fn new() -> Self {
        Self(IdentTopic::new(GAME_TOPIC_STR))
    }

    pub fn hash(&self) -> libp2p::gossipsub::TopicHash {
        self.0.hash()
    }
}

impl Default for GossipTopic {
    fn default() -> Self {
        Self::new()
    }
}

impl From<GossipTopic> for IdentTopic {
    fn from(topic: GossipTopic) -> Self {
        topic.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    PlayerInput { tick: u64, input: PlayerInputData },
    PlayerJoin { peer_id: String },
    PlayerLeave { peer_id: String },
    Ping,
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerInputData {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub jump: bool,
}

impl PlayerInputData {
    pub fn from_bools(left: bool, right: bool, up: bool, jump: bool) -> Self {
        Self {
            left,
            right,
            up,
            jump,
        }
    }

    pub fn is_zero(&self) -> bool {
        !self.left && !self.right && !self.up && !self.jump
    }
}
