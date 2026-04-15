use bevy::prelude::*;

use crate::app::plugin::BevyP2PPlugin;
use crate::game;
use crate::p2p::plugin::P2PPlugin;
use crate::sync;
use crate::sync::network_state::NetworkState;
use crate::sync::remote_input_buffer::RemoteInputBuffer;
use crate::sync::tick::Tick;

impl Plugin for BevyP2PPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(P2PPlugin)
            .init_resource::<Tick>()
            .init_resource::<NetworkState>()
            .init_resource::<RemoteInputBuffer>()
            .add_systems(FixedUpdate, tick)
            .add_systems(FixedUpdate, game::system::input::collect)
            .add_systems(FixedUpdate, game::system::character_controller)
            .add_systems(FixedUpdate, game::system::sync_position)
            .add_systems(FixedUpdate, sync::broadcast)
            .add_systems(FixedUpdate, sync::apply_remote_inputs);
    }
}

fn tick(mut tick: ResMut<Tick>) {
    tick.next();
}
