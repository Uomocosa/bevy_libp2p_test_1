pub mod config;
pub mod fake_network;
pub mod get_game_topic;
pub mod handler;
pub mod handle_incoming_message;
pub mod log_peer_count;
pub mod plugin;
pub mod plugin_build;
pub mod poll_network;
pub mod protocol;
pub mod swarm;

pub use config::P2PConfig;
pub use config::P2PEvent;
pub use fake_network::{FakeNetwork, trigger_fake_player_join};
pub use handler::{OnDiscoveredPlayer, OnJoinRequest, OnNetworkMessage, OnPlayerJoin, OnPlayerLeave, P2PState};
pub use handle_incoming_message::handle_incoming_message;
pub use plugin::{P2PPlugin, SwarmState};
pub use protocol::{GossipTopic, NetworkMessage, PlayerInputData};
