use bevy::prelude::*;
use tracing;

use crate::game::player::{PlayerInput, Position, Velocity};
use crate::p2p::protocol::PlayerInputData;

const MOVE_SPEED: f32 = 200.0;
const JUMP_VELOCITY: f32 = 350.0;
const UP_SPEED: f32 = 150.0;
const GRAVITY: f32 = 800.0;
const GROUND_Y: f32 = -200.0;

pub fn physics_system(
    mut query: Query<
        (&mut Position, &mut Velocity, &PlayerInput),
        With<crate::game::player::Player>,
    >,
    time: Res<Time<Fixed>>,
) {
    let dt = time.delta_secs();

    for (mut pos, mut vel, input) in &mut query {
        if input.input.left && !input.input.right {
            vel.x = -MOVE_SPEED;
        } else if input.input.right && !input.input.left {
            vel.x = MOVE_SPEED;
        } else {
            vel.x = 0.0;
        }

        if input.input.up {
            vel.y = UP_SPEED;
        } else if input.input.jump && pos.y <= GROUND_Y + 1.0 {
            vel.y = JUMP_VELOCITY;
        } else if pos.y > GROUND_Y {
            vel.y -= GRAVITY * dt;
        } else {
            vel.y = 0.0;
            pos.y = GROUND_Y;
        }

        pos.x += vel.x * dt;
        pos.y += vel.y * dt;

        if pos.y < GROUND_Y {
            pos.y = GROUND_Y;
            vel.y = 0.0;
        }

        tracing::trace!(target: "physics", vel_x = vel.x, vel_y = vel.y, pos_x = pos.x, pos_y = pos.y);
    }
}

pub fn apply_input_to_velocity(
    input: &PlayerInputData,
    velocity: &mut Velocity,
    position: &Position,
) {
    if input.left && !input.right {
        velocity.x = -MOVE_SPEED;
    } else if input.right && !input.left {
        velocity.x = MOVE_SPEED;
    } else {
        velocity.x = 0.0;
    }

    if input.up {
        velocity.y = UP_SPEED;
    } else if input.jump && position.y <= GROUND_Y + 1.0 {
        velocity.y = JUMP_VELOCITY;
    }
}
