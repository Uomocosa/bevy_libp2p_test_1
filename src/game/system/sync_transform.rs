use bevy::prelude::*;
use tracing;

use crate::game::component::Position;

pub fn sync_position_to_transform(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        tracing::trace!(target: "position_sync", x = pos.x, y = pos.y);
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}
