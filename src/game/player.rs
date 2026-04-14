use bevy::prelude::*;
use libp2p::PeerId;

use crate::p2p::protocol::PlayerInputData;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub peer_id: PeerId,
    pub is_local: bool,
}

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Clone)]
pub struct PlayerInput {
    pub input: PlayerInputData,
}

#[derive(Component, Debug, Clone)]
pub struct InputBuffer {
    pub inputs: Vec<(u64, PlayerInputData)>,
    pub max_size: usize,
}
