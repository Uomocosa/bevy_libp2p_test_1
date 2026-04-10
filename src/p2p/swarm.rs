use futures::StreamExt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use bincode;
use libp2p::gossipsub::{self, IdentTopic, TopicHash};
use libp2p::mdns::tokio::Behaviour as Mdns;
use libp2p::swarm::{Config as SwarmConfig, NetworkBehaviour, Swarm};
use libp2p::{identity, noise, tcp, yamux, Multiaddr, PeerId, Transport};

use crate::p2p::protocol::NetworkMessage;

pub const GAME_TOPIC_STR: &str = "bevy_p2p_game";

#[derive(NetworkBehaviour)]
#[behaviour(event_process = false)]
pub struct SwarmBehaviour {
    pub mdns: Mdns,
    pub gossipsub: gossipsub::Behaviour,
}

pub struct P2PSwarm {
    pub local_peer_id: PeerId,
    command_sender: mpsc::Sender<SwarmCommand>,
}

pub enum SwarmCommand {
    Publish(IdentTopic, NetworkMessage),
    Dial(Multiaddr),
    GetPeers(mpsc::Sender<Vec<PeerId>>),
}

impl P2PSwarm {
    pub fn new() -> Result<(Self, mpsc::Receiver<SwarmEventType>), Box<dyn std::error::Error>> {
        let (event_tx, event_rx) = mpsc::channel(100);
        let (command_sender, command_receiver) = mpsc::channel(100);

        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(&local_key.public());

        info!("Local peer ID: {}", local_peer_id);

        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .message_id_fn(|message| {
                let mut hasher = DefaultHasher::new();
                message.data.hash(&mut hasher);
                gossipsub::MessageId::from(hasher.finish().to_string())
            })
            .build()?;

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )?;

        let mdns = Mdns::new(libp2p::mdns::Config::default(), local_peer_id)?;

        let behaviour = SwarmBehaviour { mdns, gossipsub };

        let swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            SwarmConfig::without_executor(),
        );

        let topic = IdentTopic::new(GAME_TOPIC_STR);

        let swarm = Arc::new(Mutex::new(swarm));

        let swarm_clone = swarm.clone();
        swarm_clone
            .lock()
            .unwrap()
            .behaviour_mut()
            .gossipsub
            .subscribe(&topic)
            .ok();

        std::thread::spawn(move || {
            info!(
                "Swarm thread started, subscribed to topic: {}",
                GAME_TOPIC_STR
            );

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let command_receiver = Arc::new(Mutex::new(command_receiver));
            let swarm_for_stream = swarm.clone();

            loop {
                let cmd = {
                    let mut receiver = command_receiver.lock().unwrap();
                    match receiver.try_recv() {
                        Ok(c) => Some(c),
                        Err(_) => None,
                    }
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
                            if let Err(e) = swarm.dial(addr) {
                                warn!("Dial failed: {}", e);
                            }
                        }
                        SwarmCommand::GetPeers(sender) => {
                            let peers: Vec<PeerId> = swarm.connected_peers().copied().collect();
                            drop(swarm);
                            rt.block_on(async {
                                sender.send(peers).await.ok();
                            });
                        }
                    }
                }

                let event = {
                    let mut swarm_guard = swarm_for_stream.lock().unwrap();
                    match rt.block_on(futures::future::poll_fn(|cx| {
                        swarm_guard.poll_next_unpin(cx)
                    })) {
                        Some(e) => Some(e),
                        None => None,
                    }
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

                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        });

        Ok((
            Self {
                local_peer_id,
                command_sender,
            },
            event_rx,
        ))
    }

    pub fn publish(&mut self, topic: IdentTopic, message: NetworkMessage) {
        self.command_sender
            .try_send(SwarmCommand::Publish(topic, message))
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
        if let Some(peers) = rx.blocking_recv() {
            peers
        } else {
            Vec::new()
        }
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
