use bevy::prelude::*;

use crate::boxes::component::Player;
use crate::p2p::config::P2PEvent;

pub fn handle_player_leave(
    mut events: MessageReader<P2PEvent>,
    mut commands: Commands,
    player_query: Query<(Entity, &Player)>,
) {
    for event in events.read() {
        if let P2PEvent::PlayerLeave(peer_id) = event {
            for (entity, player) in player_query.iter() {
                if player.peer_id == *peer_id {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_player_leave_empty() {
        let mut world = World::new();
        let mut query = world.query::<(Entity, &Player)>();

        assert!(query.iter(&world).next().is_none());
    }
}
