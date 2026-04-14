use bevy::prelude::*;

use crate::game::collect_input::collect_input;
use crate::game::player::PlayerInput;

pub fn player_input_system(
    mut query: Query<&mut PlayerInput>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    for mut player_input in &mut query {
        let input = collect_input(&button_input);
        player_input.set(input);
    }
}
