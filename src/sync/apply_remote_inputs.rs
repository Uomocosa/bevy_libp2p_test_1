use bevy::prelude::*;

use crate::boxes::component::Player;
use crate::boxes::component::PlayerInput;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let mut world = World::new();

        let remote_peer_id = libp2p::PeerId::random();
        world.spawn((
            Player {
                peer_id: remote_peer_id,
                is_local: false,
            },
            PlayerInput::new(),
        ));

        world.spawn((
            Player {
                peer_id: libp2p::PeerId::random(),
                is_local: true,
            },
            PlayerInput::new(),
        ));

        let mut remote_buffer = RemoteInputBuffer::default();
        remote_buffer.push(
            remote_peer_id,
            0,
            crate::p2p::protocol::PlayerInputData::from_bools(true, false, false, false),
        );
        world.insert_resource(remote_buffer);

        world.insert_resource(Tick::default());

        let mut schedule = Schedule::default();
        schedule.add_systems(apply_remote_inputs);
        schedule.run(&mut world);

        let mut query = world.query::<(&Player, &PlayerInput)>();
        let results: Vec<_> = query.iter(&world).collect();

        let remote_input = results
            .iter()
            .find(|(p, i)| !p.is_local && i.input.left)
            .cloned();

        assert!(
            remote_input.is_some(),
            "Remote player input should be applied"
        );
    }
}
