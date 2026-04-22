use bevy::prelude::*;
use libp2p::PeerId;

use crate::clicker::component::ClickCounter;
use crate::clicker::component::ClickTarget;
use crate::clicker::component::Owner;
use crate::p2p::config::P2PEvent;
use crate::p2p::handler::P2PState;

pub fn handle_player_join(
    mut events: MessageReader<P2PEvent>,
    mut commands: Commands,
    p2p_state: Res<P2PState>,
) {
    for event in events.read() {
        if let P2PEvent::PlayerJoin(peer_id) = event {
            let is_local = *peer_id == p2p_state.local_peer_id;
            spawn_click_button(&mut commands, *peer_id, is_local);
        }
    }
}

fn spawn_click_button(commands: &mut Commands, peer_id: PeerId, is_local: bool) {
    let label = if is_local { "You" } else { "Opponent" };

    commands.spawn((
        Owner(peer_id),
        ClickCounter(0),
        ClickTarget,
        Text::new(format!("{}: 0", label)),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_click_button_command() {
        let mut world = World::new();
        let peer_id = PeerId::random();

        {
            let mut commands = world.commands();
            spawn_click_button(&mut commands, peer_id, true);
        }
        world.flush();

        let mut query = world.query::<Entity>();
        assert!(query.single(&world).is_ok());
    }
}
