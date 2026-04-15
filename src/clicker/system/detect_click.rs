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
        let counter = ClickCounter(5);
        assert_eq!(counter.0, 5);
    }
}
