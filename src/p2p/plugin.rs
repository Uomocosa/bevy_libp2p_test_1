use bevy::prelude::*;
use libp2p::PeerId;
use tokio::sync::mpsc;

use crate::p2p::swarm::P2PSwarm;

pub struct P2PPlugin;

#[derive(Resource)]
pub struct SwarmState {
    pub swarm: P2PSwarm,
    pub local_peer_id: PeerId,
    pub event_receiver: mpsc::Receiver<crate::p2p::swarm::SwarmEventType>,
}
