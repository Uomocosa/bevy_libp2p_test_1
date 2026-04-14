use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::p2p::protocol::PlayerInputData;

pub fn collect_input(button_input: &ButtonInput<KeyCode>) -> PlayerInputData {
    let left = button_input.pressed(KeyCode::ArrowLeft) || button_input.pressed(KeyCode::KeyD);
    let right = button_input.pressed(KeyCode::ArrowRight);
    let up = button_input.pressed(KeyCode::ArrowUp) || button_input.pressed(KeyCode::KeyW);
    let jump = button_input.pressed(KeyCode::Space);

    tracing::trace!(target: "player_input", left, right, up, jump);

    PlayerInputData::from_bools(left, right, up, jump)
}
