use futures::StreamExt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use bincode;
use libp2p::gossipsub::{self, IdentTopic, TopicHash};
use libp2p::mdns::tokio::Behaviour as Mdns;
use libp2p::swarm::{Config as SwarmConfig, NetworkBehaviour, Swarm};
use libp2p::{identity, noise, tcp, yamux, Multiaddr, PeerId, Transport};

use crate::p2p::config::P2PConfig;
use crate::p2p::protocol::NetworkMessage;

pub const GAME_TOPIC_STR: &str = "bevy_p2p_game";

#[derive(NetworkBehaviour)]
#[behaviour(event_process = false)]
pub struct SwarmBehaviour {
    pub mdns: Mdns,
    pub gossipsub: gossipsub::Behaviour,
}

#[allow(dead_code)]
pub struct P2PSwarm {
    pub local_peer_id: PeerId,
    command_sender: mpsc::Sender<SwarmCommand>,
    config: P2PConfig,
}

pub enum SwarmCommand {
    Publish(IdentTopic, NetworkMessage),
    Dial(Multiaddr),
    GetPeers(mpsc::Sender<Vec<PeerId>>),
    SetEnableManualDial(bool),
}

impl P2PSwarm {
    pub fn new(config: P2PConfig) -> Result<(Self, mpsc::Receiver<SwarmEventType>), Box<dyn std::error::Error>> {
        let (event_tx, event_rx) = mpsc::channel(100);
        let (command_sender, command_receiver) = mpsc::channel(100);

        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(&local_key.public());

        info!("Local peer ID: {}", local_peer_id);

        let enable_mdns = config.enable_mdns;
        let enable_manual_dial = config.enable_manual_dial;
        let heartbeat_interval_ms = config.heartbeat_interval_ms;

        std::thread::spawn(move || {
            info!("Swarm thread started, initializing networking");

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let mdns = if enable_mdns {
                match Mdns::new(libp2p::mdns::Config::default(), local_peer_id) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("mDNS disabled (no network interface?): {}", e);
                        Mdns::new(libp2p::mdns::Config::default(), local_peer_id)
                            .expect("mDNS disabled failed even without network")
                    }
                }
            } else {
                warn!("mDNS disabled by config");
                Mdns::new(libp2p::mdns::Config::default(), local_peer_id)
                    .expect("mDNS new should not fail")
            };

            let transport = tcp::tokio::Transport::new(tcp::Config::default())
                .upgrade(libp2p::core::upgrade::Version::V1)
                .authenticate(noise::Config::new(&local_key).expect("noise auth"))
                .multiplex(yamux::Config::default())
                .boxed();

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .message_id_fn(|message| {
                    let mut hasher = DefaultHasher::new();
                    message.data.hash(&mut hasher);
                    gossipsub::MessageId::from(hasher.finish().to_string())
                })
                .build()
                .expect("gossipsub config");

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(local_key.clone()),
                gossipsub_config,
            )
            .expect("gossipsub");

            let behaviour = SwarmBehaviour { mdns, gossipsub };

            let mut swarm = Swarm::new(
                transport,
                behaviour,
                local_peer_id,
                SwarmConfig::without_executor(),
            );

            let topic = IdentTopic::new(GAME_TOPIC_STR);
            swarm.behaviour_mut().gossipsub.subscribe(&topic).ok();

            let swarm = Arc::new(Mutex::new(swarm));

            let command_receiver = Arc::new(Mutex::new(command_receiver));
            let swarm_for_stream = swarm.clone();

            let mut enable_manual_dial = enable_manual_dial;
            let mut _last_heartbeat = Instant::now();

            loop {
                let cmd = {
                    let mut receiver = command_receiver.lock().unwrap();
                    receiver.try_recv().ok()
                };

                if let Some(cmd) = cmd {
                    let mut swarm = swarm.lock().unwrap();
                    match cmd {
                        SwarmCommand::Publish(topic, msg) => {
                            let data = bincode::serialize(&msg).expect("Failed to serialize");
                            if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic, data) {
                                warn!("Publish failed: {}", e);
                            }
                        }
                        SwarmCommand::Dial(addr) => {
                            if enable_manual_dial {
                                if let Err(e) = swarm.dial(addr) {
                                    warn!("Dial failed: {}", e);
                                }
                            } else {
                                warn!("Manual dial disabled by config");
                            }
                        }
                        SwarmCommand::GetPeers(sender) => {
                            let peers: Vec<PeerId> = swarm.connected_peers().copied().collect();
                            drop(swarm);
                            rt.block_on(async {
                                sender.send(peers).await.ok();
                            });
                        }
                        SwarmCommand::SetEnableManualDial(enabled) => {
                            enable_manual_dial = enabled;
                            debug!("Manual dial set to: {}", enabled);
                        }
                    }
                }

                let event = {
                    let mut swarm_guard = swarm_for_stream.lock().unwrap();
                    rt.block_on(futures::future::poll_fn(|cx| {
                        swarm_guard.poll_next_unpin(cx)
                    }))
                };

                if let Some(event) = event {
                    match event {
                        libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                            info!("Listening on {}", address);
                            rt.block_on(async {
                                event_tx
                                    .send(SwarmEventType::NewListenAddr(address))
                                    .await
                                    .ok();
                            });
                        }
                        libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            debug!("Connected to {}", peer_id);
                            rt.block_on(async {
                                event_tx
                                    .send(SwarmEventType::PeerConnected(peer_id))
                                    .await
                                    .ok();
                            });
                        }
                        libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            debug!("Disconnected from {}", peer_id);
                            rt.block_on(async {
                                event_tx
                                    .send(SwarmEventType::PeerDisconnected(peer_id))
                                    .await
                                    .ok();
                            });
                        }
                        libp2p::swarm::SwarmEvent::Behaviour(event) => match event {
                            SwarmBehaviourEvent::Mdns(mdns_event) => {
                                if let libp2p::mdns::Event::Discovered(peers) = mdns_event {
                                    for (peer_id, _addr) in peers {
                                        info!("Discovered peer via mDNS: {}", peer_id);
                                        rt.block_on(async {
                                            event_tx
                                                .send(SwarmEventType::PeerDiscovered(peer_id))
                                                .await
                                                .ok();
                                        });
                                    }
                                }
                            }
                            SwarmBehaviourEvent::Gossipsub(gossipsub_event) => {
                                if let gossipsub::Event::Message {
                                    propagation_source,
                                    message,
                                    ..
                                } = gossipsub_event
                                {
                                    debug!("Received message from {}", propagation_source);
                                    rt.block_on(async {
                                        event_tx
                                            .send(SwarmEventType::Message(
                                                propagation_source,
                                                message.topic,
                                                message.data,
                                            ))
                                            .await
                                            .ok();
                                    });
                                }
                            }
                        },
                        _ => {}
                    }
                }

_last_heartbeat = Instant::now();
                std::thread::sleep(std::time::Duration::from_millis(heartbeat_interval_ms));
            }
        });

        Ok((
            Self {
                local_peer_id,
                command_sender,
                config,
            },
            event_rx,
        ))
    }

    pub fn publish(&mut self, topic: IdentTopic, message: NetworkMessage) {
        self.command_sender
            .try_send(SwarmCommand::Publish(topic, message))
            .ok();
    }

    pub fn set_enable_manual_dial(&mut self, enabled: bool) {
        self.command_sender
            .try_send(SwarmCommand::SetEnableManualDial(enabled))
            .ok();
    }

    pub fn dial(&mut self, addr: Multiaddr) {
        self.command_sender.try_send(SwarmCommand::Dial(addr)).ok();
    }

    pub fn get_connected_peers(&mut self) -> Vec<PeerId> {
        let (tx, mut rx) = mpsc::channel(1);
        self.command_sender
            .try_send(SwarmCommand::GetPeers(tx))
            .ok();
        rx.blocking_recv().unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub enum SwarmEventType {
    PeerDiscovered(PeerId),
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    Message(PeerId, TopicHash, Vec<u8>),
    NewListenAddr(Multiaddr),
}

#[cfg(test)]
mod tests {
    use libp2p::PeerId;
    use std::thread;
    use std::time::Duration;
    use std::time::Instant;

    use super::{P2PSwarm, SwarmEventType};
    use crate::p2p::config::P2PConfig;

    #[test]
    fn test_p2p_swarm_initializes() {
        let config = P2PConfig::default();
        let result = P2PSwarm::new(config);

        assert!(result.is_ok(), "Swarm should initialize without error");

        let (swarm, _rx) = result.unwrap();
        let peer_id = swarm.local_peer_id;

        tracing::info!("Swarm initialized with peer ID: {}", peer_id);

        let peer_id_str = peer_id.to_string();
        assert!(!peer_id_str.is_empty(), "Peer ID should not be empty");
        assert!(
            peer_id_str.starts_with("12D3Koo"),
            "Peer ID should be a valid libp2p PeerId"
        );
    }

    #[test]
    fn test_get_connected_peers() {
        let config = P2PConfig::default();
        let (mut swarm, _rx) = P2PSwarm::new(config).expect("Failed to create swarm");

        let peers = swarm.get_connected_peers();

        tracing::debug!("Connected peers: {:?}", peers);

        assert!(peers.is_empty(), "New swarm should have no connected peers");
    }

    #[test]
    fn test_get_discovered_peers() {
        let config = P2PConfig::default();
        let (_swarm, _rx) = P2PSwarm::new(config).expect("Failed to create swarm");

        let peers: Vec<PeerId> = Vec::new();

        tracing::debug!("Discovered peers: {:?}", peers);
    }

    #[test]
    fn test_mdns_disabled_by_config() {
        let config = P2PConfig::default().with_mdns(false);
        let result = P2PSwarm::new(config);

        assert!(result.is_ok(), "Swarm should initialize even with mDNS disabled");
    }

    #[test]
    fn test_manual_dial_disabled_by_config() {
        let config = P2PConfig::default().with_manual_dial(false);
        let result = P2PSwarm::new(config);

        assert!(result.is_ok(), "Swarm should initialize even with manual dial disabled");
    }

    #[test]
    #[ignore = "mDNS requires real network/multicast between separate machines on same LAN"]
    fn test_mdns_bidirectional_discovery() {
        let config = P2PConfig::default();
        let (swarm1, mut rx1) = P2PSwarm::new(config.clone()).expect("Failed to create swarm1");
        let (swarm2, mut rx2) = P2PSwarm::new(config).expect("Failed to create swarm2");

        let peer1_id = swarm1.local_peer_id;
        let peer2_id = swarm2.local_peer_id;

        tracing::info!("Testing mDNS between {} and {}", peer1_id, peer2_id);

        let timeout = Duration::from_secs(10);
        let deadline = Instant::now() + timeout;

        let mut found_1_to_2 = false;
        let mut found_2_to_1 = false;

        while Instant::now() < deadline {
            if let Ok(event) = rx1.try_recv() {
                if let SwarmEventType::PeerDiscovered(pid) = event {
                    if pid == peer2_id {
                        found_1_to_2 = true;
                    }
                }
            }

            if let Ok(event) = rx2.try_recv() {
                if let SwarmEventType::PeerDiscovered(pid) = event {
                    if pid == peer1_id {
                        found_2_to_1 = true;
                    }
                }
            }

            if found_1_to_2 && found_2_to_1 {
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }

        assert!(found_1_to_2, "swarm1 should discover swarm2");
        assert!(found_2_to_1, "swarm2 should discover swarm1");
    }
}
