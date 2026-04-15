pub mod component;
pub mod system;

use bevy::prelude::*;

pub struct ClickerGamePlugin;

impl Plugin for ClickerGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_click);
    }
}

pub fn detect_click(
    mut query: Query<&mut component::ClickCounter>,
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
