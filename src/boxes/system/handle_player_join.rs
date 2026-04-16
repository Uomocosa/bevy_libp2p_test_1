use bevy::prelude::*;
use libp2p::PeerId;

use crate::boxes::component::Player;
use crate::boxes::component::PlayerInput;
use crate::boxes::component::Position;
use crate::boxes::component::Velocity;
use crate::p2p::config::P2PEvent;
use crate::p2p::handler::P2PState;

pub fn handle_player_join(
    mut events: EventReader<P2PEvent>,
    mut commands: Commands,
    p2p_state: Res<P2PState>,
) {
    for event in events.read() {
        if let P2PEvent::PlayerJoin(peer_id) = event {
            spawn_remote_player(&mut commands, *peer_id);
        }
    }
}

fn spawn_remote_player(commands: &mut Commands, peer_id: PeerId) {
    commands.spawn((
        Player {
            peer_id,
            is_local: false,
        },
        Position::zero(),
        Velocity::zero(),
        PlayerInput::new(),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_remote_player_command() {
        let mut commands = Commands::new_single();
        let peer_id = PeerId::random();

        spawn_remote_player(&mut commands, peer_id);

        let entity = commands.into_iter().next();
        assert!(entity.is_some());
    }
}
