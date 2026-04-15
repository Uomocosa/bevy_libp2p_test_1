use bevy::prelude::*;

use crate::app::plugin::BevyP2PPlugin;
use crate::p2p::plugin::P2PPlugin;
use crate::sync::network_state::NetworkState;
use crate::sync::remote_input_buffer::RemoteInputBuffer;
use crate::sync::tick::Tick;

impl Plugin for BevyP2PPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(P2PPlugin)
            .init_resource::<Tick>()
            .init_resource::<NetworkState>()
            .init_resource::<RemoteInputBuffer>()
            .add_systems(FixedUpdate, tick);
    }
}

fn tick(mut tick: ResMut<Tick>) {
    tick.next();
}
