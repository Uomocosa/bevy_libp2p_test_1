use bevy::prelude::*;

use crate::clicker::component::Owner;
use crate::p2p::config::P2PEvent;

pub fn handle_player_leave(
    mut events: EventReader<P2PEvent>,
    mut commands: Commands,
    owner_query: Query<(Entity, &Owner)>,
) {
    for event in events.read() {
        if let P2PEvent::PlayerLeave(peer_id) = event {
            let peer_id = peer_id.clone();
            for (entity, owner) in owner_query.iter() {
                if owner.0 == peer_id {
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
        let query = world.query::<(Entity, &Owner)>();

        assert!(query.iter(&world).next().is_none());
    }
}
