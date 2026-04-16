use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::boxes::component::PlayerInput;
use crate::boxes::system::collect_input;

pub fn collect(mut query: Query<&mut PlayerInput>, button_input: Res<ButtonInput<KeyCode>>) {
    for mut player_input in &mut query {
        let input = collect_input(&button_input);
        player_input.set(input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::ButtonInput;
    use bevy::prelude::KeyCode;

    #[test]
    fn test_usage() {
        let mut world = World::new();

        world.spawn(PlayerInput::new());

        let mut button_input = ButtonInput::<KeyCode>::default();
        button_input.press(KeyCode::ArrowRight);

        let mut schedule = Schedule::default();
        schedule.add_systems(collect);

        world.insert_resource(button_input);
        schedule.run(&mut world);

        let mut query = world.query::<&PlayerInput>();
        let player_inputs: Vec<_> = query.iter(&world).collect();
        assert!(!player_inputs.is_empty());
        assert!(
            player_inputs[0].input.right,
            "Right key should be registered"
        );
    }
}
