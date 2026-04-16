use bevy::prelude::*;
use libp2p::PeerId;

use crate::clicker::component::ClickCounter;
use crate::clicker::component::ClickTarget;
use crate::clicker::component::Owner;
use crate::p2p::config::P2PEvent;
use crate::p2p::handler::P2PState;

pub fn handle_player_join(
    mut events: EventReader<P2PEvent>,
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
        TextBundle::from_section(
            format!("{}: 0", label),
            TextStyle {
                font_size: 32.0,
                color: if is_local {
                    Color::rgb(0.0, 1.0, 0.0)
                } else {
                    Color::rgb(1.0, 0.0, 0.0)
                },
                ..default()
            },
        ),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_click_button_command() {
        let mut commands = Commands::new_single();
        let peer_id = PeerId::random();

        spawn_click_button(&mut commands, peer_id, true);

        let entity = commands.into_iter().next();
        assert!(entity.is_some());
    }
}
