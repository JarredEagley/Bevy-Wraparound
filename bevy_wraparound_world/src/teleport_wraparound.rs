use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::constants::WORLD_WIDTH;


pub struct TeleportWrapPlugin;
impl Plugin for TeleportWrapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, handle_wraparound)
        ;
    }
}

/// A Very, very simple teleport-based wraparound.
fn handle_wraparound(
    mut q_rigidbodies: Query<(&GlobalTransform, &mut Transform), With<RigidBody>>,
) {
    const PADDING: f32 = 5.0;

    for rigidbody in q_rigidbodies.iter_mut() {
        let (rb_global_tr, mut rb_tr) = rigidbody;
        let x_pos = rb_global_tr.translation().x;

        // Left border
        if x_pos < -PADDING {
            rb_tr.translation.x += WORLD_WIDTH;
        }
        // Right border
        else if x_pos > WORLD_WIDTH + PADDING {
            rb_tr.translation.x -= WORLD_WIDTH;
        }
    }
}
