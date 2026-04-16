use crate::boxes::component::{Position, Velocity};
use crate::p2p::protocol::PlayerInputData;

const MOVE_SPEED: f32 = 200.0;
const JUMP_VELOCITY: f32 = 350.0;
const UP_SPEED: f32 = 150.0;
const GROUND_Y: f32 = -200.0;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let mut velocity = Velocity::zero();
        let position = Position::new(0.0, -200.0);
        let input = PlayerInputData::from_bools(true, false, false, false);

        apply_input_to_velocity(&input, &mut velocity, &position);

        assert_eq!(velocity.x, -MOVE_SPEED, "Should move left");
    }
}
