use libp2p::PeerId;

#[derive(Clone, Debug)]
pub struct P2PConfig {
    pub enable_mdns: bool,
    pub enable_manual_dial: bool,
    pub heartbeat_interval_ms: u64,
    pub connection_timeout_ms: u64,
    pub auto_accept_join: bool,
    pub max_players: Option<usize>,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            enable_mdns: true,
            enable_manual_dial: true,
            heartbeat_interval_ms: 5000,
            connection_timeout_ms: 30000,
            auto_accept_join: true,
            max_players: None,
        }
    }
}

impl P2PConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mdns(mut self, enabled: bool) -> Self {
        self.enable_mdns = enabled;
        self
    }

    pub fn with_manual_dial(mut self, enabled: bool) -> Self {
        self.enable_manual_dial = enabled;
        self
    }

    pub fn with_heartbeat(mut self, ms: u64) -> Self {
        self.heartbeat_interval_ms = ms;
        self
    }

    pub fn with_connection_timeout(mut self, ms: u64) -> Self {
        self.connection_timeout_ms = ms;
        self
    }

    pub fn with_auto_accept(mut self, accept: bool) -> Self {
        self.auto_accept_join = accept;
        self
    }

    pub fn with_max_players(mut self, max: usize) -> Self {
        self.max_players = Some(max);
        self
    }

    pub fn coop() -> Self {
        Self {
            enable_mdns: true,
            enable_manual_dial: true,
            heartbeat_interval_ms: 5000,
            connection_timeout_ms: 30000,
            auto_accept_join: true,
            max_players: None,
        }
    }

    pub fn pvp() -> Self {
        Self {
            enable_mdns: true,
            enable_manual_dial: true,
            heartbeat_interval_ms: 2000,
            connection_timeout_ms: 15000,
            auto_accept_join: false,
            max_players: Some(2),
        }
    }

    pub fn mmo() -> Self {
        Self {
            enable_mdns: false,
            enable_manual_dial: true,
            heartbeat_interval_ms: 5000,
            connection_timeout_ms: 60000,
            auto_accept_join: true,
            max_players: None,
        }
    }

    pub fn lan_coop() -> Self {
        Self::coop()
    }

    pub fn lan_pvp() -> Self {
        Self::pvp()
    }
}

#[derive(Clone, Debug)]
pub enum P2PEvent {
    DiscoveredPlayer(PeerId),
    JoinRequest(PeerId),
    PlayerJoin(PeerId),
    PlayerLeave(PeerId),
    Message(PeerId, Vec<u8>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = P2PConfig::default();
        assert!(config.enable_mdns);
        assert!(config.enable_manual_dial);
        assert!(config.auto_accept_join);
    }

    #[test]
    fn test_coop_config() {
        let config = P2PConfig::coop();
        assert!(config.enable_mdns);
        assert!(config.auto_accept_join);
    }

    #[test]
    fn test_pvp_config() {
        let config = P2PConfig::pvp();
        assert!(!config.auto_accept_join);
        assert_eq!(config.max_players, Some(2));
    }

    #[test]
    fn test_builder_pattern() {
        let config = P2PConfig::new()
            .with_mdns(false)
            .with_auto_accept(false)
            .with_max_players(4);

        assert!(!config.enable_mdns);
        assert!(!config.auto_accept_join);
        assert_eq!(config.max_players, Some(4));
    }
}
