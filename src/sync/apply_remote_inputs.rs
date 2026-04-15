use bevy::prelude::*;

use crate::game::component::Player;
use crate::game::component::PlayerInput;
use crate::sync::remote_input_buffer::RemoteInputBuffer;
use crate::sync::tick::Tick;

pub fn apply_remote_inputs(
    remote_buffer: ResMut<RemoteInputBuffer>,
    tick: Res<Tick>,
    mut players: Query<(&Player, &mut PlayerInput)>,
) {
    let current_tick = tick.current();

    for (player, mut input) in &mut players {
        if player.is_local {
            continue;
        }

        if let Some(remote_input) = remote_buffer.get(&player.peer_id, current_tick) {
            input.set(remote_input);
        }
    }
}
