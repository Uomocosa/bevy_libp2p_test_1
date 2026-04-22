use bevy::prelude::*;
use libp2p::PeerId;

use crate::boxes::component::Player;
use crate::boxes::component::PlayerInput;
use crate::boxes::component::Position;
use crate::boxes::component::Velocity;
use crate::p2p::config::P2PEvent;

pub fn handle_player_join(mut events: MessageReader<P2PEvent>, mut commands: Commands) {
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
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_remote_player_command() {
        let mut world = World::new();
        let peer_id = PeerId::random();

        {
            let mut commands = world.commands();
            spawn_remote_player(&mut commands, peer_id);
        }
        world.flush();

        let mut query = world.query::<&Player>();
        let player = query.single(&world).unwrap();
        assert!(player.peer_id == peer_id);
    }
}