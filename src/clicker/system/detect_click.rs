use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use crate::clicker::component::ClickCounter;
use crate::clicker::component::Owner;
use crate::p2p::handler::P2PState;

pub fn detect_click(
    mut click_targets: Query<(&Owner, &mut ClickCounter, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    p2p_state: Res<P2PState>,
) {
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let Some(window) = windows.single().ok() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let window_size = Vec2::new(window.width(), window.height());
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
    let world_pos = Vec3::new(ndc.x * 100.0, ndc.y * 100.0, 0.0);

    for (owner, mut counter, transform) in &mut click_targets {
        let button_pos = transform.translation().truncate();
        let distance = world_pos.truncate().distance(button_pos);

        if distance < 50.0 {
            let is_self = owner.0 == p2p_state.local_peer_id;
            if is_self {
                counter.increment();
                tracing::debug!(target: "clicker", "Self-click! New count: {}", counter.0);
            } else {
                counter.decrement();
                tracing::debug!(target: "clicker", "Opponent-click! New count: {}", counter.0);
            }
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::schedule::Schedule;
    use bevy::input::mouse::MouseButton;
    use bevy::window::WindowResolution;
    use libp2p::PeerId;

    use crate::p2p::config::P2PConfig;

    #[test]
    fn test_usage() {
        let mut world = World::new();

        let local_peer = PeerId::random();
        world.insert_resource(P2PState {
            config: P2PConfig::default(),
            local_peer_id: local_peer,
            connected_peers: Vec::new(),
            discovered_peers: Vec::new(),
            pending_join_requests: Vec::new(),
        });

        let mut window = Window::default();
        window.resolution = WindowResolution::new(200, 200);
        window.set_cursor_position(Some(Vec2::new(100.0, 100.0)));
        world.spawn(window);

        world.spawn((
            Owner(local_peer),
            ClickCounter(0),
            GlobalTransform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ));

        let mut mouse_input = ButtonInput::<MouseButton>::default();
        mouse_input.press(MouseButton::Left);
        world.insert_resource(mouse_input);

        let mut schedule = Schedule::default();
        schedule.add_systems(detect_click);
        schedule.run(&mut world);

        let mut query = world.query::<&ClickCounter>();
        let counter = query.single(&world).unwrap();
        assert_eq!(counter.0, 1, "Counter should increment when clicking on own target");
    }
}