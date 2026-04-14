use bevy::prelude::*;

use crate::app::plugin::BevyP2PPlugin;
use crate::game::physics::physics_system;
use crate::game::sync_transform::sync_position_to_transform;
use crate::game::system::player_input_system;
use crate::p2p::plugin::P2PPlugin;
use crate::sync::apply_remote_inputs::apply_remote_inputs_system;
use crate::sync::broadcast_input::broadcast_input_system;
use crate::sync::network_state::NetworkState;
use crate::sync::remote_input_buffer::RemoteInputBuffer;
use crate::sync::tick::Tick;

impl Plugin for BevyP2PPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(P2PPlugin)
            .init_resource::<Tick>()
            .init_resource::<NetworkState>()
            .init_resource::<RemoteInputBuffer>()
            .add_systems(FixedUpdate, tick_system)
            .add_systems(FixedUpdate, player_input_system)
            .add_systems(FixedUpdate, physics_system)
            .add_systems(FixedUpdate, sync_position_to_transform)
            .add_systems(FixedUpdate, broadcast_input_system)
            .add_systems(FixedUpdate, apply_remote_inputs_system);
    }
}

fn tick_system(mut tick: ResMut<Tick>) {
    tick.next();
}
