use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use crate::clicker::component::ClickCounter;

pub fn detect_click(
    mut query: Query<&mut ClickCounter>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    for mut counter in &mut query {
        counter.increment();
        tracing::debug!(target: "clicker", "Clicked! New count: {}", counter.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        world.spawn(ClickCounter(0));

        let mut mouse_input = ButtonInput::<MouseButton>::default();
        mouse_input.press(MouseButton::Left);
        world.insert_resource(mouse_input);

        let mut schedule = Schedule::default();
        schedule.add_systems(detect_click);
        schedule.run(&mut world);

        let mut query = world.query::<&ClickCounter>();
        let counters: Vec<_> = query.iter(&world).collect();
        assert!(!counters.is_empty());
        assert_eq!(counters[0].0, 1);
    }
}
