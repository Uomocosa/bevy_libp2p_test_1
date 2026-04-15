use bevy::prelude::*;

use crate::boxes::component::PlayerInput;
use crate::boxes::system::collect_input;

pub fn collect(mut query: Query<&mut PlayerInput>, button_input: Res<ButtonInput<KeyCode>>) {
    for mut player_input in &mut query {
        let input = collect_input(&button_input);
        player_input.set(input);
    }
}
