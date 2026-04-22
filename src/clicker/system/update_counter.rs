use bevy::prelude::*;

use crate::clicker::component::ClickCounter;
use crate::clicker::component::Owner;
use crate::p2p::handler::P2PState;

pub fn update_counter(p2p_state: Res<P2PState>, mut query: Query<(&Owner, &ClickCounter, &mut Text)>) {
    for (owner, counter, mut text) in &mut query {
        let label = if owner.0 == p2p_state.local_peer_id {
            "You"
        } else {
            "Opponent"
        };
        text.0 = format!("{}: {}", label, counter.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clicker::component::Owner;

    #[test]
    fn test_usage() {
        let mut world = World::new();

        let local_peer = libp2p::PeerId::random();
        let remote_peer = libp2p::PeerId::random();

        world.spawn((Owner(local_peer), ClickCounter(5), Text::new("You: 0")));
        world.spawn((
            Owner(remote_peer),
            ClickCounter(3),
            Text::new("Opponent: 0"),
        ));

        let p2p_state = P2PState::new(crate::p2p::config::P2PConfig::default(), local_peer);
        world.insert_resource(p2p_state);

        let mut schedule = Schedule::default();
        schedule.add_systems(update_counter);
        schedule.run(&mut world);

        let texts: Vec<_> = world.query::<&Text>().iter(&world).collect();
        assert_eq!(texts[0].0, "You: 5");
        assert_eq!(texts[1].0, "Opponent: 3");
    }
}
