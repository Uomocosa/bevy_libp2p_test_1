pub mod component;
pub mod system;

use bevy::prelude::*;

pub struct BoxesGamePlugin;

impl Plugin for BoxesGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, system::collect)
            .add_systems(FixedUpdate, system::character_controller)
            .add_systems(FixedUpdate, system::sync_position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let _plugin = BoxesGamePlugin;
    }
}
