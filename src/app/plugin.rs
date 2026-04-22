use crate::p2p::config::P2PConfig;

pub struct BevyP2PPlugin {
    config: P2PConfig,
}

impl BevyP2PPlugin {
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
}
