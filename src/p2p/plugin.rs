use bevy::prelude::*;
use libp2p::PeerId;
use tokio::sync::mpsc;

use crate::p2p::config::P2PConfig;
use crate::p2p::swarm::P2PSwarm;

pub struct P2PPlugin {
    config: P2PConfig,
}

impl P2PPlugin {
    pub fn new(config: P2PConfig) -> Self {
        Self { config }
    }

    pub fn coop() -> Self {
        Self {
            config: P2PConfig::coop(),
        }
    }

    pub fn pvp() -> Self {
        Self {
            config: P2PConfig::pvp(),
        }
    }

    pub fn mmo() -> Self {
        Self {
            config: P2PConfig::mmo(),
        }
    }

    pub fn lan_coop() -> Self {
        Self {
            config: P2PConfig::lan_coop(),
        }
    }

    pub fn lan_pvp() -> Self {
        Self {
            config: P2PConfig::lan_pvp(),
        }
    }

    pub fn config(&self) -> &P2PConfig {
        &self.config
    }

    pub fn with_config(mut self, config: P2PConfig) -> Self {
        self.config = config;
        self
    }
}

#[derive(Resource)]
pub struct SwarmState {
    pub swarm: P2PSwarm,
    pub local_peer_id: PeerId,
    pub event_receiver: mpsc::Receiver<crate::p2p::swarm::SwarmEventType>,
    pub config: P2PConfig,
}
