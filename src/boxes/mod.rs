pub mod component;
pub mod system;

use bevy::prelude::*;

pub struct BoxesGamePlugin;

impl Plugin for BoxesGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, collect)
            .add_systems(FixedUpdate, character_controller)
            .add_systems(FixedUpdate, sync_position);
    }
}

fn collect(
    mut query: Query<&mut crate::boxes::component::PlayerInput>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    use crate::boxes::system::collect_input::collect_input;
    for mut player_input in &mut query {
        let input = collect_input(&button_input);
        player_input.set(input);
    }
}

fn character_controller(
    mut query: Query<
        (
            &mut crate::boxes::component::Position,
            &mut crate::boxes::component::Velocity,
            &crate::boxes::component::PlayerInput,
        ),
        With<crate::boxes::component::Player>,
    >,
    time: Res<Time<Fixed>>,
) {
    const MOVE_SPEED: f32 = 200.0;
    const JUMP_VELOCITY: f32 = 350.0;
    const UP_SPEED: f32 = 150.0;
    const GRAVITY: f32 = 800.0;
    const GROUND_Y: f32 = -200.0;
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
    }
}

fn sync_position(mut query: Query<(&crate::boxes::component::Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}
